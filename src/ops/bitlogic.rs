use crate::VM;
use std::ops::Not;

impl VM {
    pub fn not_reg_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg, 1);
        let (num, read_cost) = self.read_byte_reg(&reg, offset);
        let write_cost = self.write_byte_reg(&reg, offset, num.not());
        offset_cost + read_cost + write_cost + self.post_process(&reg, 1)
    }

    pub fn not_reg_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg, 2);
        let (num, read_cost) = self.read_word_reg(&reg, offset);
        let write_cost = self.write_word_reg(&reg, offset, num.not());
        offset_cost + read_cost + write_cost + self.post_process(&reg, 2)
    }

    pub fn bl_reg_reg_byte(&mut self, method: fn(u8, u8) -> u8) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst, 1);
        let (src_offset, src_offset_cost) = self.pre_process(&src, 1);
        let (dst_value, dst_read_cost) = self.read_byte_reg(&dst, dst_offset);
        let (src_value, src_read_cost) = self.read_byte_reg(&src, src_offset);
        let value = method(dst_value, src_value);
        let write_cost = self.write_byte_reg(&dst, dst_offset, value);
        dst_offset_cost
            + src_offset_cost
            + dst_read_cost
            + src_read_cost
            + write_cost
            + self.post_process(&dst, 1)
            + self.post_process(&src, 1)
    }

    pub fn bl_reg_reg_word(&mut self, method: fn(u16, u16) -> u16) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst, 2);
        let (src_offset, src_offset_cost) = self.pre_process(&src, 2);
        let (dst_value, dst_read_cost) = self.read_word_reg(&dst, dst_offset);
        let (src_value, src_read_cost) = self.read_word_reg(&src, src_offset);
        let value = method(dst_value, src_value);
        let write_cost = self.write_word_reg(&dst, dst_offset, value);
        dst_offset_cost
            + src_offset_cost
            + dst_read_cost
            + src_read_cost
            + write_cost
            + self.post_process(&dst, 2)
            + self.post_process(&src, 2)
    }

    pub fn bl_reg_num_byte(&mut self, method: fn(u8, u8) -> u8) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_byte();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst, 1);
        let (dst_value, dst_read_cost) = self.read_byte_reg(&dst, dst_offset);
        let value = method(dst_value, src);
        let write_cost = self.write_byte_reg(&dst, dst_offset, value);
        dst_offset_cost + dst_read_cost + write_cost + self.post_process(&dst, 1)
    }

    pub fn bl_reg_num_word(&mut self, method: fn(u16, u16) -> u16) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let (dst_offset, dst_offset_cost) = self.pre_process(&dst, 2);
        let (dst_value, dst_read_cost) = self.read_word_reg(&dst, dst_offset);
        let value = method(dst_value, src);
        let write_cost = self.write_word_reg(&dst, dst_offset, value);
        dst_offset_cost + dst_read_cost + write_cost + self.post_process(&dst, 2)
    }
}

#[cfg(test)]
mod test {
    use crate::ops::test::check_cycles;
    use crate::VM;
    use maikor_platform::mem::address::RESERVED;
    use std::ops::{BitAnd, BitOr, BitXor};

    pub fn bitwise_check_cycles_byte(
        bytes: &[u8],
        expected_cycles: usize,
        method: fn(&mut VM, fn(u8, u8) -> u8) -> usize,
        bmethod: fn(u8, u8) -> u8,
    ) {
        let mut vm = VM::new_test();
        vm.arg_ptr = RESERVED;
        for (i, byte) in bytes.iter().enumerate() {
            vm.memory[RESERVED as usize + i] = *byte;
        }
        assert_eq!(method(&mut vm, bmethod), expected_cycles)
    }

    pub fn bitwise_check_cycles_word(
        bytes: &[u8],
        expected_cycles: usize,
        method: fn(&mut VM, fn(u16, u16) -> u16) -> usize,
        bmethod: fn(u16, u16) -> u16,
    ) {
        let mut vm = VM::new_test();
        vm.arg_ptr = RESERVED;
        for (i, byte) in bytes.iter().enumerate() {
            vm.memory[RESERVED as usize + i] = *byte;
        }
        assert_eq!(method(&mut vm, bmethod), expected_cycles)
    }

    #[test]
    fn test_costs() {
        check_cycles(&[0, 0], 2, VM::not_reg_byte);
        check_cycles(&[0, 0], 4, VM::not_reg_word);

        bitwise_check_cycles_byte(&[0, 0], 3, VM::bl_reg_reg_byte, u8::bitand);
        bitwise_check_cycles_byte(&[0, 0], 3, VM::bl_reg_reg_byte, u8::bitor);
        bitwise_check_cycles_byte(&[0, 0], 3, VM::bl_reg_reg_byte, u8::bitxor);
        bitwise_check_cycles_word(&[0, 0], 6, VM::bl_reg_reg_word, u16::bitand);
        bitwise_check_cycles_word(&[0, 0], 6, VM::bl_reg_reg_word, u16::bitor);
        bitwise_check_cycles_word(&[0, 0], 6, VM::bl_reg_reg_word, u16::bitxor);

        bitwise_check_cycles_byte(&[0, 0], 2, VM::bl_reg_num_byte, u8::bitand);
        bitwise_check_cycles_byte(&[0, 0], 2, VM::bl_reg_num_byte, u8::bitor);
        bitwise_check_cycles_byte(&[0, 0], 2, VM::bl_reg_num_byte, u8::bitxor);
        bitwise_check_cycles_word(&[0, 0], 4, VM::bl_reg_num_word, u16::bitand);
        bitwise_check_cycles_word(&[0, 0], 4, VM::bl_reg_num_word, u16::bitor);
        bitwise_check_cycles_word(&[0, 0], 4, VM::bl_reg_num_word, u16::bitxor);
    }
}
