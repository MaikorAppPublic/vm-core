pub mod memory_access;
pub mod register_access;

use crate::constants::registers;
use crate::mem::{address, sizes};
use crate::register::Register;
use crate::VM;

impl VM {
    fn increment_register_byte(&mut self, addr: usize) {
        self.registers[addr] = self.registers[addr].wrapping_add(1);
    }

    fn decrement_register_byte(&mut self, addr: usize) {
        self.registers[addr] = self.registers[addr].wrapping_sub(1);
    }

    fn increment_register_word(&mut self, addr: usize) {
        let bytes = u16::from_be_bytes([self.registers[addr], self.registers[addr + 1]])
            .wrapping_add(1)
            .to_be_bytes();
        self.registers[addr] = bytes[0];
        self.registers[addr + 1] = bytes[1];
    }

    fn decrement_register_word(&mut self, addr: usize) {
        let bytes = u16::from_be_bytes([self.registers[addr], self.registers[addr + 1]])
            .wrapping_sub(1)
            .to_be_bytes();
        self.registers[addr] = bytes[0];
        self.registers[addr + 1] = bytes[1];
    }

    //Execute pre/post inc/dec on register
    pub fn process_arg(&mut self, reg: &Register, is_post: bool) {
        if reg.is_calc && reg.is_post == is_post {
            if reg.is_inc {
                match reg.size {
                    1 => self.increment_register_byte(reg.addr),
                    2 => self.increment_register_word(reg.addr),
                    _ => self.fail(format!("Invalid register size: {}", reg.addr)),
                }
            } else {
                match reg.size {
                    1 => self.decrement_register_byte(reg.addr),
                    2 => self.decrement_register_word(reg.addr),
                    _ => self.fail(format!("Invalid register size: {}", reg.addr)),
                }
            }
        }
    }

    pub fn required_extended_register(
        &mut self,
        op_name: &'static str,
        reason: WordRegisterErrorReason,
        register: &Register,
    ) {
        if register.size != 2 {
            self.fail(format!(
                "Invalid {op_name} param: {}, {}",
                registers::id::name(register.addr as u8),
                reason.text()
            ));
        }
    }

    pub fn get_sp(&self) -> u16 {
        let bytes = self.get_memory(address::SP, sizes::SP);
        u16::from_be_bytes([bytes[0], bytes[1]])
    }

    pub fn get_fp(&self) -> u16 {
        let bytes = self.get_memory(address::FP, sizes::FP);
        u16::from_be_bytes([bytes[0], bytes[1]])
    }
}

pub enum WordRegisterErrorReason {
    Required,
    Indirect,
}

impl WordRegisterErrorReason {
    fn text(&self) -> &'static str {
        match self {
            WordRegisterErrorReason::Required => "can only use extended registers",
            WordRegisterErrorReason::Indirect => "only extended registers can be indirect",
        }
    }
}
