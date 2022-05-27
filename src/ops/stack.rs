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

    fn write_to_stack(&mut self, value: u16) {
        let sp = self.get_sp() as usize;
        let bytes = value.to_be_bytes();
        self.memory[sp] = bytes[0];
        self.memory[sp + 1] = bytes[1];
        self.update_sp(2);
    }

    fn read_from_stack(&mut self) -> u16 {
        self.update_sp(-2);
        let sp = self.get_sp() as usize;
        u16::from_be_bytes([self.memory[sp], self.memory[sp + 1]])
    }

    /// calls fail() and returns false if the stack limited has been reached
    /// true if execution can continue
    fn setup_stack(&mut self, target_pc: u16) -> bool {
        if self.check_for_overflow(4) {
            self.write_to_stack(self.get_fp());
            self.write_to_stack(target_pc);
            false
        } else {
            true
        }
    }

    pub fn call_addr(&mut self) -> usize {
        if self.setup_stack(self.pc + 3) {
            self.pc = self.read_arg_word();
        }
        3
    }

    pub fn call_reg(&mut self) -> usize {
        let reg = self.read_arg_register();
        let (offset, offset_calc) = self.pre_process(&reg);
        let (addr, calc) = self.read_word_reg(&reg, offset);
        if self.setup_stack(self.pc + 2) {
            self.pc = addr;
        }
        offset_calc + calc + 3 + self.post_process(&reg)
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
        offset_cost + read_cost + self.post_process(&reg)
    }

    pub fn push_reg_word(&mut self) -> usize {
        0
    }

    pub fn push_num_byte(&mut self) -> usize {
        self.memory[address::SP] = self.read_arg_byte();
        self.update_sp(1);
        1
    }

    pub fn push_num_word(&mut self) -> usize {
        let value = self.read_arg_word();
        self.write_to_stack(value);
        1
    }

    pub fn pop_reg_byte(&mut self) -> usize {
        0
    }

    pub fn pop_reg_word(&mut self) -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::VM;

    #[test]
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
