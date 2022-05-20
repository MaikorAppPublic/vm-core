use crate::mem::{address, sizes};
use crate::register::offset;
use maikor_platform::constants::{SAVE_COUNT, SPRITE_COUNT};
use maikor_platform::registers;

mod internals;
mod mem;
mod ops;
mod register;

pub struct VM {
    /// Order is AH, AL, BH, BL, CH, CL, DH, DL, FLG
    /// Extended registers (AX, BX, etc) are made of H+L, i.e.
    /// AX is \[AH,AL]
    pub registers: [u8; registers::SIZE],
    pub pc: u16,
    /// All changes MUST go through debug_set_mem or debug_set_mem_range
    /// otherwise banks won't change, etc
    pub memory: [u8; sizes::TOTAL],
    pub ram_banks: Vec<[u8; sizes::RAM_BANK]>,
    pub code_banks: Vec<[u8; sizes::CODE_BANK]>,
    pub save_banks: Vec<[u8; sizes::SAVE_BANK]>,
    /// if a flag is true, then the matching data in save_banks should be written to disk
    /// and the flag set to false (also, if `memory[SAVE_CONTROL]` & `AUTO_SAVE` is 0, then
    /// `memory[SAVE_CONTROL]` should set to 0)
    pub save_dirty_flag: [bool; SAVE_COUNT],
    pub atlas_banks: Vec<[u8; sizes::ATLAS]>,
    pub error: Option<String>,
    /// if true the VM has stopped (EoF or error) and can't continue
    pub halted: bool,
    /// Count of operations executed this session
    pub op_executed: usize,
    /// Count of cycles executed this session
    pub cycles_executed: usize,
    /// index in memory where arguments are being read from
    arg_ptr: u16,
}

impl VM {
    #[allow(clippy::new_without_default)] //not necessary
    pub fn new() -> Self {
        let mut registers = [0; registers::SIZE];
        registers[offset::FLAGS] = registers::FLG_DEFAULT;
        let mut memory = [0; sizes::TOTAL];
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
            error: None,
            halted: false,
            op_executed: 0,
            cycles_executed: 0,
            arg_ptr: 0,
        }
    }
}

impl VM {
    fn fail(&mut self, error_message: String) {
        self.error = Some(format!("{}\n{}", error_message, self.dump()));
        self.halted = true;
    }
}

impl VM {
    /// Load game and saves
    /// This only copies data to banks, it doesn't reset PC, registers, etc
    /// Call [VM::init()] once before any [VM::step()] calls
    pub fn load_game(&mut self, game: Vec<u8>, saves: &[[u8; sizes::SAVE_BANK]]) {
        for (i, save_data) in saves.iter().enumerate() {
            unsafe {
                let dst = self
                    .get_memory_mut(
                        address::SAVE_BANK + (i * sizes::SAVE_BANK),
                        sizes::SAVE_BANK,
                    )
                    .as_mut_ptr();
                std::ptr::copy_nonoverlapping(save_data.as_ptr(), dst, sizes::SAVE_BANK);
            }
        }
        todo!()
    }

    /// Loads initial banks
    /// This should be called after `load_game()` and any needed changes are made
    /// Once this has been called the banks and memory shouldn't be changed by the host
    /// (except for setting flags, interrupts, etc)
    pub fn init(&mut self) {
        self.write_byte_mem(address::RAM_BANK_ID as u16, 0);
        self.write_byte_mem(address::CODE_BANK_ID as u16, 0);
        self.write_byte_mem(address::ATLAS1_BANK_ID as u16, 0);
        if self.atlas_banks.len() > 1 {
            self.write_byte_mem(address::ATLAS2_BANK_ID as u16, 1);
        } else {
            self.write_byte_mem(address::ATLAS2_BANK_ID as u16, 0);
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
            return 0;
        }
        let op_byte = self.memory[self.pc as usize];
        self.arg_ptr = self.pc + 1;
        match self.execute(op_byte) {
            Ok((jumped, cycles)) => {
                if !jumped {
                    //arg_ptr is advanced as operands are read
                    //and should be at byte of the next op when this one has completed
                    self.pc = self.arg_ptr;
                }
                self.cycles_executed += cycles;
                self.op_executed += 1;
                return cycles;
            }
            Err(msg) => self.fail(msg),
        }
        0
    }

    /// Run arbitrary op, does not advance PC automatically (JMP, etc ops still work)
    /// This works by writing the bytes to a section of reserved and setting the PC to there
    pub fn execute_op(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            panic!("Must have at least one byte");
        }
        for (i, b) in bytes.iter().enumerate() {
            self.memory[address::RESERVED + i] = *b;
        }
        self.arg_ptr = (address::RESERVED + 1) as u16;
        let result = self.execute(bytes[0]);
        if let Err(msg) = result {
            self.fail(msg);
        }
    }

    /// Writes registers to String
    pub fn dump(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n{}",
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
            format_args!(
                "Stack: {}",
                self.memory[address::STACK..self.get_sp() as usize]
                    .iter()
                    .map(|num| format!("{:02X}", num))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            format_args!("Code: {}", {
                let start = self.pc.saturating_sub(6) as usize;
                let end = self.pc.saturating_add(7) as usize;
                self.memory[start..end]
                    .iter()
                    .map(|num| format!("{:02X}", num))
                    .collect::<Vec<String>>()
                    .join(" ")
            })
        )
    }
}

impl VM {
    /// Set one byte in memory
    /// This will trigger interrupts, bank switching, etc
    pub fn debug_set_mem(&mut self, addr: u16, value: u8) {
        self.write_byte_mem(addr, value);
    }

    /// Set bytes in memory
    /// This will trigger interrupts, bank switching, etc
    pub fn debug_set_mem_range(&mut self, addr: u16, values: &[u8]) {
        let addr = addr;
        for (i, value) in values.iter().enumerate() {
            self.write_byte_mem(addr + i as u16, *value);
        }
    }
}
