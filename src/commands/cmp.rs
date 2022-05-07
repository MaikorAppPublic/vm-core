use crate::internals::flags::Flags;
use crate::internals::memory_access::MemoryAccess;
use crate::internals::register_access::RegisterAccess;
use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;
use std::fmt::Debug;

impl VM {
    fn compare_reg_reg<T: Copy + Eq + Debug>(
        &mut self,
        name: &'static str,
        lhs: Register,
        rhs: Register,
        signed: bool,
    ) where
        VM: RegisterAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&lhs, false);
        self.process_arg(&rhs, false);
        let lvalue: T = self.read(name, &lhs);
        let rvalue = self.read(name, &rhs);
        self.set_cmp_flags(lvalue, rvalue, signed);
        self.process_arg(&rhs, true);
        self.process_arg(&lhs, true);
    }

    fn compare_reg_addr<T: Copy + Eq + Debug>(
        &mut self,
        name: &'static str,
        lhs: Register,
        rhs: Address,
        signed: bool,
    ) where
        VM: RegisterAccess<T>,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&lhs, false);
        let lvalue: T = self.read(name, &lhs);
        let rvalue = self.read_mem(rhs);
        self.set_cmp_flags(lvalue, rvalue, signed);
        self.process_arg(&lhs, true);
    }

    fn compare_reg_num<T: Copy + Eq + Debug>(
        &mut self,
        name: &'static str,
        lhs: Register,
        rhs: T,
        signed: bool,
    ) where
        VM: RegisterAccess<T>,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&lhs, false);
        let lvalue: T = self.read(name, &lhs);
        self.set_cmp_flags(lvalue, rhs, signed);
        self.process_arg(&lhs, true);
    }

    pub fn cmp_reg_reg_byte(&mut self, lhs: Register, rhs: Register) {
        self.compare_reg_reg::<Byte>(full::CMP_REG_REG_BYTE, lhs, rhs, false);
    }

    pub fn cmp_reg_reg_word(&mut self, lhs: Register, rhs: Register) {
        self.compare_reg_reg::<Word>(full::CMP_REG_REG_WORD, lhs, rhs, false);
    }

    pub fn cmps_reg_reg_byte(&mut self, lhs: Register, rhs: Register) {
        self.compare_reg_reg::<Byte>(full::CMPS_REG_REG_BYTE, lhs, rhs, true);
    }

    pub fn cmps_reg_reg_word(&mut self, lhs: Register, rhs: Register) {
        self.compare_reg_reg::<Word>(full::CMPS_REG_REG_WORD, lhs, rhs, true);
    }

    pub fn cmp_reg_addr_byte(&mut self, lhs: Register, rhs: Address) {
        self.compare_reg_addr::<Byte>(full::CMP_REG_ADDR_BYTE, lhs, rhs, false);
    }

    pub fn cmp_reg_addr_word(&mut self, lhs: Register, rhs: Address) {
        self.compare_reg_addr::<Word>(full::CMP_REG_ADDR_WORD, lhs, rhs, false);
    }

    pub fn cmps_reg_addr_byte(&mut self, lhs: Register, rhs: Address) {
        self.compare_reg_addr::<Byte>(full::CMPS_REG_ADDR_BYTE, lhs, rhs, true);
    }

    pub fn cmps_reg_addr_word(&mut self, lhs: Register, rhs: Address) {
        self.compare_reg_addr::<Word>(full::CMPS_REG_ADDR_WORD, lhs, rhs, true);
    }

    pub fn cmp_reg_num_byte(&mut self, lhs: Register, rhs: Byte) {
        self.compare_reg_num::<Byte>(full::CMP_REG_NUM_BYTE, lhs, rhs, false);
    }

    pub fn cmp_reg_num_word(&mut self, lhs: Register, rhs: Word) {
        self.compare_reg_num::<Word>(full::CMP_REG_NUM_WORD, lhs, rhs, false);
    }

    pub fn cmps_reg_num_byte(&mut self, lhs: Register, rhs: Byte) {
        self.compare_reg_num::<Byte>(full::CMPS_REG_NUM_BYTE, lhs, rhs, true);
    }

    pub fn cmps_reg_num_word(&mut self, lhs: Register, rhs: Word) {
        self.compare_reg_num::<Word>(full::CMPS_REG_NUM_WORD, lhs, rhs, true);
    }
}
