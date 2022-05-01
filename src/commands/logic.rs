use crate::internals::register_access::WrappedRegisterAccess;
use crate::register::Register;
use crate::types::{BitLogic, Byte, Word};
use crate::VM;

impl VM {
    pub fn not_reg_byte(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Byte = self.read("NOT.B (R)", &reg);
        self.write("NOT.B (R)", &reg, value.not());
        self.process_arg(&reg, true);
    }

    pub fn not_reg_word(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Word = self.read("NOT.W (R)", &reg);
        self.write("NOT.W (R)", &reg, value.not());
        self.process_arg(&reg, true);
    }

    pub fn and_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Byte = self.read("AND.B (R,R)", &dst);
        let src_value: Byte = self.read("AND.B (R,R)", &src);
        self.write("AND.B (R,R)", &dst, dst_value.and(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn and_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Word = self.read("AND.W (R,R)", &dst);
        let src_value: Word = self.read("AND.W (R,R)", &src);
        self.write("AND.W (R,R)", &dst, dst_value.and(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn xor_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Byte = self.read("XOR.B (R,R)", &dst);
        let src_value: Byte = self.read("XOR.B (R,R)", &src);
        self.write("XOR.B (R,R)", &dst, dst_value.xor(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn xor_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Word = self.read("XOR.W (R,R)", &dst);
        let src_value: Word = self.read("XOR.W (R,R)", &src);
        self.write("XOR.W (R,R)", &dst, dst_value.xor(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn or_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Byte = self.read("OR.B (R,R)", &dst);
        let src_value: Byte = self.read("OR.B (R,R)", &src);
        self.write("OR.B (R,R)", &dst, dst_value.or(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn or_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Word = self.read("OR.W (R,R)", &dst);
        let src_value: Word = self.read("OR.W (R,R)", &src);
        self.write("OR.W (R,R)", &dst, dst_value.or(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn and_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.process_arg(&dst, false);
        let dst_value: Byte = self.read("AND.B (R,N)", &dst);
        self.write("AND.B (R,N)", &dst, dst_value.and(src));
        self.process_arg(&dst, true);
    }

    pub fn and_reg_num_word(&mut self, dst: Register, src: Word) {
        self.process_arg(&dst, false);
        let dst_value: Word = self.read("AND.W (R,N)", &dst);
        self.write("AND.W (R,N)", &dst, dst_value.and(src));
        self.process_arg(&dst, true);
    }

    pub fn or_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.process_arg(&dst, false);
        let dst_value: Byte = self.read("OR.B (R,N)", &dst);
        self.write("OR.B (R,N)", &dst, dst_value.or(src));
        self.process_arg(&dst, true);
    }

    pub fn or_reg_num_word(&mut self, dst: Register, src: Word) {
        self.process_arg(&dst, false);
        let dst_value: Word = self.read("OR.W (R,N)", &dst);
        self.write("OR.W (R,N)", &dst, dst_value.or(src));
        self.process_arg(&dst, true);
    }

    pub fn xor_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.process_arg(&dst, false);
        let dst_value: Byte = self.read("XOR.B (R,N)", &dst);
        self.write("XOR.B (R,N)", &dst, dst_value.xor(src));
        self.process_arg(&dst, true);
    }

    pub fn xor_reg_num_word(&mut self, dst: Register, src: Word) {
        self.process_arg(&dst, false);
        let dst_value: Word = self.read("XOR.W (R,N)", &dst);
        self.write("XOR.W (R,N)", &dst, dst_value.xor(src));
        self.process_arg(&dst, true);
    }
}
