use crate::register::Register;
use crate::types::math::{mul_byte, mul_word};
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;

impl VM {
    pub fn mul_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.change_reg(full::MUL_REG_NUM_WORD, &dst, src, mul_byte);
    }

    pub fn mul_reg_num_word(&mut self, dst: Register, src: Word) {
        self.change_reg(full::MUL_REG_NUM_WORD, &dst, src, mul_word);
    }

    pub fn mul_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::MUL_REG_REG_BYTE, dst, src, mul_byte);
    }

    pub fn mul_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::MUL_REG_REG_WORD, dst, src, mul_word);
    }

    pub fn mul_reg_addr_byte(&mut self, dst: Register, src: Address) {
        self.change_reg_with_addr(full::MUL_REG_ADDR_BYTE, dst, src, mul_byte);
    }

    pub fn mul_reg_addr_word(&mut self, dst: Register, src: Address) {
        self.change_reg_with_addr(full::MUL_REG_ADDR_WORD, dst, src, mul_word);
    }

    pub fn mul_addr_num_byte(&mut self, dst: Address, src: Byte) {
        self.change_addr(full::MUL_ADDR_NUM_BYTE, dst, src, mul_byte);
    }

    pub fn mul_addr_num_word(&mut self, dst: Address, src: Word) {
        self.change_addr(full::MUL_ADDR_NUM_WORD, dst, src, mul_word);
    }

    pub fn mul_addr_reg_byte(&mut self, dst: Address, src: Register) {
        self.change_addr_with_reg(full::MUL_ADDR_REG_BYTE, dst, src, mul_byte);
    }

    pub fn mul_addr_reg_word(&mut self, dst: Address, src: Register) {
        self.change_addr_with_reg(full::MUL_ADDR_REG_WORD, dst, src, mul_word);
    }

    pub fn mul_addr_addr_byte(&mut self, dst: Address, src: Address) {
        self.change_addr_with_addr(full::MUL_ADDR_ADDR_BYTE, dst, src, mul_byte);
    }

    pub fn mul_addr_addr_word(&mut self, dst: Address, src: Address) {
        self.change_addr_with_addr(full::MUL_ADDR_ADDR_WORD, dst, src, mul_word);
    }
}
