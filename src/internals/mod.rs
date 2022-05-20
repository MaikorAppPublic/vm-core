use crate::{address, VM};

pub mod flags;
pub mod mem_access;
pub mod reg_access;

impl VM {
    #[inline(always)]
    pub fn get_sp(&self) -> u16 {
        let (value, _) = self.read_word_mem(address::SP as u16);
        value
    }

    #[inline(always)]
    pub fn get_fp(&self) -> u16 {
        let (value, _) = self.read_word_mem(address::FP as u16);
        value
    }
}
