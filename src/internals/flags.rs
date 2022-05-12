use crate::register::offset;
use crate::registers::flags;
use crate::types::{Byte, Word};
use crate::VM;
use std::ops::{BitAnd, BitOr, Not};

impl VM {
    #[inline]
    pub fn check_flag(&self, flag: u8) -> bool {
        self.registers[offset::FLAGS].bitand(flag) == flag
    }

    #[inline]
    pub fn set_flag(&mut self, flag: u8) {
        self.registers[offset::FLAGS] = self.registers[offset::FLAGS].bitor(flag)
    }

    #[inline]
    pub fn clear_flag(&mut self, flag: u8) {
        self.registers[offset::FLAGS] = self.registers[offset::FLAGS].bitand(flag.not())
    }

    #[inline]
    fn update_flag_conditionally(&mut self, flag: u8, condition: bool) {
        if condition {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }
}

pub trait FirstBitSet {
    fn is_first_bit_set(&self) -> bool;
}

impl FirstBitSet for Byte {
    #[inline]
    fn is_first_bit_set(&self) -> bool {
        self.0 & 0b10000000 == 0b10000000
    }
}

impl FirstBitSet for Word {
    #[inline]
    fn is_first_bit_set(&self) -> bool {
        self.0 & 0b1000000000000000 == 0b1000000000000000
    }
}

pub trait Flags<T> {
    fn set_flags(&mut self, value: T);
    fn set_math_flags(&mut self, value: T, set_carry: bool, set_overflow: bool);
    fn set_cmp_flags(&mut self, lhs: T, rhs: T, signed: bool);
}

impl Flags<Byte> for VM {
    fn set_flags(&mut self, value: Byte) {
        self.update_flag_conditionally(flags::ZERO, value == Byte::ZERO);
        self.update_flag_conditionally(flags::SIGNED, value.is_first_bit_set());
        self.clear_flag(flags::GREATER_THAN);
        self.clear_flag(flags::LESS_THAN);
        self.clear_flag(flags::OVERFLOW);
        self.clear_flag(flags::CARRY);
    }

    fn set_math_flags(&mut self, value: Byte, set_carry: bool, set_overflow: bool) {
        self.update_flag_conditionally(flags::ZERO, value == Byte::ZERO);
        self.update_flag_conditionally(flags::SIGNED, value.is_first_bit_set());
        self.clear_flag(flags::GREATER_THAN);
        self.clear_flag(flags::LESS_THAN);
        self.update_flag_conditionally(flags::OVERFLOW, set_overflow);
        self.update_flag_conditionally(flags::CARRY, set_carry);
    }

    fn set_cmp_flags(&mut self, lhs: Byte, rhs: Byte, signed: bool) {
        if signed {
            self.update_flag_conditionally(flags::LESS_THAN, (lhs.0 as i8) < (rhs.0 as i8));
            self.update_flag_conditionally(flags::GREATER_THAN, lhs.0 as i8 > rhs.0 as i8);
        } else {
            self.update_flag_conditionally(flags::LESS_THAN, lhs < rhs);
            self.update_flag_conditionally(flags::GREATER_THAN, lhs > rhs);
        }
        self.update_flag_conditionally(flags::ZERO, lhs == Byte::ZERO);
        self.update_flag_conditionally(flags::SIGNED, lhs.is_first_bit_set());
        self.clear_flag(flags::CARRY);
        self.clear_flag(flags::OVERFLOW);
    }
}

impl Flags<Word> for VM {
    fn set_flags(&mut self, value: Word) {
        self.update_flag_conditionally(flags::ZERO, value == Word::ZERO);
        self.update_flag_conditionally(flags::SIGNED, value.is_first_bit_set());
        self.clear_flag(flags::GREATER_THAN);
        self.clear_flag(flags::LESS_THAN);
        self.clear_flag(flags::OVERFLOW);
        self.clear_flag(flags::CARRY);
    }

    fn set_math_flags(&mut self, value: Word, set_carry: bool, set_overflow: bool) {
        self.update_flag_conditionally(flags::ZERO, value == Word::ZERO);
        self.update_flag_conditionally(flags::SIGNED, value.is_first_bit_set());
        self.clear_flag(flags::GREATER_THAN);
        self.clear_flag(flags::LESS_THAN);
        self.update_flag_conditionally(flags::OVERFLOW, set_overflow);
        self.update_flag_conditionally(flags::CARRY, set_carry);
    }

    fn set_cmp_flags(&mut self, lhs: Word, rhs: Word, signed: bool) {
        if signed {
            self.update_flag_conditionally(flags::LESS_THAN, (lhs.0 as i16) < (rhs.0 as i16));
            self.update_flag_conditionally(flags::GREATER_THAN, lhs.0 as i16 > rhs.0 as i16);
        } else {
            self.update_flag_conditionally(flags::LESS_THAN, lhs < rhs);
            self.update_flag_conditionally(flags::GREATER_THAN, lhs > rhs);
        }
        self.update_flag_conditionally(flags::ZERO, lhs == Word::ZERO);
        self.update_flag_conditionally(flags::SIGNED, lhs.is_first_bit_set());
        self.clear_flag(flags::CARRY);
        self.clear_flag(flags::OVERFLOW);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use maikor_language::ops;
    use maikor_language::registers::flags::*;
    use maikor_language::registers::id;

    #[test]
    fn test_is_first_bit_set_byte() {
        for i in 0..128 {
            assert!(!Byte(i).is_first_bit_set(), "{i}")
        }
        for i in 128..255 {
            assert!(Byte(i).is_first_bit_set(), "{i}")
        }
    }

    #[test]
    fn test_if_first_bit_set_word() {
        for i in 0..32768 {
            assert!(!Word(i).is_first_bit_set(), "{i}")
        }
        for i in 32768..65535 {
            assert!(Word(i).is_first_bit_set(), "{i}")
        }
    }

    #[test]
    fn test_set_cmp_flags() {
        let mut vm = VM::new();
        vm.set_cmp_flags(Byte(5), Byte(10), false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags(Byte(10), Byte(5), false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags(Byte(10), Byte(10), false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags(Byte(0), Byte(5), false);
        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags(Byte(255), Byte(5), false);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
        vm.set_cmp_flags(Byte(255), Byte(5), true);
        assert!(!vm.check_flag(ZERO));
        assert!(vm.check_flag(SIGNED));
        assert!(vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));
    }

    #[test]
    fn test_set_math_flags() {
        let mut vm = VM::new();
        vm.set_math_flags(Byte(64), false, false);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags(Byte(64), true, true);
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(vm.check_flag(CARRY));
        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags(Byte(0), true, true);
        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(vm.check_flag(CARRY));
        assert!(vm.check_flag(OVERFLOW));
        assert!(vm.check_flag(INTERRUPTS));

        vm.set_math_flags(Byte(200), false, true);
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

    #[test]
    fn test_inc() {
        let mut vm = VM::new();

        assert!(!vm.check_flag(CARRY));
        assert!(vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(vm.check_flag(INTERRUPTS));

        vm.execute_op(&[ops::INC_REG_BYTE, id::AL as u8]);

        assert!(!vm.check_flag(CARRY));
        assert!(!vm.check_flag(ZERO));
        assert!(!vm.check_flag(SIGNED));
        assert!(!vm.check_flag(OVERFLOW));
        assert!(!vm.check_flag(LESS_THAN));
        assert!(!vm.check_flag(GREATER_THAN));
        assert!(vm.check_flag(INTERRUPTS));
    }
}
