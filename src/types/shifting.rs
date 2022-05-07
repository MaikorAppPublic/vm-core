use crate::types::Byte;

pub trait Shifting {
    fn asl(&self, amount: usize) -> Self;
    fn asr(&self, amount: usize) -> Self;
    fn lsr(&self, amount: usize) -> Self;
    fn rol(&self, amount: usize) -> Self;
    fn ror(&self, amount: usize) -> Self;
}

impl Shifting for Byte {
    fn asl(&self, amount: usize) -> Self {
        (self.0 << amount).into()
    }

    fn asr(&self, amount: usize) -> Self {
        ((self.0 as i8 >> amount) as u8).into()
    }

    fn lsr(&self, amount: usize) -> Self {
        (self.0 >> amount).into()
    }

    fn rol(&self, amount: usize) -> Self {
        self.0.rotate_left(amount as u32).into()
    }

    fn ror(&self, amount: usize) -> Self {
        self.0.rotate_left(amount as u32).into()
    }
}
