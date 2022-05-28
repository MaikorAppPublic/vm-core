use crate::internals::flags::{
    is_first_bit_set_byte, is_first_bit_set_word, is_last_bit_set_byte, is_last_bit_set_word,
    set_first_bit_byte, set_first_bit_word, set_last_bit_byte, set_last_bit_word,
};
use crate::register::Register;
use crate::VM;
use maikor_platform::registers::flags::CARRY;
use std::ops::{Shl, Shr};

// ASL   left most goes to carry, 0 into right most
// LSR   right most goes to carry, 0 into left most
// ASR   right most goes to carry, sign into left most
// ROL   right most goes to left most
// ROR   left most goes to right most
// RCL   right most goes to carry, carry goes to left most
// RCR   left most goes to carry, carry goes to right most

/// bitwise internal methods
impl VM {
    fn bitwise_addr_byte(&mut self, method: fn(u8, u32) -> u8) -> usize {
        let addr = self.read_arg_word();
        let (value, read_value) = self.read_byte_mem(addr);
        let result = method(value, 1);
        let write_value = self.write_byte_mem(addr, result);
        read_value + write_value
    }

    fn bitwise_addr_word(&mut self, method: fn(u16, u32) -> u16) -> usize {
        let addr = self.read_arg_word();
        let (value, read_value) = self.read_word_mem(addr);
        let result = method(value, 1);
        let write_value = self.write_word_mem(addr, result);
        read_value + write_value
    }

    fn bitwise_reg_byte(&mut self, reg: Register, amount: u32, method: fn(u8, u32) -> u8) -> usize {
        let (offset, offset_cost) = self.pre_process(&reg, 1);
        let (value, read_cost) = self.read_byte_reg(&reg, offset);
        let result = method(value, amount);
        let write_cost = self.write_byte_reg(&reg, offset, result);
        write_cost + read_cost + offset_cost + self.post_process(&reg, 1)
    }

    fn bitwise_reg_word(
        &mut self,
        reg: Register,
        amount: u32,
        method: fn(u16, u32) -> u16,
    ) -> usize {
        let (offset, offset_cost) = self.pre_process(&reg, 2);
        let (value, read_cost) = self.read_word_reg(&reg, offset);
        let result = method(value, amount);
        let write_cost = self.write_word_reg(&reg, offset, result);
        write_cost + read_cost + offset_cost + self.post_process(&reg, 2)
    }

    fn rcl_byte(&mut self, reg: Register, amount: u32) -> usize {
        let (offset, offset_cost) = self.pre_process(&reg, 1);
        let (mut value, read_cost) = self.read_byte_reg(&reg, offset);
        for _ in 0..amount {
            let set_carry = is_first_bit_set_byte(value);
            value = value.shl(1);
            if self.check_flag(CARRY) {
                set_last_bit_byte(value);
            }
            if set_carry {
                self.set_flag(CARRY)
            } else {
                self.clear_flag(CARRY)
            }
        }
        let write_cost = self.write_byte_reg(&reg, offset, value);
        write_cost + read_cost + offset_cost + (amount * 2) as usize + self.post_process(&reg, 1)
    }

    fn rcr_byte(&mut self, reg: Register, amount: u32) -> usize {
        let (offset, offset_cost) = self.pre_process(&reg, 1);
        let (mut value, read_cost) = self.read_byte_reg(&reg, offset);
        for _ in 0..amount {
            let set_carry = is_last_bit_set_byte(value);
            value = value.shr(1);
            if self.check_flag(CARRY) {
                set_first_bit_byte(value);
            }
            if set_carry {
                self.set_flag(CARRY)
            } else {
                self.clear_flag(CARRY)
            }
        }
        let write_cost = self.write_byte_reg(&reg, offset, value);
        write_cost + read_cost + offset_cost + (amount * 2) as usize + self.post_process(&reg, 1)
    }

    fn rcl_word(&mut self, reg: Register, amount: u32) -> usize {
        let (offset, offset_cost) = self.pre_process(&reg, 2);
        let (mut value, read_cost) = self.read_word_reg(&reg, offset);
        for _ in 0..amount {
            let set_carry = is_first_bit_set_word(value);
            value = value.shl(1);
            if self.check_flag(CARRY) {
                set_last_bit_word(value);
            }
            if set_carry {
                self.set_flag(CARRY)
            } else {
                self.clear_flag(CARRY)
            }
        }
        let write_cost = self.write_word_reg(&reg, offset, value);
        write_cost + read_cost + offset_cost + (amount * 2) as usize + self.post_process(&reg, 2)
    }

