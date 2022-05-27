use crate::VM;

impl VM {
    pub fn inc_reg_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        let (value, cost) = self.read_byte_reg(&reg, offset);
        let (value, _) = value.overflowing_add(1);
        let final_cost = self.write_byte_reg(&reg, offset, value) + cost;
        final_cost + offset_cost + self.post_process(&reg)
    }

    pub fn dec_reg_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        let (value, cost) = self.read_byte_reg(&reg, offset);
        let (value, _) = value.overflowing_sub(1);
        let final_cost = self.write_byte_reg(&reg, offset, value) + cost;
        final_cost + offset_cost + self.post_process(&reg)
    }

    pub fn inc_reg_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        let (value, cost) = self.read_word_reg(&reg, offset);
        let (value, _) = value.overflowing_add(1);
        let final_cost = self.write_word_reg(&reg, offset, value) + cost;
        final_cost + offset_cost + self.post_process(&reg)
    }

    pub fn dec_reg_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        let (value, cost) = self.read_word_reg(&reg, offset);
        let (value, _) = value.overflowing_sub(1);
        let final_cost = self.write_word_reg(&reg, offset, value) + cost;
        final_cost + offset_cost + self.post_process(&reg)
    }

    pub fn inc_addr_byte(&mut self) -> usize {
        let addr = self.read_arg_word();
        let (value, cost) = self.read_byte_mem(addr);
        let (value, _) = value.overflowing_add(1);
        self.write_byte_mem(addr, value) + cost
    }

    pub fn dec_addr_byte(&mut self) -> usize {
        let addr = self.read_arg_word();
        let (value, cost) = self.read_byte_mem(addr);
        let (value, _) = value.overflowing_sub(1);
        self.write_byte_mem(addr, value) + cost
    }

    pub fn inc_addr_word(&mut self) -> usize {
        let addr = self.read_arg_word();
        let (value, cost) = self.read_word_mem(addr);
        let (value, _) = value.overflowing_add(1);
        self.write_word_mem(addr, value) + cost
    }

    pub fn dec_addr_word(&mut self) -> usize {
        let addr = self.read_arg_word();
        let (value, cost) = self.read_word_mem(addr);
        let (value, _) = value.overflowing_sub(1);
        self.write_word_mem(addr, value) + cost
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
        check_cycles(&[id::AL as u8], 2, VM::inc_reg_byte);
        check_cycles(&[id::AX as u8], 4, VM::inc_reg_word);
        check_cycles(&[id::AX as u8 | INDIRECT], 6, VM::inc_reg_byte);
        check_cycles(&[id::AX as u8 | IND_OFFSET_NUM], 6, VM::inc_reg_byte);
        check_cycles(&[id::AX as u8 | INDIRECT], 8, VM::inc_reg_word);
        check_cycles(&[id::AX as u8 | IND_PRE_DEC], 10, VM::inc_reg_word);
        check_cycles(&[id::AL as u8], 2, VM::dec_reg_byte);
        check_cycles(&[id::AX as u8], 4, VM::dec_reg_word);
        check_cycles(&[id::AX as u8 | INDIRECT], 6, VM::dec_reg_byte);
        check_cycles(&[id::AX as u8 | INDIRECT], 8, VM::dec_reg_word);
        check_cycles(&[id::AX as u8 | IND_OFFSET_REG], 9, VM::dec_reg_word);
        check_cycles(
            &[id::AX as u8 | IND_OFFSET_EXT_REG, id::BX as u8],
            10,
            VM::dec_reg_word,
        );
        check_cycles(&[0], 2, VM::inc_addr_byte);
        check_cycles(&[0], 4, VM::inc_addr_word);
        check_cycles(&[0], 2, VM::dec_addr_byte);
        check_cycles(&[0], 4, VM::dec_addr_word);
    }
}
