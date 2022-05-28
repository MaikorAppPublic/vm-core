use crate::{address, VM};

impl VM {
    /// calls fail() and returns false if the stack limited has been reached
    /// true if execution can continue
    fn check_for_overflow(&mut self, room_needed: u16) -> bool {
        if self.get_sp() >= (65535 - room_needed) {
            self.fail(String::from("Stack overflow"));
            false
        } else {
            true
        }
    }

    fn update_sp(&mut self, diff: i16) {
        let sp = (self.get_sp() as i16 + diff).to_be_bytes();
        self.memory[address::SP] = sp[0];
        self.memory[address::SP + 1] = sp[1];
    }

    fn set_fp(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.memory[address::FP] = bytes[0];
        self.memory[address::FP + 1] = bytes[1];
    }

    #[must_use]
    fn write_to_stack(&mut self, value: u16) -> usize {
        let sp = self.get_sp() as usize;
        let bytes = value.to_be_bytes();
        self.memory[sp] = bytes[0];
        self.memory[sp + 1] = bytes[1];
        self.update_sp(2);
        2
    }

    fn read_from_stack(&mut self) -> u16 {
        self.update_sp(-2);
        let sp = self.get_sp() as usize;
        u16::from_be_bytes([self.memory[sp], self.memory[sp + 1]])
    }

    /// calls fail() and returns false if the stack limited has been reached
    /// true if execution can continue
    fn setup_stack(&mut self, target_pc: u16) -> (bool, usize) {
        if self.check_for_overflow(4) {
            (
                false,
                self.write_to_stack(self.get_fp()) + self.write_to_stack(target_pc),
            )
        } else {
            (true, 0)
        }
    }
}

impl VM {
    pub fn call_addr(&mut self) -> usize {
        let (result, cost) = self.setup_stack(self.pc + 3);
        if result {
            self.pc = self.read_arg_word();
        }
        cost
    }

    pub fn call_reg(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_calc) = self.pre_process(&reg);
        let (addr, calc) = self.read_word_reg(&reg, offset);
        let (result, cost) = self.setup_stack(self.pc + 2);
        if result {
            self.pc = addr;
        }
        offset_calc + calc + cost + self.post_process(&reg)
    }

    pub fn ret(&mut self) -> usize {
        self.pc = self.read_from_stack();
        let value = self.read_from_stack();
        self.set_fp(value);
        3
    }

    pub fn push_reg_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        let (value, read_cost) = self.read_byte_reg(&reg, offset);
        self.memory[address::SP] = value;
        self.update_sp(1);
        offset_cost + read_cost + self.post_process(&reg) + 1
    }

    pub fn push_reg_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        let (value, read_cost) = self.read_word_reg(&reg, offset);
        let write_cost = self.write_to_stack(value);
        write_cost + offset_cost + read_cost + self.post_process(&reg)
    }

    pub fn push_num_byte(&mut self) -> usize {
        self.memory[address::SP] = self.read_arg_byte();
        self.update_sp(1);
        1
    }

    pub fn push_num_word(&mut self) -> usize {
        let value = self.read_arg_word();
        self.write_to_stack(value)
    }

    pub fn pop_reg_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        self.update_sp(-1);
        let (addr, read_cost) = self.read_byte_mem(self.get_sp());
        let write_cost = self.write_byte_reg(&reg, offset, addr);
        offset_cost + read_cost + write_cost
    }

    pub fn pop_reg_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&reg);
        self.update_sp(-1);
        let (addr, read_cost) = self.read_word_mem(self.get_sp());
        let write_cost = self.write_word_reg(&reg, offset, addr);
        offset_cost + read_cost + write_cost
    }
}

#[cfg(test)]
mod test {
    use crate::ops::test::check_cycles;
    use crate::VM;

    #[test]
    fn test_cycles() {
        check_cycles(&[0, 0], 4, VM::call_addr);
        check_cycles(&[0, 0], 6, VM::call_reg);
        check_cycles(&[0, 0], 1, VM::push_num_byte);
        check_cycles(&[0, 0], 2, VM::push_num_word);
        check_cycles(&[0, 0], 2, VM::push_reg_byte);
        check_cycles(&[0, 0], 4, VM::push_reg_word);
        check_cycles(&[0, 0], 2, VM::pop_reg_byte);
        check_cycles(&[0, 0], 4, VM::pop_reg_word);
        check_cycles(&[0, 0], 3, VM::ret);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_internal_stack_commands() {
        let mut vm = VM::new_test();
        assert_eq!(vm.get_sp(), 0xFC18);
        assert_eq!(vm.get_fp(), 0);
        vm.write_to_stack(0xFFFF);
        assert_eq!(vm.get_sp(), 0xFC1A);
        assert_eq!(vm.get_fp(), 0);
        let value = vm.read_from_stack();
        assert_eq!(vm.get_sp(), 0xFC18);
        assert_eq!(vm.get_fp(), 0);
        assert_eq!(value, 0xFFFF);
        vm.write_to_stack(1);
        vm.write_to_stack(104);
        assert_eq!(vm.get_sp(), 0xFC1C);
        assert_eq!(vm.get_fp(), 0);
        let value = vm.read_from_stack();
        assert_eq!(vm.get_sp(), 0xFC1A);
        assert_eq!(vm.get_fp(), 0);
        assert_eq!(value, 104);

        assert_eq!(vm.get_fp(), 0);
        vm.set_fp(15);
        assert_eq!(vm.get_fp(), 15);
        vm.set_fp(2215);
        assert_eq!(vm.get_fp(), 2215);

        assert_eq!(vm.get_sp(), 0xFC1A);
        vm.update_sp(2);
        assert_eq!(vm.get_sp(), 0xFC1C);
        vm.update_sp(-4);
        assert_eq!(vm.get_sp(), 0xFC18);
    }
}
