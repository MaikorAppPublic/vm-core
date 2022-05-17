use crate::execute_command::ArgParams;
use crate::internals::memory_access::MemoryAccess;
use crate::mem::{address, sizes};
use crate::register::offset;
use crate::types::Byte;
use maikor_language::constants::SPRITE_COUNT;
use maikor_language::ops::get_byte_count;
use maikor_language::{registers, SAVE_COUNT};

mod commands;
mod execute_command;
mod internals;
mod mem;
mod register;
mod types;

pub struct VM {
    /// Order is AH, AL, BH, BL, CH, CL, DH, DL, FLG
    /// Extended registers (AX, BX, etc) are made of H+L, i.e.
    /// AX is \[AH,AL]
    pub registers: [u8; registers::SIZE],
    pub pc: u16,
    //All changes MUST go through debug_set_mem or debug_set_mem_range
    //otherwise banks won't change, etc
    pub memory: [u8; sizes::TOTAL],
    pub ram_banks: Vec<[u8; sizes::RAM_BANK]>,
    pub code_banks: Vec<[u8; sizes::CODE_BANK]>,
    pub save_banks: Vec<[u8; sizes::SAVE_BANK]>,
    /// if a flag is true, then the matching data in save_banks should be written to disk
    /// and the flag set to false (also, if `memory[SAVE_CONTROL]` & `AUTO_SAVE` is 0, then
    /// `memory[SAVE_CONTROL]` should set to 0)
    pub save_dirty_flag: [bool; SAVE_COUNT],
    pub atlas_banks: Vec<[u8; sizes::ATLAS]>,
    pub mem_change_affects_flags: bool,
    pub error: Option<String>,
    /// if true the VM has stopped (EoF or error) and can't continue
    pub halted: bool,
    /// Count of operations executed this session
    pub op_executed: usize,
    /// Count of cycles executed this session
    pub cycles_executed: usize,
    /// index in memory where arguments are being read from
    arg_ptr: usize,
}

impl VM {
    #[allow(clippy::new_without_default)] //not necessary
    pub fn new() -> Self {
        let mut registers = [0; registers::SIZE];
        registers[offset::FLAGS] = registers::FLG_DEFAULT;
        let mut memory = [0; sizes::TOTAL];
        //disable all sprites by default
        for i in 0..SPRITE_COUNT {
            let addr = i * sizes::SPRITE + address::SPRITE_TABLE.0 as usize;
            memory[addr + 2] = 255;
            memory[addr + 3] = 128;
        }
        let stack = (address::STACK as u16).to_be_bytes();
        memory[address::SP] = stack[0];
        memory[address::SP + 1] = stack[1];
        memory[address::FP] = stack[0];
        memory[address::FP + 1] = stack[1];
        Self {
            registers,
            pc: 0,
            memory,
            ram_banks: vec![],
            code_banks: vec![],
            save_banks: vec![],
            save_dirty_flag: [false; SAVE_COUNT],
            atlas_banks: vec![],
            mem_change_affects_flags: false,
            error: None,
            halted: false,
            op_executed: 0,
            arg_ptr: 0,
        }
    }
}

/// Public interface to VM
impl VM {
    /// Advance VM one operation
    ///
    /// If this fails (invalid register, etc) then [VM::halted] will be set to true
    ///
    /// returns number of cycles used  
    pub fn step(&mut self) -> usize {
        if self.halted {
            return;
        }
        let op_byte = self.memory[self.pc as usize];
        let param_byte_count = get_byte_count(op_byte);
        self.arg_ptr = (self.pc + 1) as usize;
        // let arg_params = ArgParams::new(&self.memory, self.pc as usize + 1, param_byte_count);
        let result = self.execute(op_byte);
        match result {
            Ok(jumped) => {
                if !jumped {
                    self.pc = self.pc.wrapping_add((1 + param_byte_count) as u16);
                }
            }
            Err(msg) => self.fail(msg),
        }
    }

    /// Run arbitrary op, does not advance PC automatically (JMP, etc ops still work)
    /// This works by writing the bytes to a section of reserved and setting the PC to there
    pub fn execute_op(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            panic!("Must have at least one byte");
        }
        // let param_byte_count = get_byte_count(bytes[0]);
        // let arg_params = ArgParams::new(&bytes, 1, param_byte_count);
        // let result = self.execute(bytes[0], arg_params);
        // if result.is_err() {
        //     self.fail(result.err().unwrap());
        // }
    }

    pub fn fail(&mut self, error_message: String) {
        self.error = Some(format!("{}\n{}", error_message, self.dump()));
        self.halted = true;
    }

    /// Writes registers to String
    pub fn dump(&self) -> String {
        format!(
            "{}\n{}\n{}",
            format_args!(
                "AH: {:02X}  AL: {:02X}  BH: {:02X}  BL: {:02X}",
                self.registers[0], self.registers[1], self.registers[2], self.registers[3]
            ),
            format_args!(
                "CH: {:02X}  CL: {:02X}  DH: {:02X}  DL: {:02X}",
                self.registers[4], self.registers[5], self.registers[6], self.registers[7]
            ),
            format_args!(
                "FLG: {:02X} SP: {:04X} FP: {:04X} PC: {:04X}",
                self.registers[8],
                self.get_sp(),
                self.get_fp(),
                self.pc
            ),
        )
    }
}

impl VM {
    /// Set one byte in memory
    /// This will trigger interrupts, bank switching, etc
    pub fn debug_set_mem(&mut self, addr: u16, value: u8) {
        self.write_mem(addr.into(), Byte::from(value));
    }
}

    /// Set bytes in memory
    /// This will trigger interrupts, bank switching, etc
    pub fn debug_set_mem_range(&mut self, addr: u16, values: &[u8]) {
        let addr = addr.into();
        for value in values {
            self.write_mem(addr, Byte::from(*value));
        }
    }
}