    fn rcr_word(&mut self, reg: Register, amount: u32) -> usize {
        let (offset, offset_cost) = self.pre_process(&reg, 2);
        let (mut value, read_cost) = self.read_word_reg(&reg, offset);
        for _ in 0..amount {
            let set_carry = is_last_bit_set_word(value);
            value = value.shr(1);
            if self.check_flag(CARRY) {
                set_first_bit_word(value);
            }
            if set_carry {
                self.set_flag(CARRY)
            } else {
                self.clear_flag(CARRY)
            }
        }
        let write_cost = self.write_word_reg(&reg, offset, value);
        write_cost + read_cost + offset_cost + (amount * 2) as usize + self.post_process(&reg, 2)
    }
}

/// ADDR   WORD
impl VM {
    pub fn asl_addr_word(&mut self) -> usize {
        self.bitwise_addr_word(u16::shl)
    }

    pub fn asr_addr_word(&mut self) -> usize {
        self.bitwise_addr_word(asr_word)
    }

    pub fn lsr_addr_word(&mut self) -> usize {
        self.bitwise_addr_word(u16::shr)
    }

    pub fn rol_addr_word(&mut self) -> usize {
        self.bitwise_addr_word(u16::rotate_left)
    }

    pub fn ror_addr_word(&mut self) -> usize {
        self.bitwise_addr_word(u16::rotate_right)
    }

    pub fn rcl_addr_word(&mut self) -> usize {
        let addr = self.read_arg_word();
        let (value, read_value) = self.read_word_mem(addr);
        let set_carry = is_first_bit_set_word(value);
        let result = value.shl(1);
        if self.check_flag(CARRY) {
            set_last_bit_word(result);
        }
        if set_carry {
            self.set_flag(CARRY)
        } else {
            self.clear_flag(CARRY)
        }
        let write_value = self.write_word_mem(addr, result);
        read_value + write_value + 3
    }

    pub fn rcr_addr_word(&mut self) -> usize {
        let addr = self.read_arg_word();
        let (value, read_value) = self.read_word_mem(addr);
        let set_carry = is_first_bit_set_word(value);
        let result = value.shr(1);
        if self.check_flag(CARRY) {
            set_last_bit_word(result);
        }
        if set_carry {
            self.set_flag(CARRY)
        } else {
            self.clear_flag(CARRY)
        }
        let write_value = self.write_word_mem(addr, result);
        read_value + write_value + 3
    }
}

/// ADDR   BYTE
impl VM {
    pub fn asl_addr_byte(&mut self) -> usize {
        self.bitwise_addr_byte(u8::shl)
    }

    pub fn asr_addr_byte(&mut self) -> usize {
        self.bitwise_addr_byte(asr_byte)
    }

    pub fn lsr_addr_byte(&mut self) -> usize {
        self.bitwise_addr_byte(u8::shr)
    }

    pub fn rol_addr_byte(&mut self) -> usize {
        self.bitwise_addr_byte(u8::rotate_left)
    }

    pub fn ror_addr_byte(&mut self) -> usize {
        self.bitwise_addr_byte(u8::rotate_right)
    }

    pub fn rcl_addr_byte(&mut self) -> usize {
        let addr = self.read_arg_word();
        let (value, read_value) = self.read_byte_mem(addr);
        let set_carry = is_first_bit_set_byte(value);
        let result = value.shl(1);
        if self.check_flag(CARRY) {
            set_last_bit_byte(result);
        }
        if set_carry {
            self.set_flag(CARRY)
        } else {
            self.clear_flag(CARRY)
        }
        let write_value = self.write_byte_mem(addr, result);
        read_value + write_value + 3
    }

    pub fn rcr_addr_byte(&mut self) -> usize {
        let addr = self.read_arg_word();
        let (value, read_value) = self.read_byte_mem(addr);
        let set_carry = is_first_bit_set_byte(value);
        let result = value.shr(1);
        if self.check_flag(CARRY) {
            set_last_bit_byte(result);
        }
        if set_carry {
            self.set_flag(CARRY)
        } else {
            self.clear_flag(CARRY)
        }
        let write_value = self.write_byte_mem(addr, result);
        read_value + write_value + 3
    }
}

/// REG, NUM   BYTE
impl VM {
    pub fn asl_reg_num_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_byte() as u32;
        self.bitwise_reg_byte(reg, num, u8::shl)
    }

    pub fn asr_reg_num_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_byte() as u32;
        self.bitwise_reg_byte(reg, num, asr_byte)
    }

    pub fn lsr_reg_num_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_byte() as u32;
        self.bitwise_reg_byte(reg, num, u8::shr)
    }

    pub fn rol_reg_num_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_byte() as u32;
        self.bitwise_reg_byte(reg, num, u8::rotate_left)
    }

    pub fn ror_reg_num_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_byte() as u32;
        self.bitwise_reg_byte(reg, num, u8::rotate_right)
    }

    pub fn rcl_reg_num_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_byte() as u32;
        self.rcl_byte(reg, num)
    }

    pub fn rcr_reg_num_byte(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_byte() as u32;
        self.rcr_byte(reg, num)
    }
}

