use crate::register::Register;
use crate::types::math::{add_byte, add_word};
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;

impl VM {
    pub fn add_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.change_reg(full::ADD_REG_NUM_BYTE, &dst, src, add_byte);
    }

    pub fn add_reg_num_word(&mut self, dst: Register, src: Word) {
        self.change_reg(full::ADD_REG_NUM_WORD, &dst, src, add_word);
    }

    pub fn add_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::ADD_REG_REG_BYTE, dst, src, add_byte);
    }

    pub fn add_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::ADD_REG_REG_WORD, dst, src, add_word);
    }

    pub fn add_reg_addr_byte(&mut self, dst: Register, src: Address) {
        self.change_reg_with_addr(full::ADD_REG_ADDR_BYTE, dst, src, add_byte);
    }

    pub fn add_reg_addr_word(&mut self, dst: Register, src: Address) {
        self.change_reg_with_addr(full::ADD_REG_ADDR_WORD, dst, src, add_word);
    }

    pub fn add_addr_num_byte(&mut self, dst: Address, src: Byte) {
        self.change_addr(full::ADD_ADDR_NUM_BYTE, dst, src, add_byte);
    }

    pub fn add_addr_num_word(&mut self, dst: Address, src: Word) {
        self.change_addr(full::ADD_ADDR_NUM_WORD, dst, src, add_word);
    }

    pub fn add_addr_reg_byte(&mut self, dst: Address, src: Register) {
        self.change_addr_with_reg(full::ADD_ADDR_REG_BYTE, dst, src, add_byte);
    }

    pub fn add_addr_reg_word(&mut self, dst: Address, src: Register) {
        self.change_addr_with_reg(full::ADD_ADDR_REG_WORD, dst, src, add_word);
    }

    pub fn add_addr_addr_byte(&mut self, dst: Address, src: Address) {
        self.change_addr_with_addr(full::ADD_ADDR_ADDR_BYTE, dst, src, add_byte);
    }

    pub fn add_addr_addr_word(&mut self, dst: Address, src: Address) {
        self.change_addr_with_addr(full::ADD_ADDR_ADDR_WORD, dst, src, add_word);
    }
}
