use crate::VM;
use maikor_platform::registers::flags::{GREATER_THAN, LESS_THAN};

impl VM {
    fn jmp_addr_conditional(&mut self, jump: bool) -> (bool, usize) {
        let addr = self.read_arg_word();
        if jump {
            self.pc = addr;
            (true, 1)
        } else {
            (false, 1)
        }
    }

    fn jmp_reg_conditional(&mut self, jump: bool) -> (bool, usize) {
        let dst = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&dst);
        let (addr, read_cost) = self.read_word_reg(&dst, offset);
        let result = if jump {
            self.pc = addr;
            (true, offset_cost + read_cost)
        } else {
            (false, offset_cost + read_cost)
        };
        (result.0, result.1 + self.post_process(&dst))
    }

    pub fn jmp_addr(&mut self) -> (bool, usize) {
        self.jmp_addr_conditional(true)
    }

    pub fn je_addr(&mut self) -> (bool, usize) {
        self.jmp_addr_conditional(!self.check_flag(LESS_THAN) && !self.check_flag(GREATER_THAN))
    }

    pub fn jne_addr(&mut self) -> (bool, usize) {
        self.jmp_addr_conditional(self.check_flag(LESS_THAN) || self.check_flag(GREATER_THAN))
    }

    pub fn jl_addr(&mut self) -> (bool, usize) {
        self.jmp_addr_conditional(self.check_flag(LESS_THAN))
    }

    pub fn jg_addr(&mut self) -> (bool, usize) {
        self.jmp_addr_conditional(self.check_flag(GREATER_THAN))
    }

    pub fn jle_addr(&mut self) -> (bool, usize) {
        self.jmp_addr_conditional(!self.check_flag(GREATER_THAN))
    }

    pub fn jge_addr(&mut self) -> (bool, usize) {
        self.jmp_addr_conditional(!self.check_flag(LESS_THAN))
    }

    pub fn jmp_reg(&mut self) -> (bool, usize) {
        self.jmp_reg_conditional(true)
    }

    pub fn je_reg(&mut self) -> (bool, usize) {
        self.jmp_reg_conditional(!self.check_flag(LESS_THAN) && !self.check_flag(GREATER_THAN))
    }

    pub fn jne_reg(&mut self) -> (bool, usize) {
        self.jmp_reg_conditional(self.check_flag(LESS_THAN) || self.check_flag(GREATER_THAN))
    }

    pub fn jl_reg(&mut self) -> (bool, usize) {
        self.jmp_reg_conditional(self.check_flag(LESS_THAN))
    }

    pub fn jg_reg(&mut self) -> (bool, usize) {
        self.jmp_reg_conditional(self.check_flag(GREATER_THAN))
    }

    pub fn jle_reg(&mut self) -> (bool, usize) {
        self.jmp_reg_conditional(!self.check_flag(GREATER_THAN))
    }

    pub fn jge_reg(&mut self) -> (bool, usize) {
        self.jmp_reg_conditional(!self.check_flag(LESS_THAN))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ops::test::check_jmp_cycles;

    #[test]
    fn test_costs() {
        check_jmp_cycles(&[0, 0], 1, VM::jmp_addr);
        check_jmp_cycles(&[0, 0], 1, VM::je_addr);
        check_jmp_cycles(&[0, 0], 1, VM::jne_addr);
        check_jmp_cycles(&[0, 0], 1, VM::jg_addr);
        check_jmp_cycles(&[0, 0], 1, VM::jl_addr);
        check_jmp_cycles(&[0, 0], 1, VM::jge_addr);
        check_jmp_cycles(&[0, 0], 1, VM::jle_addr);
        check_jmp_cycles(&[0], 2, VM::jmp_reg);
        check_jmp_cycles(&[0], 2, VM::je_reg);
        check_jmp_cycles(&[0], 2, VM::jne_reg);
        check_jmp_cycles(&[0], 2, VM::jg_reg);
        check_jmp_cycles(&[0], 2, VM::jl_reg);
        check_jmp_cycles(&[0], 2, VM::jge_reg);
        check_jmp_cycles(&[0], 2, VM::jle_reg);
    }
}
