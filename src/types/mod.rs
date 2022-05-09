pub mod bitlogic;
pub mod impls;
pub mod math;
pub mod shifting;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Byte(pub u8);
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Word(pub u16);
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Address(pub u16);

impl Byte {
    pub const ZERO: Byte = Byte(0);
    pub const ONE: Byte = Byte(1);

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl Address {
    pub const ZERO: Address = Address(0);
    pub const ONE: Address = Address(1);
}

impl Word {
    pub const ZERO: Word = Word(0);
    pub const ONE: Word = Word(1);

    pub fn to_address(self) -> Address {
        Address(self.0)
    }

    pub fn to_be_bytes(self) -> [Byte; 2] {
        let bytes = self.0.to_be_bytes();
        [Byte(bytes[0]), Byte(bytes[1])]
    }
}

pub trait HasValue<T> {
    fn value(&self) -> T;
}

impl HasValue<u8> for Byte {
    fn value(&self) -> u8 {
        self.0
    }
}

impl HasValue<u16> for Word {
    fn value(&self) -> u16 {
        self.0
    }
}

impl HasValue<i8> for Byte {
    fn value(&self) -> i8 {
        self.0 as i8
    }
}

impl HasValue<i16> for Word {
    fn value(&self) -> i16 {
        self.0 as i16
    }
}
