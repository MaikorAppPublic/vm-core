use crate::VM;

impl VM {
    pub fn swap_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, offset_cost1) = self.pre_process(&dst);
        let (src_offset, offset_cost2) = self.pre_process(&src);
        if !dst.is_indirect && !src.is_indirect {
            self.registers.swap(dst.addr, src.addr);
            return self.post_process(&dst) + self.post_process(&src) + 1;
        }
        let (lhs, cost1) = self.read_byte_reg(&dst, dst_offset);
        let (rhs, cost2) = self.read_byte_reg(&src, src_offset);
        let cost3 = self.write_byte_reg(&dst, dst_offset, rhs);
        let cost4 = self.write_byte_reg(&src, src_offset, lhs);
        self.post_process(&dst)
            + self.post_process(&src)
            + cost1
            + cost2
            + cost3
            + cost4
            + offset_cost1
            + offset_cost2
    }

    pub fn swap_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, offset_cost1) = self.pre_process(&dst);
        let (src_offset, offset_cost2) = self.pre_process(&src);
        if !dst.is_indirect && !src.is_indirect {
            self.registers.swap(dst.addr, src.addr);
            self.registers.swap(dst.addr + 1, src.addr + 1);
            return self.post_process(&dst) + self.post_process(&src) + 1;
        }
        let (lhs, cost1) = self.read_word_reg(&dst, dst_offset);
        let (rhs, cost2) = self.read_word_reg(&src, src_offset);
        let cost3 = self.write_word_reg(&dst, dst_offset, rhs);
        let cost4 = self.write_word_reg(&src, src_offset, lhs);
        self.post_process(&dst)
            + self.post_process(&src)
            + cost1
            + cost2
            + cost3
            + cost4
            + offset_cost1
            + offset_cost2
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ops::test::check_cycles;
    use maikor_platform::op_params::{INDIRECT, IND_OFFSET_EXT_REG};
    use maikor_platform::registers::id;

    #[test]
    fn test_costs() {
        check_cycles(&[id::AL as u8, id::AH as u8], 1, VM::swap_byte);
        check_cycles(&[id::AX as u8 | INDIRECT, id::AH as u8], 8, VM::swap_byte);
        check_cycles(&[id::AL as u8, id::BX as u8 | INDIRECT], 8, VM::swap_byte);
        check_cycles(
            &[id::AX as u8 | INDIRECT, id::BX as u8 | INDIRECT],
            12,
            VM::swap_byte,
        );
        check_cycles(&[id::AX as u8, id::BX as u8], 1, VM::swap_word);
        check_cycles(&[id::AX as u8 | INDIRECT, id::BX as u8], 12, VM::swap_word);
        check_cycles(&[id::AX as u8, id::BX as u8 | INDIRECT], 12, VM::swap_word);
        check_cycles(
            &[id::AX as u8 | INDIRECT, id::BX as u8 | INDIRECT],
            16,
            VM::swap_word,
        );
        check_cycles(
            &[
                id::AX as u8 | IND_OFFSET_EXT_REG,
                id::BX as u8 | IND_OFFSET_EXT_REG,
                id::CX as u8,
                id::DX as u8,
            ],
            20,
            VM::swap_word,
        );
    }
}
