use crate::internals::register_access::RegisterAccess;
use crate::register::Register;
use crate::types::bitlogic::BitLogic;
use crate::types::math::{and_byte, and_word, or_byte, or_word, xor_byte, xor_word};
use crate::types::{Byte, Word};
use crate::VM;
use maikor_language::names::full;

impl VM {
    pub fn and_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.change_reg(full::AND_REG_NUM_BYTE, &dst, src, and_byte);
    }

    pub fn and_reg_num_word(&mut self, dst: Register, src: Word) {
        self.change_reg(full::AND_REG_NUM_WORD, &dst, src, and_word);
    }

    pub fn and_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::AND_REG_REG_BYTE, dst, src, and_byte);
    }

    pub fn and_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::AND_REG_REG_WORD, dst, src, and_word);
    }

    pub fn or_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.change_reg(full::OR_REG_NUM_BYTE, &dst, src, or_byte);
    }

    pub fn or_reg_num_word(&mut self, dst: Register, src: Word) {
        self.change_reg(full::OR_REG_NUM_WORD, &dst, src, or_word);
    }

    pub fn or_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::OR_REG_REG_BYTE, dst, src, or_byte);
    }

    pub fn or_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::OR_REG_REG_WORD, dst, src, or_word);
    }

    pub fn xor_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.change_reg(full::XOR_REG_NUM_BYTE, &dst, src, xor_byte);
    }

    pub fn xor_reg_num_word(&mut self, dst: Register, src: Word) {
        self.change_reg(full::XOR_REG_NUM_WORD, &dst, src, xor_word);
    }

    pub fn xor_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::XOR_REG_REG_BYTE, dst, src, xor_byte);
    }

    pub fn xor_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.change_reg_with_reg(full::XOR_REG_REG_WORD, dst, src, xor_word);
    }

    pub fn not_reg_byte(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Byte = self.read(full::NOT_REG_BYTE, &reg);
        self.write(full::NOT_REG_BYTE, &reg, value.not());
        self.process_arg(&reg, true);
    }

    pub fn not_reg_word(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Word = self.read(full::NOT_REG_WORD, &reg);
        self.write(full::NOT_REG_WORD, &reg, value.not());
        self.process_arg(&reg, true);
    }
}
