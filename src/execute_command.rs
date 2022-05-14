use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::ops;
use maikor_language::ops::{MAY_JMP_OPS, MUST_JMP_OPS};

#[rustfmt::skip]
impl VM {
    /// Execute op with params
    /// Returns true if op has adjusted PC (and so VM shouldn't automatically advance)
    pub fn execute(&mut self, op: u8) -> Result<bool, String> {
        // println!("Executing from {}", self.pc);
        self.op_executed += 1;
        let jumped = match op {
            ops::INC_REG_WORD => {
                let reg = self.register()?;
                self.inc_reg_word(reg);
                false
            }
            ops::JL_ADDR => {
                let addr = self.address();
                self.jl_addr(addr)
            }
            ops::CMP_REG_NUM_WORD => {
                let reg = self.register()?;
                let num = self.word();
                self.cmp_reg_num_word(reg, num);
                false
            }
            ops::HALT => {
                self.halted = true;
                false
            }
            _ => panic!("unsupported: {:02X}", op)
        };
        Ok(jumped)
    }
    
    fn execute_must_jmp_op(&mut self, op: u8, mut params: ArgParams) -> Result<bool, String> {
        match op {
            // ops::RET => {},
            // ops::RETI => {},
            // ops::CALL_ADDR => {},
            // ops::CALL_REG => {},
            ops::JRF_BYTE => self.jrf_num(params.byte()),
            ops::JRB_BYTE => self.jrb_num(params.byte()),
            ops::JMP_REG => self.jmp_reg(params.register()?),
            ops::JMP_ADDR => self.jmp_addr(params.address()),
            _ => return Err(format!("OP in MUST_JMP list but not supported by VM: {:02X}", op)),
        }
        Ok(true)
    }
    
    fn execute_may_jmp_op(&mut self, op: u8, mut params: ArgParams) -> Result<bool, String> {
        let jumped = match op {
            ops::JE_REG => self.je_reg(params.register()?),
            ops::JE_ADDR => self.je_addr(params.address()),
            ops::JNE_REG => self.jne_reg(params.register()?),
            ops::JNE_ADDR => self.jne_addr(params.address()),
            ops::JL_REG => self.jl_reg(params.register()?),
            ops::JL_ADDR => self.jl_addr(params.address()),
            ops::JG_REG => self.jg_reg(params.register()?),
            ops::JG_ADDR => self.jg_addr(params.address()),
            ops::JLE_REG => self.jle_reg(params.register()?),
            ops::JLE_ADDR => self.jle_addr(params.address()),
            ops::JGE_REG => self.jge_reg(params.register()?),
            ops::JGE_ADDR => self.jge_addr(params.address()),
            ops::JBS_REG_REG => self.jbs_reg_reg(params.register()?, params.register()?),
            ops::JBS_REG_NUM => self.jbs_reg_num(params.register()?, params.byte()),
            ops::JBS_ADDR_REG => self.jbs_addr_reg(params.address(), params.register()?),
            ops::JBS_ADDR_NUM => self.jbs_addr_num(params.address(), params.byte()),
            ops::JBC_REG_REG => self.jbc_reg_reg(params.register()?, params.register()?),
            ops::JBC_REG_NUM => self.jbc_reg_num(params.register()?, params.byte()),
            ops::JBC_ADDR_REG => self.jbc_addr_reg(params.address(), params.register()?),
            ops::JBC_ADDR_NUM => self.jbc_addr_num(params.address(), params.byte()),
            _ =>return Err(format!("OP in MAY_JMP list but not supported by VM: {:02X}", op)),
        };
        Ok(jumped)
    }
    
    fn next_byte(&mut self) -> u8 {
        let byte = self.memory[self.arg_ptr];
        self.arg_ptr += 1;
        byte
    }
    
    fn byte(&mut self) -> Byte {
        Byte(self.next_byte())
    }
    
    fn word(&mut self) -> Word {
        Word(u16::from_be_bytes([self.next_byte(), self.next_byte()]))
    }
    
    fn register(&mut self) -> Result<Register, String> {
        Register::from(self.next_byte())
    }
    
    fn address(&mut self) -> Address {
        Address(u16::from_be_bytes([self.next_byte(), self.next_byte()]))
    }
}

#[derive(Debug)]
pub struct ArgParams {
    bytes: [u8; 6],
    len: usize,
    ptr: usize,
}

impl ArgParams {
    pub fn new(bytes: &[u8], start: usize, count: usize) -> Self {
        let mut params = Self {
            bytes: [0; 6],
            len: count,
            ptr: 0,
        };
        for i in 0..count {
            params.bytes[i] = bytes[start + i];
        }
        params
    }

    pub fn next_byte(&mut self) -> u8 {
        let byte = self.bytes[self.ptr];
        self.ptr += 1;
        byte
    }
}

trait ReadParams {
    fn byte(&mut self) -> Byte;
    fn word(&mut self) -> Word;
    fn register(&mut self) -> Result<Register, String>;
    fn address(&mut self) -> Address;
}

impl ReadParams for ArgParams {
    fn byte(&mut self) -> Byte {
        Byte(self.next_byte())
    }

    fn word(&mut self) -> Word {
        Word(u16::from_be_bytes([self.next_byte(), self.next_byte()]))
    }

    fn register(&mut self) -> Result<Register, String> {
        Register::from(self.next_byte())
    }

    fn address(&mut self) -> Address {
        Address(u16::from_be_bytes([self.next_byte(), self.next_byte()]))
    }
}
