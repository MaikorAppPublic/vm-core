use crate::types::{Byte, HasValue, Word};

pub trait WrappingMath {
    fn wrapping_add(&self, other: Self, carry: bool) -> (Self, bool)
    where
        Self: Sized;
    fn wrapping_sub(&self, other: Self, carry: bool) -> (Self, bool)
    where
        Self: Sized;
    fn wrapping_mul(&self, other: Self) -> (Word, bool);
    fn wrapping_div(&self, other: Self) -> (Self, bool)
    where
        Self: Sized;
    fn wrapping_muls(&self, other: Self) -> (Word, bool);
    fn wrapping_divs(&self, other: Self) -> (Self, bool)
    where
        Self: Sized;
}

impl WrappingMath for Byte {
    fn wrapping_add(&self, other: Byte, carry: bool) -> (Self, bool) {
        let (partial, overflow1) = self.0.overflowing_add(other.0);
        let (result, overflow2) = partial.overflowing_add(carry as u8);
        (result.into(), overflow1 | overflow2)
    }

    fn wrapping_sub(&self, other: Byte, carry: bool) -> (Self, bool) {
        let (partial, overflow1) = self.0.overflowing_sub(other.0);
        let (result, overflow2) = partial.overflowing_sub(carry as u8);
        (result.into(), overflow1 | overflow2)
    }

    fn wrapping_mul(&self, other: Byte) -> (Word, bool) {
        (((self.0 as u16) * (other.0 as u16)).into(), false)
    }

    fn wrapping_div(&self, other: Self) -> (Self, bool) {
        (self.0.wrapping_div(other.0).into(), false)
    }

    fn wrapping_muls(&self, other: Self) -> (Word, bool) {
        ((((self.0 as i16) * (other.0 as i16)) as u16).into(), false)
    }

    fn wrapping_divs(&self, other: Self) -> (Self, bool) {
        (
            ((self.0 as i8).wrapping_div(other.0 as i8) as u8).into(),
            false,
        )
    }
}

impl WrappingMath for Word {
    fn wrapping_add(&self, other: Word, carry: bool) -> (Self, bool) {
        let (partial, overflow1) = self.0.overflowing_add(other.0);
        let (result, overflow2) = partial.overflowing_add(carry as u16);
        (result.into(), overflow1 | overflow2)
    }

    fn wrapping_sub(&self, other: Word, carry: bool) -> (Self, bool) {
        let (partial, overflow1) = self.0.overflowing_sub(other.0);
        let (result, overflow2) = partial.overflowing_sub(carry as u16);
        (result.into(), overflow1 | overflow2)
    }

    fn wrapping_mul(&self, other: Word) -> (Word, bool) {
        let (result, carry) = self.0.overflowing_mul(other.0);
        (result.into(), carry)
    }

    fn wrapping_div(&self, other: Self) -> (Self, bool) {
        (self.0.wrapping_div(other.0).into(), false)
    }

    fn wrapping_muls(&self, other: Self) -> (Word, bool) {
        let (result, carry) = (self.0 as i16).overflowing_mul(other.0 as i16);
        ((result as u16).into(), carry)
    }

    fn wrapping_divs(&self, other: Self) -> (Self, bool) {
        (
            ((self.0 as i16).wrapping_div(other.0 as i16) as u16).into(),
            false,
        )
    }
}
