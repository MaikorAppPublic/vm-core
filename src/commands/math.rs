use crate::internals::memory_access::MemoryAccess;
use crate::internals::register_access::WrappedRegisterAccess;
use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;

// Math ops (INC, DEC, ADD, SUB, MUL, DIV, MULS, DIVS)
impl VM {
    pub fn inc_reg_byte(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Byte = self.read("INC.B (R)", &reg);
        self.write("INC.B (R)", &reg, value.wrapping_add(Byte::ONE));
        self.process_arg(&reg, true);
    }

    pub fn inc_reg_word(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Word = self.read("INC.W (R)", &reg);
        self.write("INC.W (R)", &reg, value.wrapping_add(Word::ONE));
        self.process_arg(&reg, true);
    }

    pub fn inc_mem_byte(&mut self, addr: Address) {
        let value: Byte = self.read_mem(addr);
        self.write_mem(addr, value.wrapping_add(Byte::ONE));
    }

    pub fn inc_mem_word(&mut self, addr: Address) {
        let value: Word = self.read_mem(addr);
        self.write_mem(addr, value.wrapping_add(Word::ONE));
    }

    pub fn add_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Byte = self.read("ADD.B (R,R)", &dst);
        let src_value: Byte = self.read("ADD.B (R,R)", &src);
        self.write("ADD.B (R,R)", &dst, dst_value.wrapping_add(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn add_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Word = self.read("ADD.W (R,R)", &dst);
        let src_value: Word = self.read("ADD.W (R,R)", &src);
        self.write("ADD.W (R,R)", &dst, dst_value.wrapping_add(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn add_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.process_arg(&dst, false);
        let dst_value: Byte = self.read("ADD.B (R,N)", &dst);
        self.write("ADD.B (R,N)", &dst, dst_value.wrapping_add(src));
        self.process_arg(&dst, true);
    }

    pub fn add_reg_num_word(&mut self, dst: Register, src: Word) {
        self.process_arg(&dst, false);
        let dst_value: Word = self.read("ADD.W (R,N)", &dst);
        self.write("ADD.W (R,N)", &dst, dst_value.wrapping_add(src));
        self.process_arg(&dst, true);
    }

    pub fn add_mem_reg_byte(&mut self, dst: Address, src: Register) {
        self.process_arg(&src, false);
        let dst_value: Byte = self.read_mem(dst);
        let src_value: Byte = self.read("ADD.B (A,R)", &src);
        self.write_mem(dst, dst_value.wrapping_add(src_value));
        self.process_arg(&src, true);
    }

    pub fn add_mem_reg_word(&mut self, dst: Address, src: Register) {
        self.process_arg(&src, false);
        let dst_value: Word = self.read_mem(dst);
        let src_value: Word = self.read("ADD.W (A,R)", &src);
        self.write_mem(dst, dst_value.wrapping_add(src_value));
        self.process_arg(&src, true);
    }

    pub fn add_mem_num_byte(&mut self, dst: Address, src: Byte) {
        let dst_value: Byte = self.read_mem(dst);
        self.write_mem(dst, dst_value.wrapping_add(src));
    }

    pub fn add_mem_num_word(&mut self, dst: Address, src: Word) {
        let dst_value: Word = self.read_mem(dst);
        self.write_mem(dst, dst_value.wrapping_add(src));
    }

    pub fn add_reg_mem_byte(&mut self, dst: Register, src: Address) {
        self.add_reg_num_byte(dst, self.read_mem(src))
    }

    pub fn add_reg_mem_word(&mut self, dst: Register, src: Address) {
        self.add_reg_num_word(dst, self.read_mem(src))
    }

    pub fn add_mem_mem_byte(&mut self, dst: Address, src: Address) {
        self.add_mem_num_byte(dst, self.read_mem(src))
    }

    pub fn add_mem_mem_word(&mut self, dst: Address, src: Address) {
        self.add_mem_num_word(dst, self.read_mem(src))
    }

    pub fn dec_reg_byte(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Byte = self.read("DEC.B (R)", &reg);
        self.write("DEC.B (R)", &reg, value.wrapping_sub(Byte::ONE));
        self.process_arg(&reg, true);
    }

    pub fn dec_reg_word(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        let value: Word = self.read("DEC.W (R)", &reg);
        self.write("DEC.W (R)", &reg, value.wrapping_sub(Word::ONE));
        self.process_arg(&reg, true);
    }

    pub fn dec_mem_byte(&mut self, addr: Address) {
        let value: Byte = self.read_mem(addr);
        self.write_mem(addr, value.wrapping_sub(Byte::ONE));
    }

    pub fn dec_mem_word(&mut self, addr: Address) {
        let value: Word = self.read_mem(addr);
        self.write_mem(addr, value.wrapping_sub(Word::ONE));
    }

    pub fn sub_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Byte = self.read("SUB.B (R,R)", &dst);
        let src_value: Byte = self.read("SUB.B (R,R)", &src);
        self.write("SUB.B (R,R)", &dst, dst_value.wrapping_sub(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn sub_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Word = self.read("SUB.W (R,R)", &dst);
        let src_value: Word = self.read("SUB.W (R,R)", &src);
        self.write("SUB.W (R,R)", &dst, dst_value.wrapping_sub(src_value));
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn sub_reg_num_byte(&mut self, dst: Register, src: Byte) {
        self.process_arg(&dst, false);
        let dst_value: Byte = self.read("SUB.B (R,N)", &dst);
        self.write("SUB.B (R,N)", &dst, dst_value.wrapping_sub(src));
        self.process_arg(&dst, true);
    }

    pub fn sub_reg_num_word(&mut self, dst: Register, src: Word) {
        self.process_arg(&dst, false);
        let dst_value: Word = self.read("SUB.W (R,N)", &dst);
        self.write("SUB.W (R,N)", &dst, dst_value.wrapping_sub(src));
        self.process_arg(&dst, true);
    }

    pub fn sub_mem_reg_byte(&mut self, dst: Address, src: Register) {
        self.process_arg(&src, false);
        let dst_value: Byte = self.read_mem(dst);
        let src_value: Byte = self.read("SUB.B (A,R)", &src);
        self.write_mem(dst, dst_value.wrapping_sub(src_value));
        self.process_arg(&src, true);
    }

    pub fn sub_mem_reg_word(&mut self, dst: Address, src: Register) {
        self.process_arg(&src, false);
        let dst_value: Word = self.read_mem(dst);
        let src_value: Word = self.read("SUB.W (A,R)", &src);
        self.write_mem(dst, dst_value.wrapping_sub(src_value));
        self.process_arg(&src, true);
    }

    pub fn sub_mem_num_byte(&mut self, dst: Address, src: Byte) {
        let dst_value: Byte = self.read_mem(dst);
        self.write_mem(dst, dst_value.wrapping_sub(src));
    }

    pub fn sub_mem_num_word(&mut self, dst: Address, src: Word) {
        let dst_value: Word = self.read_mem(dst);
        self.write_mem(dst, dst_value.wrapping_sub(src));
    }

    pub fn sub_reg_mem_byte(&mut self, dst: Register, src: Address) {
        self.sub_reg_num_byte(dst, self.read_mem(src))
    }

    pub fn sub_reg_mem_word(&mut self, dst: Register, src: Address) {
        self.sub_reg_num_word(dst, self.read_mem(src))
    }

    pub fn sub_mem_mem_byte(&mut self, dst: Address, src: Address) {
        self.sub_mem_num_byte(dst, self.read_mem(src))
    }

    pub fn sub_mem_mem_word(&mut self, dst: Address, src: Address) {
        self.sub_mem_num_word(dst, self.read_mem(src))
    }
}