/// REG, NUM   WORD
impl VM {
    pub fn asl_reg_num_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_word() as u32;
        self.bitwise_reg_word(reg, num, u16::shl)
    }

    pub fn asr_reg_num_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_word() as u32;
        self.bitwise_reg_word(reg, num, asr_word)
    }

    pub fn lsr_reg_num_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_word() as u32;
        self.bitwise_reg_word(reg, num, u16::shr)
    }

    pub fn rol_reg_num_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_word() as u32;
        self.bitwise_reg_word(reg, num, u16::rotate_left)
    }

    pub fn ror_reg_num_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_word() as u32;
        self.bitwise_reg_word(reg, num, u16::rotate_right)
    }

    pub fn rcl_reg_num_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_word() as u32;
        self.rcl_word(reg, num)
    }

    pub fn rcr_reg_num_word(&mut self) -> usize {
        let reg = self.read_arg_register();
        let num = self.read_arg_word() as u32;
        self.rcr_word(reg, num)
    }
}

/// REG, REG   BYTE
impl VM {
    pub fn asl_reg_reg_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 1);
        let (value, read_cost) = self.read_byte_reg(&src, offset);
        let calc_cost = self.bitwise_reg_byte(dst, value as u32, u8::shl);
        self.post_process(&src, 1) + offset_cost + read_cost + calc_cost
    }

    pub fn asr_reg_reg_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 1);
        let (value, read_cost) = self.read_byte_reg(&src, offset);
        let calc_cost = self.bitwise_reg_byte(dst, value as u32, asr_byte);
        self.post_process(&src, 1) + offset_cost + read_cost + calc_cost
    }

    pub fn lsr_reg_reg_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 1);
        let (value, read_cost) = self.read_byte_reg(&src, offset);
        let calc_cost = self.bitwise_reg_byte(dst, value as u32, u8::shr);
        self.post_process(&src, 1) + offset_cost + read_cost + calc_cost
    }

    pub fn rol_reg_reg_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 1);
        let (value, read_cost) = self.read_byte_reg(&src, offset);
        let calc_cost = self.bitwise_reg_byte(dst, value as u32, u8::rotate_left);
        self.post_process(&src, 1) + offset_cost + read_cost + calc_cost
    }

    pub fn ror_reg_reg_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 1);
        let (value, read_cost) = self.read_byte_reg(&src, offset);
        let calc_cost = self.bitwise_reg_byte(dst, value as u32, u8::rotate_right);
        self.post_process(&src, 1) + offset_cost + read_cost + calc_cost
    }

    pub fn rcl_reg_reg_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 1);
        let (value, read_cost) = self.read_byte_reg(&src, offset);
        let calc_cost = self.rcl_byte(dst, value as u32);
        self.post_process(&src, 1) + offset_cost + read_cost + calc_cost
    }

    pub fn rcr_reg_reg_byte(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 1);
        let (value, read_cost) = self.read_byte_reg(&src, offset);
        let calc_cost = self.rcr_byte(dst, value as u32);
        self.post_process(&src, 1) + offset_cost + read_cost + calc_cost
    }
}

/// REG, REG   WORD
impl VM {
    pub fn asl_reg_reg_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 2);
        let (value, read_cost) = self.read_word_reg(&src, offset);
        let calc_cost = self.bitwise_reg_word(dst, value as u32, u16::shl);
        self.post_process(&src, 2) + offset_cost + read_cost + calc_cost
    }

    pub fn asr_reg_reg_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 2);
        let (value, read_cost) = self.read_word_reg(&src, offset);
        let calc_cost = self.bitwise_reg_word(dst, value as u32, asr_word);
        self.post_process(&src, 2) + offset_cost + read_cost + calc_cost
    }

    pub fn lsr_reg_reg_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 2);
        let (value, read_cost) = self.read_word_reg(&src, offset);
        let calc_cost = self.bitwise_reg_word(dst, value as u32, u16::shr);
        self.post_process(&src, 2) + offset_cost + read_cost + calc_cost
    }

    pub fn rol_reg_reg_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 2);
        let (value, read_cost) = self.read_word_reg(&src, offset);
        let calc_cost = self.bitwise_reg_word(dst, value as u32, u16::rotate_left);
        self.post_process(&src, 2) + offset_cost + read_cost + calc_cost
    }

    pub fn ror_reg_reg_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 2);
        let (value, read_cost) = self.read_word_reg(&src, offset);
        let calc_cost = self.bitwise_reg_word(dst, value as u32, u16::rotate_right);
        self.post_process(&src, 2) + offset_cost + read_cost + calc_cost
    }

    pub fn rcl_reg_reg_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 2);
        let (value, read_cost) = self.read_word_reg(&src, offset);
        let calc_cost = self.rcl_word(dst, value as u32);
        self.post_process(&src, 2) + offset_cost + read_cost + calc_cost
    }

    pub fn rcr_reg_reg_word(&mut self) -> usize {
        let dst = self.read_arg_register();
        let src = self.read_arg_register();
        let (offset, offset_cost) = self.pre_process(&src, 2);
        let (value, read_cost) = self.read_word_reg(&src, offset);
        let calc_cost = self.rcr_word(dst, value as u32);
        self.post_process(&src, 2) + offset_cost + read_cost + calc_cost
    }
}

