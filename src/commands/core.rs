use crate::internals::register_access::WrappedRegisterAccess;
use crate::register::Register;
use crate::types::{Byte, Word};
use crate::VM;

impl VM {
    pub fn swap_reg_reg_byte(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Byte = self.read("SWAP.B (R,R)", &dst);
        let src_value: Byte = self.read("SWAP.B (R,R)", &src);
        self.write("SWAP.B (R,R)", &dst, src_value);
        self.write("SWAP.B (R,R)", &src, dst_value);
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }

    pub fn swap_reg_reg_word(&mut self, dst: Register, src: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_value: Word = self.read("SWAP.W (R,R)", &dst);
        let src_value: Word = self.read("SWAP.W (R,R)", &src);
        self.write("SWAP.W (R,R)", &dst, src_value);
        self.write("SWAP.W (R,R)", &src, dst_value);
        self.process_arg(&dst, true);
        self.process_arg(&src, true);
    }
}
