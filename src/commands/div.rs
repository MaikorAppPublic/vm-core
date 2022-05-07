use crate::register::Register;
use crate::types::math::{div_byte, div_word};
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;

impl VM {
    pub fn div_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.change_reg(full::DIV_REG_NUM_BYTE, &dst, src, div_byte);
    }

    pub fn div_reg_num_word(&mut self, dst: Register, src: Word) {
        self.change_reg(full::DIV_REG_NUM_WORD, &dst, src, div_word);
    }

    pub fn div_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::DIV_REG_REG_BYTE, dst, src, div_byte);
    }

    pub fn div_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::DIV_REG_REG_WORD, dst, src, div_word);
    }

    pub fn div_reg_addr_byte(&mut self, dst: Register, src: Address) {
        self.change_reg_with_addr(full::DIV_REG_ADDR_BYTE, dst, src, div_byte);
    }

    pub fn div_reg_addr_word(&mut self, dst: Register, src: Address) {
        self.change_reg_with_addr(full::DIV_REG_ADDR_WORD, dst, src, div_word);
    }

    pub fn div_addr_num_byte(&mut self, dst: Address, src: Byte) {
        self.change_addr(full::DIV_ADDR_NUM_BYTE, dst, src, div_byte);
    }

    pub fn div_addr_num_word(&mut self, dst: Address, src: Word) {
        self.change_addr(full::DIV_ADDR_NUM_WORD, dst, src, div_word);
    }

    pub fn div_addr_reg_byte(&mut self, dst: Address, src: Register) {
        self.change_addr_with_reg(full::DIV_ADDR_REG_BYTE, dst, src, div_byte);
    }

    pub fn div_addr_reg_word(&mut self, dst: Address, src: Register) {
        self.change_addr_with_reg(full::DIV_ADDR_REG_WORD, dst, src, div_word);
    }

    pub fn div_addr_addr_byte(&mut self, dst: Address, src: Address) {
        self.change_addr_with_addr(full::DIV_ADDR_ADDR_BYTE, dst, src, div_byte);
    }

    pub fn div_addr_addr_word(&mut self, dst: Address, src: Address) {
        self.change_addr_with_addr(full::DIV_ADDR_ADDR_WORD, dst, src, div_word);
    }
}
