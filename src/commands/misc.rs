use crate::internals::register_access::RegisterAccess;
use crate::register::Register;
use crate::types::{Byte, Word};
use crate::VM;
use maikor_language::names::full;

impl VM {
    pub fn nop(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn swap_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Byte = self.read(full::SWAP_REG_REG_BYTE, &dst);
        let src_value: Byte = self.read(full::SWAP_REG_REG_BYTE, &src);
        self.write(full::SWAP_REG_REG_BYTE, &dst, src_value);
        self.write(full::SWAP_REG_REG_BYTE, &src, dst_value);
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn swap_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Word = self.read(full::SWAP_REG_REG_WORD, &dst);
        let src_value: Word = self.read(full::SWAP_REG_REG_WORD, &src);
        self.write(full::SWAP_REG_REG_WORD, &dst, src_value);
        self.write(full::SWAP_REG_REG_WORD, &src, dst_value);
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }
}
