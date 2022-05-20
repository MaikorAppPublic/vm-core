use crate::register::offset;
use crate::registers::flags;
use crate::VM;
use std::ops::{BitAnd, BitOr, Not};

impl VM {
    #[inline(always)]
    pub fn check_flag(&self, flag: u8) -> bool {
        self.registers[offset::FLAGS].bitand(flag) == flag
    }

    #[inline(always)]
    pub fn set_flag(&mut self, flag: u8) {
        self.registers[offset::FLAGS] = self.registers[offset::FLAGS].bitor(flag)
    }

    #[inline(always)]
    pub fn clear_flag(&mut self, flag: u8) {
        self.registers[offset::FLAGS] = self.registers[offset::FLAGS].bitand(flag.not())
    }

    #[inline(always)]
    fn update_flag_conditionally(&mut self, flag: u8, condition: bool) {
        if condition {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }
}

#[inline(always)]
fn is_first_bit_set_byte(value: u8) -> bool {
    value & 0b10000000 == 0b10000000
}

#[inline(always)]
fn is_first_bit_set_word(value: u16) -> bool {
    value & 0b1000000000000000 == 0b1000000000000000
}

#[inline(always)]
pub fn has_overflowed_byte(lhs: u8, rhs: u8) -> bool {
    is_first_bit_set_byte(lhs) ^ is_first_bit_set_byte(rhs)
}

#[inline(always)]
pub fn has_overflowed_word(lhs: u16, rhs: u16) -> bool {
    is_first_bit_set_word(lhs) ^ is_first_bit_set_word(rhs)
}

impl VM {
    pub fn set_flags_byte(&mut self, value: u8) {
        self.update_flag_conditionally(flags::ZERO, value == 0);
        self.update_flag_conditionally(flags::SIGNED, is_first_bit_set_byte(value));
        self.clear_flag(flags::GREATER_THAN);
        self.clear_flag(flags::LESS_THAN);
        self.clear_flag(flags::OVERFLOW);
        self.clear_flag(flags::CARRY);
    }

    pub fn set_math_flags_byte(&mut self, value: u8, set_carry: bool, set_overflow: bool) {
        self.update_flag_conditionally(flags::ZERO, value == 0);
        self.update_flag_conditionally(flags::SIGNED, is_first_bit_set_byte(value));
        self.clear_flag(flags::GREATER_THAN);
        self.clear_flag(flags::LESS_THAN);
        self.update_flag_conditionally(flags::OVERFLOW, set_overflow);
        self.update_flag_conditionally(flags::CARRY, set_carry);
    }

    pub fn set_cmp_flags_byte(&mut self, lhs: u8, rhs: u8, signed: bool) {
        if signed {
            self.update_flag_conditionally(flags::LESS_THAN, (lhs as i8) < (rhs as i8));
            self.update_flag_conditionally(flags::GREATER_THAN, lhs as i8 > rhs as i8);
        } else {
            self.update_flag_conditionally(flags::LESS_THAN, lhs < rhs);
            self.update_flag_conditionally(flags::GREATER_THAN, lhs > rhs);
        }
        self.update_flag_conditionally(flags::ZERO, lhs == 0);
        self.update_flag_conditionally(flags::SIGNED, is_first_bit_set_byte(lhs));
        self.clear_flag(flags::CARRY);
        self.clear_flag(flags::OVERFLOW);
    }
}

impl VM {
    pub fn set_flags_word(&mut self, value: u16) {
        self.update_flag_conditionally(flags::ZERO, value == 0);
        self.update_flag_conditionally(flags::SIGNED, is_first_bit_set_word(value));
        self.clear_flag(flags::GREATER_THAN);
        self.clear_flag(flags::LESS_THAN);
        self.clear_flag(flags::OVERFLOW);
        self.clear_flag(flags::CARRY);
    }

    pub fn set_math_flags_word(&mut self, value: u16, set_carry: bool, set_overflow: bool) {
        self.update_flag_conditionally(flags::ZERO, value == 0);
        self.update_flag_conditionally(flags::SIGNED, is_first_bit_set_word(value));
        self.clear_flag(flags::GREATER_THAN);
        self.clear_flag(flags::LESS_THAN);
        self.update_flag_conditionally(flags::OVERFLOW, set_overflow);
        self.update_flag_conditionally(flags::CARRY, set_carry);
    }

