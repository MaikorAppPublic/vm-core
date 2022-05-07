use crate::types::{Address, Byte, Word};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Sub};

impl From<u8> for Byte {
    fn from(value: u8) -> Self {
        Byte(value)
    }
}

impl From<i8> for Byte {
    fn from(value: i8) -> Self {
        Byte(value as u8)
    }
}

impl From<i16> for Word {
    fn from(value: i16) -> Self {
        Word(value as u16)
    }
}

impl From<bool> for Byte {
    fn from(value: bool) -> Self {
        if value {
            Byte::ONE
        } else {
            Byte::ZERO
        }
    }
}

impl From<u16> for Word {
    fn from(value: u16) -> Self {
        Word(value)
    }
}

impl From<bool> for Word {
    fn from(value: bool) -> Self {
        if value {
            Word::ONE
        } else {
            Word::ZERO
        }
    }
}

impl From<u16> for Address {
    fn from(value: u16) -> Self {
        Address(value)
    }
}

impl Index<Address> for [u8] {
    type Output = u8;

    fn index(&self, index: Address) -> &Self::Output {
        &self[index.0 as usize]
    }
}

impl IndexMut<Address> for [u8] {
    fn index_mut(&mut self, index: Address) -> &mut Self::Output {
        &mut self[index.0 as usize]
    }
}

impl Display for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Sub for Address {
    type Output = Address;

    fn sub(self, rhs: Self) -> Self::Output {
        Address(self.0 - rhs.0)
    }
}

impl Add for Address {
    type Output = Address;

    fn add(self, rhs: Self) -> Self::Output {
        Address(self.0 + rhs.0)
    }
}

impl Add<usize> for Address {
    type Output = Address;

    fn add(self, rhs: usize) -> Self::Output {
        Address(self.0 + rhs as u16)
    }
}
