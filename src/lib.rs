use crate::mem::{address, sizes};
use crate::register::offset;
use crate::sizes::MAIN_CODE;
use crate::sound::Sound;
use maikor_platform::constants::SAVE_COUNT;
use maikor_platform::input::controller_type;
use maikor_platform::mem::address::interrupt;
use maikor_platform::mem::interrupt_flags;
use maikor_platform::registers;
use maikor_platform::registers::flags::INTERRUPTS;
use maikor_vm_file::GameFile;

mod internals;
mod mem;
mod ops;
mod register;
mod sound;

pub struct VM {
    /// Order is AH, AL, BH, BL, CH, CL, DH, DL, FLG
    /// Extended registers (AX, BX, etc) are made of H+L, i.e.
    /// AX is \[AH,AL]
    pub registers: [u8; registers::SIZE],
    /// Program Counter, address/index of byte currently being executed
    pub pc: u16,
    /// All changes MUST go through debug_set_mem or debug_set_mem_range
    /// otherwise banks won't change, etc
    pub memory: [u8; sizes::TOTAL],
    pub ram_banks: Vec<[u8; sizes::RAM_BANK]>,
    pub code_banks: Vec<[u8; sizes::CODE_BANK]>,
    pub save_banks: Vec<[u8; sizes::SAVE_BANK]>,
    pub controller_graphics_banks: Vec<[u8; sizes::CONTROLLER_GRAPHICS]>,
    /// if a flag is true, then the matching data in save_banks should be written to disk
    /// and the flag set to false (also, if `memory[SAVE_CONTROL]` & `AUTO_SAVE` is 0, then
    /// `memory[SAVE_CONTROL]` should set to 0)
    pub save_dirty_flag: [bool; SAVE_COUNT],
    pub atlas_banks: Vec<[u8; sizes::ATLAS]>,
    /// error message from EHALT
    pub error: Option<String>,
    /// if true the VM has stopped (EoF or error) and can't continue
    pub halted: bool,
    /// Count of operations executed this session
    pub op_executed: usize,
    /// Count of cycles executed this session
    pub cycles_executed: usize,
    /// index in memory where arguments are being read from
    arg_ptr: u16,
    pub sound: Sound,
}

impl VM {
    /// Create a new VM
    /// Requires an AudioPlayer, as an example see maikor-interface
    pub fn new(player: Box<dyn AudioPlayer>) -> Self {
        let mut registers = [0; registers::SIZE];
        registers[offset::FLAGS] = registers::FLG_DEFAULT;
        let mut memory = [0; sizes::TOTAL];
        let stack = (address::STACK as u16).to_be_bytes();
        memory[address::SP] = stack[0];
        memory[address::SP + 1] = stack[1];
        Self {
            registers,
            pc: 0,
            memory,
            ram_banks: vec![],
            code_banks: vec![],
            save_banks: vec![],
            controller_graphics_banks: vec![],
            save_dirty_flag: [false; SAVE_COUNT],
            atlas_banks: vec![],
            error: None,
            halted: false,
            op_executed: 0,
            cycles_executed: 0,
            arg_ptr: 0,
            sound: Sound::new(player),
        }
    }

    /// Create an instance with a dummy audio player for testing
    pub fn new_test() -> VM {
        struct TestAudio {}
        impl AudioPlayer for TestAudio {
            fn play(&mut self, _left_channel: &[f32], _right_channel: &[f32]) {}

            fn samples_rate(&self) -> u32 {
                44100
            }

            fn underflowed(&self) -> bool {
                false
            }
        }
        Self::new(Box::new(TestAudio {}))
    }
}

impl VM {
    fn fail(&mut self, error_message: String) {
        self.error = Some(format!("{}\n{}", error_message, self.dump()));
        self.halted = true;
    }
}

impl VM {
    /// Load game
    /// This only copies data to banks, it doesn't reset PC, registers, etc
    /// Call [VM::init()] once before any [VM::step()] calls
    pub fn load_game(
        &mut self,
        mut game: GameFile,
        saves: &[[u8; sizes::SAVE_BANK]],
    ) -> Result<(), String> {
        game.header.validate()?;
        unsafe {
            let dst = self.get_memory_mut(0, MAIN_CODE).as_mut_ptr();
            let src = game.main_code.as_mut_ptr();
            std::ptr::copy_nonoverlapping(src, dst, MAIN_CODE);
        }
        self.save_banks = saves.to_vec();
        for code_bank in game.code_banks {
            self.code_banks.push(code_bank);
        }
        for atlas_bank in game.atlases {
            self.atlas_banks.push(atlas_bank);
        }
        for graphics in game.controller_graphics {
            self.controller_graphics_banks.push(graphics);
        }
        for _ in 0..game.header.ram_bank_count {
            self.ram_banks.push([0; sizes::RAM_BANK]);
        }
        Ok(())
    }

