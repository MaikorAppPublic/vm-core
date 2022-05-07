use crate::internals::register_access::RegisterAccess;
use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;
use maikor_language::registers::flags;

impl VM {
    pub fn jump(&mut self, addr: Address) {
        self.pc = addr.0;
    }

    pub fn jump_reg(&mut self, reg: &Register) {
        let addr: Word = self.read(full::JMP_REG, &reg);
        self.jump(addr.to_address());
    }

    pub fn jmp_reg(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        self.jump_reg(&reg);
        self.process_arg(&reg, true);
    }

    pub fn jmp_addr(&mut self, addr: Address) {
        self.jump(addr);
    }

    pub fn je_reg(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        if !self.check_flag(flags::GREATER_THAN) && !self.check_flag(flags::LESS_THAN) {
            self.jump_reg(&reg);
        }
        self.process_arg(&reg, true);
    }

    pub fn je_addr(&mut self, addr: Address) {
        if !self.check_flag(flags::GREATER_THAN) && !self.check_flag(flags::LESS_THAN) {
            self.jump(addr);
        }
    }

    pub fn jne_reg(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        if self.check_flag(flags::GREATER_THAN) || self.check_flag(flags::LESS_THAN) {
            self.jump_reg(&reg);
        }
        self.process_arg(&reg, true);
    }

    pub fn jne_addr(&mut self, addr: Address) {
        if self.check_flag(flags::GREATER_THAN) || self.check_flag(flags::LESS_THAN) {
            self.jump(addr);
        }
    }

    pub fn jg_reg(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        if self.check_flag(flags::GREATER_THAN) {
            self.jump_reg(&reg);
        }
        self.process_arg(&reg, true);
    }

    pub fn jg_addr(&mut self, addr: Address) {
        if self.check_flag(flags::GREATER_THAN) {
            self.jump(addr);
        }
    }

    pub fn jl_reg(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        if self.check_flag(flags::LESS_THAN) {
            self.jump_reg(&reg);
        }
        self.process_arg(&reg, true);
    }

    pub fn jl_addr(&mut self, addr: Address) {
        if self.check_flag(flags::LESS_THAN) {
            self.jump(addr);
        }
    }

    pub fn jge_reg(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        if !self.check_flag(flags::LESS_THAN) {
            self.jump_reg(&reg);
        }
        self.process_arg(&reg, true);
    }

    pub fn jge_addr(&mut self, addr: Address) {
        if !self.check_flag(flags::LESS_THAN) {
            self.jump(addr);
        }
    }

    pub fn jle_reg(&mut self, reg: Register) {
        self.process_arg(&reg, false);
        if !self.check_flag(flags::GREATER_THAN) {
            self.jump_reg(&reg);
        }
        self.process_arg(&reg, true);
    }

    pub fn jle_addr(&mut self, addr: Address) {
        if !self.check_flag(flags::GREATER_THAN) {
            self.jump(addr);
        }
    }

    pub fn jrf_num(&mut self, amount: Byte) {
        self.pc = self.pc.wrapping_add(amount.0 as u16);
    }

    pub fn jrb_num(&mut self, amount: Byte) {
        self.pc = self.pc.wrapping_sub(amount.0 as u16);
    }

    pub fn jbs_reg_num(&mut self, dst: Register, mask: Byte) {
        self.process_arg(&dst, false);
        if self.check_flag(mask.0) {
            self.jump_reg(&dst);
        }
        self.process_arg(&dst, true);
    }

    pub fn jbs_reg_reg(&mut self, dst: Register, mask: Register) {
        self.process_arg(&mask, false);
        self.process_arg(&dst, false);
        let mask_byte: Byte = self.read(full::JBS_REG_REG, &mask);
        if self.check_flag(mask_byte.0) {
            self.jump_reg(&dst);
        }
        self.process_arg(&dst, true);
        self.process_arg(&mask, true);
    }

    pub fn jbs_addr_num(&mut self, dst: Address, mask: Byte) {
        if self.check_flag(mask.0) {
            self.jump(dst);
        }
    }

    pub fn jbs_addr_reg(&mut self, dst: Address, mask: Register) {
        self.process_arg(&mask, false);
        let mask_byte: Byte = self.read(full::JBS_ADDR_REG, &mask);
        if self.check_flag(mask_byte.0) {
            self.jump(dst);
        }
        self.process_arg(&mask, true);
    }

    pub fn jbc_reg_num(&mut self, dst: Register, mask: Byte) {
        self.process_arg(&dst, false);
        if !self.check_flag(mask.0) {
            self.jump_reg(&dst);
        }
        self.process_arg(&dst, true);
    }

    pub fn jbc_reg_reg(&mut self, dst: Register, mask: Register) {
        self.process_arg(&mask, false);
        self.process_arg(&dst, false);
        let mask_byte: Byte = self.read(full::JBC_REG_REG, &mask);
        if !self.check_flag(mask_byte.0) {
            self.jump_reg(&dst);
        }
        self.process_arg(&dst, true);
        self.process_arg(&mask, true);
    }

    pub fn jbc_addr_num(&mut self, dst: Address, mask: Byte) {
        if !self.check_flag(mask.0) {
            self.jump(dst);
        }
    }

    pub fn jbc_addr_reg(&mut self, dst: Address, mask: Register) {
        self.process_arg(&mask, false);
        let mask_byte: Byte = self.read(full::JBC_ADDR_REG, &mask);
        if !self.check_flag(mask_byte.0) {
            self.jump(dst);
        }
        self.process_arg(&mask, true);
    }
}
