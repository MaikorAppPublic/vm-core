use crate::internals::flags::Flags;
use crate::internals::memory_access::MemoryAccess;
use crate::internals::register_access::RegisterAccess;
use crate::register::Register;
use crate::types::math::{add_byte, add_word, sub_byte, sub_word};
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;

impl VM {
    pub fn inc_reg_byte(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Byte = self.read(full::INC_REG_BYTE, &reg);
        let (incd, _) = add_byte(value, Byte::ONE, false);
        self.write(full::INC_REG_BYTE, &reg, incd);
        self.process_arg(&reg, true);
        if !reg.is_indirect || self.mem_change_affects_flags {
            self.set_flags(incd);
        }
    }

    pub fn inc_addr_byte(&mut self, addr: Address) {
        let value: Byte = self.read_mem(addr);
        let (incd, _) = add_byte(value, Byte::ONE, false);
        self.write_mem(addr, incd);
        if self.mem_change_affects_flags {
            self.set_flags(incd);
        }
    }

    pub fn inc_reg_word(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Word = self.read(full::INC_REG_WORD, &reg);
        let (incd, _) = add_word(value, Word::ONE, false);
        self.write(full::INC_REG_WORD, &reg, incd);
        self.process_arg(&reg, true);
        if !reg.is_indirect || self.mem_change_affects_flags {
            self.set_flags(incd);
        }
    }

    pub fn inc_addr_word(&mut self, addr: Address) {
        let value: Word = self.read_mem(addr);
        let (incd, _) = add_word(value, Word::ONE, false);
        self.write_mem(addr, incd);
        if self.mem_change_affects_flags {
            self.set_flags(incd);
        }
    }

    pub fn dec_reg_byte(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Byte = self.read(full::DEC_REG_BYTE, &reg);
        let (incd, _) = sub_byte(value, Byte::ONE, false);
        self.write(full::DEC_REG_BYTE, &reg, incd);
        self.process_arg(&reg, true);
        if !reg.is_indirect || self.mem_change_affects_flags {
            self.set_flags(incd);
        }
    }

    pub fn dec_addr_byte(&mut self, addr: Address) {
        let value: Byte = self.read_mem(addr);
        let (incd, _) = sub_byte(value, Byte::ONE, false);
        self.write_mem(addr, incd);
        if self.mem_change_affects_flags {
            self.set_flags(incd);
        }
    }

    pub fn dec_reg_word(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Word = self.read(full::DEC_REG_WORD, &reg);
        let (incd, _) = sub_word(value, Word::ONE, false);
        self.write(full::DEC_REG_WORD, &reg, incd);
        self.process_arg(&reg, true);
        if !reg.is_indirect || self.mem_change_affects_flags {
            self.set_flags(incd);
        }
    }

    pub fn dec_addr_word(&mut self, addr: Address) {
        let value: Word = self.read_mem(addr);
        let (incd, _) = sub_word(value, Word::ONE, false);
        self.write_mem(addr, incd);
        if self.mem_change_affects_flags {
            self.set_flags(incd);
        }
    }
}
