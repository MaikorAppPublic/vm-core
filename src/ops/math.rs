use crate::internals::flags::{has_overflowed_byte, has_overflowed_word};
use crate::VM;

impl VM {
    pub fn math_reg_num_byte(&mut self, method: fn(u8, u8) -> (u8, bool)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_byte();
        let (offset, offset_cost) = self.pre_process(&dst, 1);
        let (dst_value, read_cost) = self.read_byte_reg(&dst, offset);
        let (result, carried) = method(dst_value, src);
        let write_cost = self.write_byte_reg(&dst, offset, result);
        self.set_math_flags_byte(result, carried, has_overflowed_byte(dst_value, result));
        self.post_process(&dst, 1) + read_cost + write_cost + offset_cost
    }

    pub fn math_reg_reg_byte(&mut self, method: fn(u8, u8) -> (u8, bool)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, offset_cost1) = self.pre_process(&dst, 1);
        let (src_offset, offset_cost2) = self.pre_process(&src, 1);
        let (dst_value, read_cost1) = self.read_byte_reg(&dst, dst_offset);
        let (src_value, read_cost2) = self.read_byte_reg(&src, src_offset);
        let (result, carried) = method(dst_value, src_value);
        let write_cost = self.write_byte_reg(&dst, dst_offset, result);
        self.set_math_flags_byte(result, carried, has_overflowed_byte(dst_value, result));
        self.post_process(&src, 1)
            + self.post_process(&dst, 1)
            + write_cost
            + read_cost1
            + read_cost2
            + offset_cost1
            + offset_cost2
    }

    pub fn math_addr_num_byte(&mut self, method: fn(u8, u8) -> (u8, bool)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_byte();
        let (dst_value, read_cost) = self.read_byte_mem(dst);
        let (result, carried) = method(dst_value, src);
        let write_cost = self.write_byte_mem(dst, result);
        self.set_math_flags_byte(result, carried, has_overflowed_byte(dst_value, result));
        read_cost + write_cost
    }

    pub fn math_addr_reg_byte(&mut self, method: fn(u8, u8) -> (u8, bool)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 1);
        let (dst_value, read_cost1) = self.read_byte_mem(dst);
        let (src_value, read_cost2) = self.read_byte_reg(&src, offset);
        let (result, carried) = method(dst_value, src_value);
        let write_cost = self.write_byte_mem(dst, result);
        self.set_math_flags_byte(result, carried, has_overflowed_byte(dst_value, result));
        self.post_process(&src, 1) + read_cost1 + read_cost2 + offset_cost + write_cost
    }

    pub fn math_addr_addr_byte(&mut self, method: fn(u8, u8) -> (u8, bool)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_word();
        let (dst_value, read_cost1) = self.read_byte_mem(dst);
        let (src_value, read_cost2) = self.read_byte_mem(src);
        let (result, carried) = method(dst_value, src_value);
        let write_cost = self.write_byte_mem(dst, result);
        self.set_math_flags_byte(result, carried, has_overflowed_byte(dst_value, result));
        read_cost1 + read_cost2 + write_cost
    }

    pub fn math_reg_addr_byte(&mut self, method: fn(u8, u8) -> (u8, bool)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let (offset, offset_cost) = self.pre_process(&dst, 1);
        let (dst_value, read_cost1) = self.read_byte_reg(&dst, offset);
        let (src_value, read_cost2) = self.read_byte_mem(src);
        let (result, carried) = method(dst_value, src_value);
        let write_cost = self.write_byte_reg(&dst, offset, result);
        self.set_math_flags_byte(result, carried, has_overflowed_byte(dst_value, result));
        self.post_process(&dst, 1) + write_cost + read_cost1 + read_cost2 + offset_cost
    }

    pub fn math_reg_num_word(&mut self, method: fn(u16, u16) -> (u16, bool)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let (offset, offset_cost) = self.pre_process(&dst, 2);
        let (dst_value, read_cost) = self.read_word_reg(&dst, offset);
        let (result, carried) = method(dst_value, src);
        let write_cost = self.write_word_reg(&dst, offset, result);
        self.set_math_flags_word(result, carried, has_overflowed_word(dst_value, result));
        self.post_process(&dst, 2) + read_cost + write_cost + offset_cost
    }

    pub fn math_reg_reg_word(&mut self, method: fn(u16, u16) -> (u16, bool)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (dst_offset, offset_cost1) = self.pre_process(&dst, 2);
        let (src_offset, offset_cost2) = self.pre_process(&src, 2);
        let (dst_value, read_cost1) = self.read_word_reg(&dst, dst_offset);
        let (src_value, read_cost2) = self.read_word_reg(&src, src_offset);
        let (result, carried) = method(dst_value, src_value);
        let write_cost = self.write_word_reg(&dst, dst_offset, result);
        self.set_math_flags_word(result, carried, has_overflowed_word(dst_value, result));
        self.post_process(&src, 2)
            + self.post_process(&dst, 2)
            + write_cost
            + read_cost1
            + read_cost2
            + offset_cost1
            + offset_cost2
    }

    pub fn math_addr_num_word(&mut self, method: fn(u16, u16) -> (u16, bool)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_word();
        let (dst_value, read_cost) = self.read_word_mem(dst);
        let (result, carried) = method(dst_value, src);
        let write_cost = self.write_word_mem(dst, result);
        self.set_math_flags_word(result, carried, has_overflowed_word(dst_value, result));
        read_cost + write_cost
    }

    pub fn math_addr_reg_word(&mut self, method: fn(u16, u16) -> (u16, bool)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 2);
        let (dst_value, read_cost1) = self.read_word_mem(dst);
        let (src_value, read_cost2) = self.read_word_reg(&src, offset);
        let (result, carried) = method(dst_value, src_value);
        let write_cost = self.write_word_mem(dst, result);
        self.set_math_flags_word(result, carried, has_overflowed_word(dst_value, result));
        self.post_process(&src, 2) + read_cost1 + read_cost2 + offset_cost + write_cost
    }

    pub fn math_addr_addr_word(&mut self, method: fn(u16, u16) -> (u16, bool)) -> usize {
        let dst = self.read_arg_word();
        let src = self.read_arg_word();
        let (dst_value, read_cost1) = self.read_word_mem(dst);
        let (src_value, read_cost2) = self.read_word_mem(src);
        let (result, carried) = method(dst_value, src_value);
        let write_cost = self.write_word_mem(dst, result);
        self.set_math_flags_word(result, carried, has_overflowed_word(dst_value, result));
        read_cost1 + read_cost2 + write_cost
    }

    pub fn math_reg_addr_word(&mut self, method: fn(u16, u16) -> (u16, bool)) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_word();
        let (offset, offset_cost) = self.pre_process(&dst, 2);
        let (dst_value, read_cost1) = self.read_word_reg(&dst, offset);
        let (src_value, read_cost2) = self.read_word_mem(src);
        let (result, carried) = method(dst_value, src_value);
        let write_cost = self.write_word_reg(&dst, offset, result);
        self.set_math_flags_word(result, carried, has_overflowed_word(dst_value, result));
        self.post_process(&dst, 2) + write_cost + read_cost1 + read_cost2 + offset_cost
    }
}

