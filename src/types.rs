use std::fmt::{Display, Formatter};
use std::ops::{Add, BitAnd, BitOr, BitXor, Index, IndexMut, Not, Sub};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Byte(pub u8);
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Word(pub u16);
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Address(pub u16);

pub trait AsByte {
    fn as_byte(&self) -> Byte;
}

pub trait AsWord {
    fn as_word(&self) -> Word;
}

pub trait AsAddress {
    fn as_address(&self) -> Address;
}

impl AsByte for u8 {
    fn as_byte(&self) -> Byte {
        Byte(*self)
    }
}

impl AsWord for u16 {
    fn as_word(&self) -> Word {
        Word(*self)
    }
}

impl AsAddress for u16 {
    #[inline]
    fn as_address(&self) -> Address {
        Address(*self)
    }
}

impl Address {
    pub const ZERO: Address = Address(0);
    pub const ONE: Address = Address(1);
}

pub trait BitLogic<I>
where
    Self: Sized,
    I: Not<Output = I> + BitAnd<Output = I> + BitXor<Output = I> + BitOr<Output = I> + Copy,
{
    fn new(value: I) -> Self;
    fn value(item: &Self) -> I;

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

    fn value(item: &Byte) -> u8 {
        item.0
    }
}

impl BitLogic<u16> for Word {
    fn new(value: u16) -> Word {
        Word(value)
    }

    fn value(item: &Word) -> u16 {
        item.0
    }
}

impl Byte {
    pub const ZERO: Byte = Byte(0);
    pub const ONE: Byte = Byte(1);

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn wrapping_add(&self, byte: Byte) -> Byte {
        self.0.wrapping_add(byte.0).as_byte()
    }
    pub fn wrapping_sub(&self, byte: Byte) -> Byte {
        self.0.wrapping_sub(byte.0).as_byte()
    }
}

impl Word {
    pub const ZERO: Word = Word(0);
    pub const ONE: Word = Word(1);

    pub fn wrapping_add(&self, word: Word) -> Word {
        self.0.wrapping_add(word.0).as_word()
    }

    pub fn wrapping_sub(&self, word: Word) -> Word {
        self.0.wrapping_sub(word.0).as_word()
    }

    pub fn to_address(self) -> Address {
        Address(self.0)
    }

    pub fn to_be_bytes(self) -> [Byte; 2] {
        let bytes = self.0.to_be_bytes();
        [Byte(bytes[0]), Byte(bytes[1])]
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
