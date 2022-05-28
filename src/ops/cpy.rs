use crate::VM;

impl VM {
    pub fn cpy_reg_num_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_byte();
        let (offset, offset_cost) = self.pre_process(&dst, 1);
        let write_cost = self.write_byte_reg(&dst, offset, src);
        offset_cost + write_cost + self.post_process(&dst, 1)
    }

    pub fn cpy_reg_reg_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst, 1);
        let (src_offset, src_offset_cost) = self.pre_process(&src, 1);
        let (src_value, read_cost) = self.read_byte_reg(&src, src_offset);
        let write_cost = self.write_byte_reg(&dst, dst_offset, src_value);
        dst_offset_cost + write_cost + src_offset_cost + read_cost + self.post_process(&dst, 1)
    }

    pub fn cpy_reg_addr_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let (offset, offset_cost) = self.pre_process(&dst, 1);
        let (src_value, read_cost) = self.read_byte_mem(src);
        let write_cost = self.write_byte_reg(&dst, offset, src_value);
        offset_cost + write_cost + read_cost + self.post_process(&dst, 1)
    }

    pub fn cpy_reg_num_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let (offset, offset_cost) = self.pre_process(&dst, 2);
        let write_cost = self.write_word_reg(&dst, offset, src);
        offset_cost + write_cost + self.post_process(&dst, 2)
    }

    pub fn cpy_reg_reg_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst, 2);
        let (src_offset, src_offset_cost) = self.pre_process(&src, 2);
        let (src_value, read_cost) = self.read_word_reg(&src, src_offset);
        let write_cost = self.write_word_reg(&dst, dst_offset, src_value);
        dst_offset_cost + write_cost + src_offset_cost + read_cost + self.post_process(&dst, 2)
    }

    pub fn cpy_reg_addr_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let (offset, offset_cost) = self.pre_process(&dst, 2);
        let (src_value, read_cost) = self.read_word_mem(src);
        let write_cost = self.write_word_reg(&dst, offset, src_value);
        offset_cost + write_cost + read_cost + self.post_process(&dst, 2)
    }

    pub fn cpy_addr_num_byte(&mut self) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_byte();
        self.write_byte_mem(dst, src)
    }

    pub fn cpy_addr_addr_byte(&mut self) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_word();
        let (src_value, read_cost) = self.read_byte_mem(src);
        let write_cost = self.write_byte_mem(dst, src_value);
        write_cost + read_cost
    }

    pub fn cpy_addr_reg_byte(&mut self) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 1);
        let (src_value, read_cost) = self.read_byte_reg(&src, offset);
        let write_cost = self.write_byte_mem(dst, src_value);
        write_cost + read_cost + offset_cost + self.post_process(&src, 1)
    }

    pub fn cpy_addr_num_word(&mut self) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_word();
        self.write_word_mem(dst, src)
    }

    pub fn cpy_addr_addr_word(&mut self) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_word();
        let (src_value, read_cost) = self.read_word_mem(src);
        let write_cost = self.write_word_mem(dst, src_value);
        write_cost + read_cost
    }

    pub fn cpy_addr_reg_word(&mut self) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 2);
        let (src_value, read_cost) = self.read_word_reg(&src, offset);
        let write_cost = self.write_word_mem(dst, src_value);
        write_cost + offset_cost + read_cost + self.post_process(&src, 2)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ops::test::check_cycles;
    use maikor_platform::op_params::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_costs() {
        check_cycles(&[id::AL as u8, 10], 1, VM::cpy_reg_num_byte);
        check_cycles(&[id::AL as u8, id::AH as u8], 2, VM::cpy_reg_reg_byte);
        check_cycles(&[id::AL as u8, 0, 0], 2, VM::cpy_reg_addr_byte);
        check_cycles(&[0, 0, 10], 1, VM::cpy_addr_num_byte);
        check_cycles(&[3, 4, id::CH as u8], 2, VM::cpy_addr_reg_byte);
        check_cycles(&[1, 2, 3, 4], 2, VM::cpy_addr_addr_byte);
        check_cycles(&[id::AX as u8, 10], 2, VM::cpy_reg_num_word);
        check_cycles(&[id::AL as u8, id::BX as u8], 4, VM::cpy_reg_reg_word);
        check_cycles(&[id::AX as u8, 10, 1], 4, VM::cpy_reg_addr_word);
        check_cycles(&[9, 9, 10], 2, VM::cpy_addr_num_word);
        check_cycles(&[1, 2, id::CX as u8], 4, VM::cpy_addr_reg_word);
        check_cycles(&[6, 7, 8, 9], 4, VM::cpy_addr_addr_word);

        check_cycles(
            &[id::CX as u8 | INDIRECT, id::AH as u8],
            4,
            VM::cpy_reg_reg_byte,
        );
        check_cycles(
            &[id::AL as u8 | IND_OFFSET_REG, 0, 0, 0],
            5,
            VM::cpy_reg_addr_byte,
        );
        check_cycles(
            &[id::AX as u8 | IND_PRE_DEC, 10, 1],
            8,
            VM::cpy_reg_addr_word,
        );
    }
}
