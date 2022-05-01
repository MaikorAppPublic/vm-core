use crate::constants::ops;
use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;
use std::collections::VecDeque;

#[rustfmt::skip]
impl VM {
    pub fn execute(&mut self, op: u8, mut params: VecDeque<u8>) {
        match op {
            ops::NOP => {},
            ops::HALT => {},
            ops::RETURN => {},
            ops::RETURN_FROM_INTERRUPT => {},
            ops::INC_REG_BYTE => self.inc_reg_byte(params.register()),
            ops::INC_REG_WORD => self.inc_reg_word(params.register()),
            ops::INC_ADDR_BYTE => self.inc_mem_byte(params.address()),
            ops::INC_ADDR_WORD => self.inc_mem_word(params.address()),
            ops::ADD_REG_REG_BYTE => self.add_reg_reg_byte(params.register(), params.register()),
            ops::ADD_REG_REG_WORD => self.add_reg_reg_word(params.register(), params.register()),
            ops::ADD_REG_NUM_BYTE => self.add_reg_num_byte(params.register(), params.byte()),
            ops::ADD_REG_NUM_WORD => self.add_reg_num_word(params.register(), params.word()),
            ops::ADD_REG_ADDR_BYTE => self.add_reg_mem_byte(params.register(), params.address()),
            ops::ADD_REG_ADDR_WORD => self.add_reg_mem_word(params.register(), params.address()),
            ops::ADD_ADDR_REG_BYTE => self.add_mem_reg_byte(params.address(), params.register()),
            ops::ADD_ADDR_ADDR_BYTE => self.add_mem_mem_byte(params.address(), params.address()),
            ops::ADD_ADDR_NUM_BYTE => self.add_mem_num_byte(params.address(), params.byte()),
            ops::ADD_ADDR_REG_WORD => self.add_mem_reg_word(params.address(), params.register()),
            ops::ADD_ADDR_ADDR_WORD => self.add_mem_mem_word(params.address(), params.address()),
            ops::ADD_ADDR_NUM_WORD => self.add_mem_num_word(params.address(), params.word()),
            ops::CPY_REG_REG_BYTE => self.cpy_reg_reg_byte(params.register(), params.register()),
            ops::CPY_REG_NUM_BYTE => self.cpy_reg_num_byte(params.register(), params.byte()),
            ops::CPY_REG_REG_WORD => self.cpy_reg_reg_word(params.register(), params.register()),
            ops::CPY_REG_ADDR_BYTE => self.cpy_reg_mem_byte(params.register(), params.address()),
            ops::CPY_REG_ADDR_WORD => self.cpy_reg_mem_word(params.register(), params.address()),
            ops::CPY_ADDR_REG_BYTE => self.cpy_mem_reg_byte(params.address(), params.register()),
            ops::CPY_ADDR_ADDR_BYTE => self.cpy_mem_mem_byte(params.address(), params.address()),
            ops::CPY_ADDR_NUM_BYTE => self.cpy_mem_num_byte(params.address(), params.byte()),
            ops::CPY_ADDR_REG_WORD => self.cpy_mem_reg_word(params.address(), params.register()),
            ops::CPY_ADDR_ADDR_WORD => self.cpy_mem_mem_word(params.address(), params.address()),
            ops::CPY_ADDR_NUM_WORD => self.cpy_mem_num_word(params.address(), params.word()),
            ops::CPY_REG_NUM_WORD => self.cpy_reg_num_word(params.register(), params.word()),
            ops::SUB_REG_REG_BYTE => self.sub_reg_reg_byte(params.register(), params.register()),
            ops::SUB_REG_REG_WORD => self.sub_reg_reg_word(params.register(), params.register()),
            ops::SUB_REG_NUM_BYTE => self.sub_reg_num_byte(params.register(), params.byte()),
            ops::SUB_REG_NUM_WORD => self.sub_reg_num_word(params.register(), params.word()),
            ops::SUB_REG_ADDR_BYTE => self.sub_reg_mem_byte(params.register(), params.address()),
            ops::SUB_REG_ADDR_WORD => self.sub_reg_mem_word(params.register(), params.address()),
            ops::SUB_ADDR_REG_BYTE => self.sub_mem_reg_byte(params.address(), params.register()),
            ops::SUB_ADDR_ADDR_BYTE => self.sub_mem_mem_byte(params.address(), params.address()),
            ops::SUB_ADDR_NUM_BYTE => self.sub_mem_num_byte(params.address(), params.byte()),
            ops::SUB_ADDR_REG_WORD => self.sub_mem_reg_word(params.address(), params.register()),
            ops::SUB_ADDR_ADDR_WORD => self.sub_mem_mem_word(params.address(), params.address()),
            ops::SUB_ADDR_NUM_WORD => self.sub_mem_num_word(params.address(), params.word()),
            ops::NOT_REG_BYTE => self.not_reg_byte(params.register()),
            ops::NOT_REG_WORD => self.not_reg_word(params.register()),
            ops::AND_REG_REG_BYTE => self.and_reg_reg_byte(params.register(), params.register()),
            ops::AND_REG_REG_WORD => self.and_reg_reg_word(params.register(), params.register()),
            ops::AND_REG_NUM_BYTE => self.and_reg_num_byte(params.register(), params.byte()),
            ops::AND_REG_NUM_WORD => self.and_reg_num_word(params.register(), params.word()),
            ops::XOR_REG_REG_BYTE => self.xor_reg_reg_byte(params.register(), params.register()),
            ops::XOR_REG_REG_WORD => self.xor_reg_reg_word(params.register(), params.register()),
            ops::XOR_REG_NUM_BYTE => self.xor_reg_num_byte(params.register(), params.byte()),
            ops::XOR_REG_NUM_WORD => self.xor_reg_num_word(params.register(), params.word()),
            ops::OR_REG_REG_BYTE => self.or_reg_reg_byte(params.register(), params.register()),
            ops::OR_REG_REG_WORD => self.or_reg_reg_word(params.register(), params.register()),
            ops::OR_REG_NUM_BYTE => self.or_reg_num_byte(params.register(), params.byte()),
            ops::OR_REG_NUM_WORD => self.or_reg_num_word(params.register(), params.word()),
            ops::SWAP_REG_REG_BYTE => self.swap_reg_reg_byte(params.register(), params.register()),
            ops::SWAP_REG_REG_WORD => self.swap_reg_reg_word(params.register(), params.register()),
            _ => self.fail(format!("Unknown op: {:02X}", op)),
        }
    }
}

trait ReadParams {
    fn byte(&mut self) -> Byte;
    fn word(&mut self) -> Word;
    fn register(&mut self) -> Register;
    fn address(&mut self) -> Address;
}

impl ReadParams for VecDeque<u8> {
    fn byte(&mut self) -> Byte {
        Byte(self.pop_front().unwrap())
    }

    fn word(&mut self) -> Word {
        Word(u16::from_be_bytes([
            self.pop_front().unwrap(),
            self.pop_front().unwrap(),
        ]))
    }

    fn register(&mut self) -> Register {
        Register::from(self.pop_front().unwrap())
    }

    fn address(&mut self) -> Address {
        Address(u16::from_be_bytes([
            self.pop_front().unwrap(),
            self.pop_front().unwrap(),
        ]))
    }
}
