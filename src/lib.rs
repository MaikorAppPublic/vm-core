use crate::internals::memory_access::MemoryAccess;
use crate::mem::{address, sizes};
use crate::register::offset;
use crate::types::Byte;
use maikor_language::constants::SPRITE_COUNT;
use maikor_language::ops::get_byte_count;
use maikor_language::{registers, SAVE_COUNT};
use std::collections::VecDeque;

mod commands;
mod execute_command;
mod internals;
mod mem;
mod register;
mod types;

pub struct VM {
    pub registers: [u8; registers::SIZE],
    pub pc: u16,
    //All changes MUST go through debug_set_mem or debug_set_mem_range
    //otherwise banks won't change, etc
    pub memory: [u8; sizes::TOTAL],
    pub ram_banks: Vec<[u8; sizes::RAM_BANK]>,
    pub code_banks: Vec<[u8; sizes::CODE_BANK]>,
    pub save_banks: Vec<[u8; sizes::SAVE_BANK]>,
    pub save_dirty_flag: [bool; SAVE_COUNT],
    pub atlas_banks: Vec<[u8; sizes::ATLAS]>,
    pub mem_change_affects_flags: bool,
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
        }
    }
}

/// Public interface to VM
impl VM {
    pub fn step(&mut self) {
        let op_byte = self.memory[self.pc as usize];
        let param_byte_count = get_byte_count(op_byte);
        if param_byte_count > 0 {
            let start = self.pc as usize + 1;
            let params = self.memory[start..=start + param_byte_count].to_owned();
            self.execute(op_byte, VecDeque::from(params));
        } else {
            self.execute(op_byte, VecDeque::new());
        }
        self.pc = self.pc.wrapping_add((1 + param_byte_count) as u16);
    }

    //Run arbitrary bits, does not advance PC
    pub fn execute_op(&mut self, bytes: &[u8]) {
        if bytes.is_empty() {
            panic!("Must have at least one byte");
        }
        if bytes.len() == 1 {
            self.execute(bytes[0], VecDeque::new());
        } else {
            self.execute(bytes[0], VecDeque::from(bytes[1..].to_owned()));
        }
    }

    pub fn fail(&mut self, error_message: String) {
        panic!("{}\n{}", error_message, self.dump())
    }

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
    pub fn debug_set_mem(&mut self, addr: u16, value: u8) {
        self.write_mem(addr.into(), Byte::from(value));
    }
}

impl VM {
    pub fn debug_set_mem_range(&mut self, addr: u16, values: &[u8]) {
        let addr = addr.into();
        for value in values {
            self.write_mem(addr, Byte::from(*value));
        }
    }
}
