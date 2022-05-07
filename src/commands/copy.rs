use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;

impl VM {
    pub fn cpy_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.set_reg(full::CPY_REG_NUM_BYTE, &dst, src);
    }

    pub fn cpy_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.set_reg_with_reg::<Byte>(full::CPY_REG_REG_BYTE, dst, src);
    }

    pub fn cpy_reg_addr_byte(&mut self, dst: Register, src: Address) {
        self.set_reg_with_addr::<Byte>(full::CPY_REG_ADDR_BYTE, dst, src);
    }

    pub fn cpy_addr_num_byte(&mut self, dst: Address, src: Byte) {
        self.set_addr(full::CPY_ADDR_NUM_BYTE, dst, src);
    }

    pub fn cpy_addr_reg_byte(&mut self, dst: Address, src: Register) {
        self.set_addr_with_reg::<Byte>(full::CPY_ADDR_REG_BYTE, dst, src);
    }

    pub fn cpy_addr_addr_byte(&mut self, dst: Address, src: Address) {
        self.set_addr_with_addr::<Byte>(full::CPY_ADDR_ADDR_BYTE, dst, src);
    }

    pub fn cpy_reg_num_word(&mut self, dst: Register, src: Word) {
        self.set_reg(full::CPY_REG_NUM_WORD, &dst, src);
    }

    pub fn cpy_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.set_reg_with_reg::<Word>(full::CPY_REG_REG_WORD, dst, src);
    }

    pub fn cpy_reg_addr_word(&mut self, dst: Register, src: Address) {
        self.set_reg_with_addr::<Word>(full::CPY_REG_ADDR_WORD, dst, src);
    }

    pub fn cpy_addr_num_word(&mut self, dst: Address, src: Word) {
        self.set_addr(full::CPY_ADDR_NUM_WORD, dst, src);
    }

    pub fn cpy_addr_reg_word(&mut self, dst: Address, src: Register) {
        self.set_addr_with_reg::<Word>(full::CPY_ADDR_REG_WORD, dst, src);
    }

    pub fn cpy_addr_addr_word(&mut self, dst: Address, src: Address) {
        self.set_addr_with_addr::<Word>(full::CPY_ADDR_ADDR_WORD, dst, src);
    }
}
