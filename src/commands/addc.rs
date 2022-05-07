use crate::register::Register;
use crate::types::math::{carrying_add_byte, carrying_add_word};
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;

impl VM {
    pub fn addc_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.change_reg(full::ADDC_REG_NUM_BYTE, &dst, src, carrying_add_byte);
    }

    pub fn addc_reg_num_word(&mut self, dst: Register, src: Word) {
        self.change_reg(full::ADDC_REG_NUM_WORD, &dst, src, carrying_add_word);
    }

    pub fn addc_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::ADDC_REG_REG_BYTE, dst, src, carrying_add_byte);
    }

    pub fn addc_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::ADDC_REG_REG_WORD, dst, src, carrying_add_word);
    }

    pub fn addc_reg_addr_byte(&mut self, dst: Register, src: Address) {
        self.change_reg_with_addr(full::ADDC_REG_ADDR_BYTE, dst, src, carrying_add_byte);
    }

    pub fn addc_reg_addr_word(&mut self, dst: Register, src: Address) {
        self.change_reg_with_addr(full::ADDC_REG_ADDR_WORD, dst, src, carrying_add_word);
    }

    pub fn addc_addr_num_byte(&mut self, dst: Address, src: Byte) {
        self.change_addr(full::ADDC_ADDR_NUM_BYTE, dst, src, carrying_add_byte);
    }

    pub fn addc_addr_num_word(&mut self, dst: Address, src: Word) {
        self.change_addr(full::ADDC_ADDR_NUM_WORD, dst, src, carrying_add_word);
    }

    pub fn addc_addr_reg_byte(&mut self, dst: Address, src: Register) {
        self.change_addr_with_reg(full::ADDC_ADDR_REG_BYTE, dst, src, carrying_add_byte);
    }

    pub fn addc_addr_reg_word(&mut self, dst: Address, src: Register) {
        self.change_addr_with_reg(full::ADDC_ADDR_REG_WORD, dst, src, carrying_add_word);
    }

    pub fn addc_addr_addr_byte(&mut self, dst: Address, src: Address) {
        self.change_addr_with_addr(full::ADDC_ADDR_ADDR_BYTE, dst, src, carrying_add_byte);
    }

    pub fn addc_addr_addr_word(&mut self, dst: Address, src: Address) {
        self.change_addr_with_addr(full::ADDC_ADDR_ADDR_WORD, dst, src, carrying_add_word);
    }
}
