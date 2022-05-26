use crate::VM;

impl VM {
    pub fn cmp_reg_num_byte(&mut self, signed: bool) -> usize {
        let lhs = self.read_arg_register();
        let rhs = self.read_arg_byte();
        let (offset, offset_cost) = self.pre_process(&lhs);
        let (lhs_value, read_value) = self.read_byte_reg(&lhs, offset);
        self.set_cmp_flags_byte(lhs_value, rhs, signed);
        self.post_process(&lhs);
        offset_cost + read_value
    }

    pub fn cmp_reg_num_word(&mut self, signed: bool) -> usize {
        let lhs = self.read_arg_register();
        let rhs = self.read_arg_word();
        let (offset, offset_cost) = self.pre_process(&lhs);
        let (lhs_value, read_value) = self.read_word_reg(&lhs, offset);
        self.set_cmp_flags_word(lhs_value, rhs, signed);
        self.post_process(&lhs);
        offset_cost + read_value
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maikor_platform::mem::address::RESERVED;
    use maikor_platform::registers::id;

    #[test]
    fn test_costs() {
        check_jmp_cycles(&[id::AL as u8, 15], 1, VM::cmp_reg_num_byte);
        check_jmp_cycles(&[id::AL as u8, 15], 2, VM::cmp_reg_num_word);
    }

    pub fn check_jmp_cycles(
        bytes: &[u8],
        expected_cycles: usize,
        method: fn(&mut VM, bool) -> usize,
    ) {
        let mut vm = VM::new_test();
        vm.arg_ptr = RESERVED;
        for (i, byte) in bytes.iter().enumerate() {
            vm.memory[RESERVED as usize + i] = *byte;
        }
        assert_eq!(method(&mut vm, true), expected_cycles);
        assert_eq!(method(&mut vm, false), expected_cycles);
    }
}
