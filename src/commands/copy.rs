use crate::internals::memory_access::MemoryAccess;
use crate::internals::register_access::WrappedRegisterAccess;
use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;

impl VM {
    pub fn cpy_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let src_value: Byte = self.read("CPY.B (R,R)", &src);
        self.write("CPY.B (R,R)", &dst, src_value);
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn cpy_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.process_arg(&dst, false);
        self.write("CPY.B (R,N)", &dst, src);
        self.process_arg(&dst, true);
    }

    pub fn cpy_reg_num_word(&mut self, dst: Register, src: Word) {
        self.process_arg(&dst, false);
        self.write("CPY.W (R,N)", &dst, src);
        self.process_arg(&dst, true);
    }

    pub fn cpy_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let src_value: Word = self.read("CPY.W (R,R)", &src);
        self.write("CPY.W (R,R)", &dst, src_value);
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn cpy_mem_reg_byte(&mut self, dst: Address, src: Register) {
        self.process_arg(&src, false);
        let dst_value: Byte = self.read_mem(dst);
        let src_value: Byte = self.read("CPY.B (A,R)", &src);
        self.write_mem(dst, dst_value.wrapping_add(src_value));
        self.process_arg(&src, true);
    }

    pub fn cpy_mem_reg_word(&mut self, dst: Address, src: Register) {
        self.process_arg(&src, false);
        let dst_value: Word = self.read_mem(dst);
        let src_value: Word = self.read("CPY.W (A,R)", &src);
        self.write_mem(dst, dst_value.wrapping_add(src_value));
        self.process_arg(&src, true);
    }

    pub fn cpy_mem_num_byte(&mut self, dst: Address, src: Byte) {
        self.write_mem(dst, src);
    }

    pub fn cpy_mem_num_word(&mut self, dst: Address, src: Word) {
        self.write_mem(dst, src);
    }

    pub fn cpy_reg_mem_byte(&mut self, dst: Register, src: Address) {
        self.cpy_reg_num_byte(dst, self.read_mem(src))
    }

    pub fn cpy_reg_mem_word(&mut self, dst: Register, src: Address) {
        self.cpy_reg_num_word(dst, self.read_mem(src))
    }

    pub fn cpy_mem_mem_byte(&mut self, dst: Address, src: Address) {
        self.cpy_mem_num_byte(dst, self.read_mem(src))
    }

    pub fn cpy_mem_mem_word(&mut self, dst: Address, src: Address) {
        self.cpy_mem_num_word(dst, self.read_mem(src))
    }
}