    /// Loads initial banks and setup sound
    /// This should be called after `load_game()` and any needed changes are made
    /// Once this has been called the banks and memory shouldn't be changed by the host
    /// (except for setting flags, interrupts, etc)
    pub fn init(&mut self) {
        if !self.code_banks.is_empty() {
            self.init_bank(
                "CODE 1",
                address::CODE_BANK_1_ID,
                0,
                self.code_banks.len(),
                None,
            );
            self.init_bank(
                "CODE 2",
                address::CODE_BANK_2_ID,
                1,
                self.code_banks.len(),
                Some(0),
            );
        }
        if !self.ram_banks.is_empty() {
            self.init_bank(
                "RAM 1",
                address::RAM_BANK_1_ID,
                0,
                self.ram_banks.len(),
                None,
            );
            self.init_bank(
                "RAM 2",
                address::RAM_BANK_2_ID,
                1,
                self.ram_banks.len(),
                Some(0),
            );
        }
        self.init_bank(
            "ATLAS 1",
            address::ATLAS1_BANK_ID,
            0,
            self.atlas_banks.len(),
            None,
        );
        self.init_bank(
            "ATLAS 2",
            address::ATLAS2_BANK_ID,
            1,
            self.atlas_banks.len(),
            Some(0),
        );
        self.init_bank(
            "ATLAS 3",
            address::ATLAS3_BANK_ID,
            2,
            self.atlas_banks.len(),
            Some(0),
        );
        self.init_bank(
            "ATLAS 4",
            address::ATLAS4_BANK_ID,
            3,
            self.atlas_banks.len(),
            Some(0),
        );
        self.init_bank(
            "SAVE",
            address::SAVE_BANK_ID,
            0,
            self.save_banks.len(),
            None,
        );
        self.init_bank(
            "CONTROLLER",
            address::CONTROLLER_GRAPHICS,
            controller_type::UNKNOWN,
            self.controller_graphics_banks.len(),
            None,
        );
    }

    #[allow(unused_must_use)]
    fn init_bank(
        &mut self,
        bank_name: &str,
        addr: usize,
        id: u8,
        len: usize,
        backup_id: Option<u8>,
    ) {
        if ((id as usize) >= len) && backup_id.is_none() {
            panic!(
                "Attempted to load {bank_name} {id} during init, but only {} available",
                len
            );
        }
        if len > id as usize {
            self.write_byte_mem(addr as u16, id);
        } else {
            self.write_byte_mem(addr as u16, backup_id.unwrap());
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

    /// Attempt to trigger the interrupt that matches `interrupt_id`
    /// VM will halt if the id is invalid
    pub fn trigger_interrupt(&mut self, interrupt_id: u8) {
        if self.check_flag(INTERRUPTS)
            && self.memory[address::IRQ_CONTROL] & interrupt_id == interrupt_id
        {
            let addr = match interrupt_id {
                interrupt_flags::IRQ_CONTROLLER => interrupt::IRQ_CONTROLLER,
                interrupt_flags::IRQ_DATETIME => interrupt::IRQ_DATETIME,
                interrupt_flags::IRQ_INPUT => interrupt::IRQ_INPUT,
                interrupt_flags::IRQ_LINE_DRAW => interrupt::IRQ_LINE_DRAW,
                interrupt_flags::IRQ_SCREEN_DRAW => interrupt::IRQ_SCREEN_DRAW,
                interrupt_flags::IRQ_TIMER => interrupt::IRQ_TIMER,
                _ => {
                    self.fail(format!(
                        "Attempted to trigger invalid interrupt id: {interrupt_id}"
                    ));
                    return;
                }
            };
            unsafe {
                let dst = self.get_memory_mut(address::IRQ_REG_DUMP, 9).as_mut_ptr();
                let src = self.registers.as_mut_ptr();
                std::ptr::copy_nonoverlapping(src, dst, 9);
            }
            let ret_addr = self.pc.to_be_bytes();
            self.memory[address::IRQ_REG_ADDR] = ret_addr[0];
            self.memory[address::IRQ_REG_ADDR + 1] = ret_addr[1];
            self.clear_flag(INTERRUPTS);
            self.pc = addr;
        }
    }

    fn return_from_interrupt(&mut self) {
        unsafe {
            let src = self.get_memory_mut(address::IRQ_REG_DUMP, 9).as_mut_ptr();
            let dst = self.registers.as_mut_ptr();
            std::ptr::copy_nonoverlapping(src, dst, 9);
        }
        self.pc = u16::from_be_bytes([
            self.memory[address::IRQ_REG_ADDR],
            self.memory[address::IRQ_REG_ADDR + 1],
        ]);
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

#[allow(unused_must_use)]
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

pub trait AudioPlayer: Send {
    fn play(&mut self, left_channel: &[f32], right_channel: &[f32]);
    fn samples_rate(&self) -> u32;
    fn underflowed(&self) -> bool;
}
