use crate::types::{Byte, HasValue, Word};
use std::ops::{BitAnd, BitOr, BitXor, Not};

pub trait BitLogic<I>
where
    Self: Sized + HasValue<I>,
    I: Not<Output = I> + BitAnd<Output = I> + BitXor<Output = I> + BitOr<Output = I> + Copy,
{
    fn new(value: I) -> Self;

    fn not(&self) -> Self {
        Self::new(Self::value(self).not())
    }

    fn xor(&self, other: Self) -> Self {
        Self::new(Self::value(self).bitxor(Self::value(&other)))
    }

    fn and(&self, other: Self) -> Self {
        Self::new(Self::value(self).bitand(Self::value(&other)))
    }

    fn or(&self, other: Self) -> Self {
        Self::new(Self::value(self).bitor(Self::value(&other)))
    }
}

impl BitLogic<u8> for Byte {
    fn new(value: u8) -> Byte {
        Byte(value)
    }
}

impl BitLogic<u16> for Word {
    fn new(value: u16) -> Word {
        Word(value)
    }
}