    pub fn set_cmp_flags_word(&mut self, lhs: u16, rhs: u16, signed: bool) {
        if signed {
            self.update_flag_conditionally(flags::LESS_THAN, (lhs as i16) < (rhs as i16));
            self.update_flag_conditionally(flags::GREATER_THAN, lhs as i16 > rhs as i16);
        } else {
            self.update_flag_conditionally(flags::LESS_THAN, lhs < rhs);
            self.update_flag_conditionally(flags::GREATER_THAN, lhs > rhs);
        }
        self.update_flag_conditionally(flags::ZERO, lhs == 0);
        self.update_flag_conditionally(flags::SIGNED, is_first_bit_set_word(lhs));
        self.clear_flag(flags::CARRY);
        self.clear_flag(flags::OVERFLOW);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maikor_platform::registers::flags::*;

    #[test]
    fn test_is_first_bit_set_byte() {
        for i in 0..128 {
            assert!(!is_first_bit_set_byte(i), "{i}")
        }
        for i in 128..255 {
            assert!(is_first_bit_set_byte(i), "{i}")
        }
    }

    #[test]
    fn test_if_first_bit_set_word() {
        for i in 0..32768 {
            assert!(!is_first_bit_set_word(i), "{i}")
        }
        for i in 32768..65535 {
            assert!(is_first_bit_set_word(i), "{i}")
        }
    }

    #[test]
    fn test_set_cmp_flags_byte() {
        let mut vm = VM::new();
        vm.set_cmp_flags_byte(5, 10, false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_byte(10, 5, false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_byte(10, 10, false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_byte(0, 5, false);
        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_byte(255, 5, false);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_byte(255, 5, true);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
    }

    #[test]
    fn test_set_math_flags_byte() {
        let mut vm = VM::new();
        vm.set_math_flags_byte(64, false, false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags_byte(64, true, true);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(vm.check_flag(CARRY));
        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags_byte(0, true, true);
        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(vm.check_flag(CARRY));
        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags_byte(200, false, true);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
    }

    #[test]
    fn test_set_cmp_flags_word() {
        let mut vm = VM::new();
        vm.set_cmp_flags_word(5, 10, false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_word(10, 5, false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_word(10, 10, false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_word(0, 5, false);
        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_word(55255, 5, false);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_word(64255, 5, true);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_word(0, 1235, false);
        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_word(41255, 5, false);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags_word(39255, 12215, true);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
    }

    #[test]
    fn test_set_math_flags_word() {
        let mut vm = VM::new();
        vm.set_math_flags_word(64, false, false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags_word(64, true, true);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(vm.check_flag(CARRY));
        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags_word(0, true, true);
        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(vm.check_flag(CARRY));
        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags_word(45200, false, true);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags_word(42000, false, true);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
    }

    #[test]
    fn test_default_flags() {
        let vm = VM::new();
        assert!(!vm.check_flag(CARRY));
        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(vm.check_flag(INTERRUPTS));
    }

    #[test]
    fn test_clear_flag() {
        let mut vm = VM::new();

        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(CARRY));

        vm.clear_flag(ZERO);
        vm.clear_flag(CARRY);

        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(CARRY));
    }

    #[test]
    fn test_set_flag() {
        let mut vm = VM::new();

        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(ZERO));

        vm.set_flag(OVERFLOW);
        vm.set_flag(ZERO);

        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(ZERO));
    }

    #[test]
    fn test_update_flag() {
        let mut vm = VM::new();

        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(vm.check_flag(ZERO));
        assert!(vm.check_flag(INTERRUPTS));

        vm.update_flag_conditionally(GREATER_THAN, true);
        vm.update_flag_conditionally(LESS_THAN, false);
        vm.update_flag_conditionally(ZERO, false);
        vm.update_flag_conditionally(INTERRUPTS, true);

        assert!(vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(INTERRUPTS));
    }
}