#[allow(clippy::type_complexity)]
#[cfg(test)]
mod test {
    use super::*;
    use maikor_platform::mem::address::RESERVED;
    use maikor_platform::registers::id;

    #[test]
    fn test_costs() {
        check_math_methods_b(u8::overflowing_add);
        check_math_methods_b(u8::overflowing_sub);
        check_math_methods_b(u8::overflowing_mul);
        check_math_methods_w(u16::overflowing_add);
        check_math_methods_w(u16::overflowing_sub);
        check_math_methods_w(u16::overflowing_mul);
    }

    pub fn check_math_methods_b(math_method: fn(u8, u8) -> (u8, bool)) {
        check_byte_math_cycles(&[id::AL as u8, 10], 2, math_method, VM::math_reg_num_byte);
        check_byte_math_cycles(
            &[id::AL as u8, id::AH as u8],
            3,
            math_method,
            VM::math_reg_reg_byte,
        );
        check_byte_math_cycles(
            &[id::CH as u8, 0, 100],
            3,
            math_method,
            VM::math_reg_addr_byte,
        );
        check_byte_math_cycles(&[1, 1, 60], 2, math_method, VM::math_addr_num_byte);
        check_byte_math_cycles(
            &[9, 9, id::AL as u8],
            3,
            math_method,
            VM::math_addr_reg_byte,
        );
        check_byte_math_cycles(&[0, 0, 1, 1], 3, math_method, VM::math_addr_addr_byte);
    }

    pub fn check_math_methods_w(math_method: fn(u16, u16) -> (u16, bool)) {
        check_word_math_cycles(&[id::AX as u8, 10], 4, math_method, VM::math_reg_num_word);
        check_word_math_cycles(
            &[id::AL as u8, id::AH as u8],
            6,
            math_method,
            VM::math_reg_reg_word,
        );
        check_word_math_cycles(
            &[id::CX as u8, 0, 100],
            6,
            math_method,
            VM::math_reg_addr_word,
        );
        check_word_math_cycles(&[1, 1, 60], 4, math_method, VM::math_addr_num_word);
        check_word_math_cycles(
            &[9, 9, id::AX as u8],
            6,
            math_method,
            VM::math_addr_reg_word,
        );
        check_word_math_cycles(&[0, 0, 1, 1], 6, math_method, VM::math_addr_addr_word);
    }

    pub fn check_byte_math_cycles(
        bytes: &[u8],
        expected_cycles: usize,
        math_method: fn(u8, u8) -> (u8, bool),
        op_method: fn(&mut VM, fn(u8, u8) -> (u8, bool)) -> usize,
    ) {
        let mut vm = VM::new_test();
        vm.arg_ptr = RESERVED;
        for (i, byte) in bytes.iter().enumerate() {
            vm.memory[RESERVED as usize + i] = *byte;
        }
        assert_eq!(op_method(&mut vm, math_method), expected_cycles)
    }

    pub fn check_word_math_cycles(
        bytes: &[u8],
        expected_cycles: usize,
        math_method: fn(u16, u16) -> (u16, bool),
        op_method: fn(&mut VM, fn(u16, u16) -> (u16, bool)) -> usize,
    ) {
        let mut vm = VM::new_test();
        vm.arg_ptr = RESERVED;
        for (i, byte) in bytes.iter().enumerate() {
            vm.memory[RESERVED as usize + i] = *byte;
        }
        assert_eq!(op_method(&mut vm, math_method), expected_cycles)
    }
}