fn asr_byte(value: u8, amount: u32) -> u8 {
    (value as i8).shr(amount) as u8
}

fn asr_word(value: u16, amount: u32) -> u16 {
    (value as i16).shr(amount) as u16
}

#[cfg(test)]
mod test {
    use crate::ops::test::check_cycles;
    use crate::VM;

    #[test]
    fn test_costs() {
        check_cycles(&[0, 0], 2, VM::asl_addr_byte);
        check_cycles(&[0, 0], 2, VM::asr_addr_byte);
        check_cycles(&[0, 0], 2, VM::lsr_addr_byte);
        check_cycles(&[0, 0], 2, VM::rol_addr_byte);
        check_cycles(&[0, 0], 2, VM::ror_addr_byte);
        check_cycles(&[0, 0], 5, VM::rcr_addr_byte);
        check_cycles(&[0, 0], 5, VM::rcl_addr_byte);

        check_cycles(&[0, 0], 4, VM::asl_addr_word);
        check_cycles(&[0, 0], 4, VM::asr_addr_word);
        check_cycles(&[0, 0], 4, VM::lsr_addr_word);
        check_cycles(&[0, 0], 4, VM::rol_addr_word);
        check_cycles(&[0, 0], 4, VM::ror_addr_word);
        check_cycles(&[0, 0], 7, VM::rcr_addr_word);
        check_cycles(&[0, 0], 7, VM::rcl_addr_word);

        check_cycles(&[0, 0], 2, VM::asl_reg_num_byte);
        check_cycles(&[0, 0], 2, VM::asr_reg_num_byte);
        check_cycles(&[0, 0], 2, VM::lsr_reg_num_byte);
        check_cycles(&[0, 0], 2, VM::rol_reg_num_byte);
        check_cycles(&[0, 0], 2, VM::ror_reg_num_byte);
        check_cycles(&[0, 0], 2, VM::rcl_reg_num_byte);
        check_cycles(&[0, 0], 2, VM::rcr_reg_num_byte);

        check_cycles(&[0, 0], 4, VM::asl_reg_num_word);
        check_cycles(&[0, 0], 4, VM::asr_reg_num_word);
        check_cycles(&[0, 0], 4, VM::lsr_reg_num_word);
        check_cycles(&[0, 0], 4, VM::rol_reg_num_word);
        check_cycles(&[0, 0], 4, VM::ror_reg_num_word);
        check_cycles(&[0, 0], 4, VM::rcl_reg_num_word);
        check_cycles(&[0, 0], 4, VM::rcr_reg_num_word);

        check_cycles(&[0, 0], 3, VM::asl_reg_reg_byte);
        check_cycles(&[0, 0], 3, VM::asr_reg_reg_byte);
        check_cycles(&[0, 0], 3, VM::lsr_reg_reg_byte);
        check_cycles(&[0, 0], 3, VM::rol_reg_reg_byte);
        check_cycles(&[0, 0], 3, VM::ror_reg_reg_byte);
        check_cycles(&[0, 0], 3, VM::rcl_reg_reg_byte);
        check_cycles(&[0, 0], 3, VM::rcr_reg_reg_byte);

        check_cycles(&[0, 0], 6, VM::asl_reg_reg_word);
        check_cycles(&[0, 0], 6, VM::asr_reg_reg_word);
        check_cycles(&[0, 0], 6, VM::lsr_reg_reg_word);
        check_cycles(&[0, 0], 6, VM::rol_reg_reg_word);
        check_cycles(&[0, 0], 6, VM::ror_reg_reg_word);
        check_cycles(&[0, 0], 6, VM::rcl_reg_reg_word);
        check_cycles(&[0, 0], 6, VM::rcr_reg_reg_word);
    }
}
