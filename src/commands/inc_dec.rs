use crate::internals::flags::Flags;
use crate::internals::memory_access::MemoryAccess;
use crate::internals::register_access::RegisterAccess;
use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;
use std::intrinsics::wrapping_add;

impl VM {
    pub fn inc_reg_byte(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Byte = self.read(full::INC_REG_BYTE, &reg);
        self.write(full::INC_REG_BYTE, &reg, value.wrapping_add(Byte::ONE));
        self.process_arg(&reg, true);
        self.set_flags(incd);
    }

    pub fn inc_addr_byte(&mut self, addr: Address) {
        let value: Byte = self.read_mem(addr);
        let incd: Byte = (value.0 + 1).into();
        self.write_mem(addr, incd);
        self.set_flags(incd);
    }

    pub fn inc_reg_word(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Word = self.read(full::INC_REG_WORD, &reg);
        self.write(full::INC_REG_WORD, &reg, value.wrapping_add(Byte::ONE));
        self.process_arg(&reg, true);
        self.set_flags(incd);
    }

    pub fn inc_addr_word(&mut self, addr: Address) {
        let value: Word = self.read_mem(addr);
        self.write_mem(addr, value.wrapping_add(Byte::ONE));
        self.set_flags(incd);
    }

    pub fn dec_reg_byte(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Byte = self.read(full::DEC_REG_BYTE, &reg);
        self.write(full::DEC_REG_BYTE, &reg, value.wrapping_sub(Byte::ONE));
        self.process_arg(&reg, true);
        self.set_flags(incd);
    }

    pub fn dec_addr_byte(&mut self, addr: Address) {
        let value: Byte = self.read_mem(addr);
        self.write_mem(addr, value.wrapping_sub(Byte::ONE));
        self.set_flags(incd);
    }

    pub fn dec_reg_word(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Word = self.read(full::DEC_REG_WORD, &reg);
        self.write(full::DEC_REG_WORD, &reg, value.wrapping_sub(Byte::ONE));
        self.process_arg(&reg, true);
        self.set_flags(incd);
    }

    pub fn dec_addr_word(&mut self, addr: Address) {
        let value: Word = self.read_mem(addr);
        self.write_mem(addr, value.wrapping_sub(Byte::ONE));
        self.set_flags(incd);
    }
}
