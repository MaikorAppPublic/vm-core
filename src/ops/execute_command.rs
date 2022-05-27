use crate::ops::mathc::{u16_carrying_add, u16_carrying_sub, u8_carrying_add, u8_carrying_sub};
use crate::ops::maths::{u16_divs, u16_muls, u8_divs, u8_muls};
use crate::VM;
use maikor_platform::ops;
use std::ops::{BitAnd, BitOr, BitXor};

#[rustfmt::skip]
impl VM {
    /// Execute op with params
    /// Returns true if op has adjusted PC (and so VM shouldn't automatically advance)
    ///     and cycles
    pub fn execute(&mut self, op: u8) -> Result<(bool, usize), String> {
        Ok(match op {
            ops::NOP => (false, 3),
            ops::HALT => {
                self.halted = true;
                (true, 0)
            }
            ops::EHALT => {
                self.halted = true;
                self.error = Some(String::from("Halted by program"));
                (true, 0)
            }
            ops::RET => (true, self.ret()),
            ops::RETI => {
                self.return_from_interrupt();
                (true, 4)
            }
            ops::CALL_ADDR => {
                (true, self.call_addr())
            }
            ops::CALL_REG => {
                (true, self.call_reg())
            }
            ops::INC_REG_BYTE => (false, self.inc_reg_byte()),
            ops::DEC_REG_BYTE => (false, self.dec_reg_byte()),
            ops::INC_REG_WORD => (false, self.inc_reg_word()),
            ops::DEC_REG_WORD => (false, self.dec_reg_word()),
            ops::INC_ADDR_BYTE => (false, self.inc_addr_byte()),
            ops::DEC_ADDR_BYTE => (false, self.dec_addr_byte()),
            ops::INC_ADDR_WORD => (false, self.inc_addr_word()),
            ops::DEC_ADDR_WORD => (false, self.dec_addr_word()),
            ops::SWAP_REG_REG_BYTE => (false, self.swap_byte()),
            ops::SWAP_REG_REG_WORD => (false, self.swap_word()),
            ops::ADD_REG_NUM_BYTE => (false, self.math_reg_num_byte(u8::overflowing_add)),
            ops::ADD_REG_REG_BYTE => (false, self.math_reg_reg_byte(u8::overflowing_add)),
            ops::ADD_REG_ADDR_BYTE => (false, self.math_reg_addr_byte(u8::overflowing_add)),
            ops::ADD_REG_NUM_WORD => (false, self.math_reg_num_word(u16::overflowing_add)),
            ops::ADD_REG_REG_WORD => (false, self.math_reg_reg_word(u16::overflowing_add)),
            ops::ADD_REG_ADDR_WORD => (false, self.math_reg_addr_word(u16::overflowing_add)),
            ops::ADD_ADDR_NUM_BYTE => (false, self.math_addr_num_byte(u8::overflowing_add)),
            ops::ADD_ADDR_REG_BYTE => (false, self.math_addr_reg_byte(u8::overflowing_add)),
            ops::ADD_ADDR_ADDR_BYTE => (false, self.math_addr_addr_byte(u8::overflowing_add)),
            ops::ADD_ADDR_NUM_WORD => (false, self.math_addr_num_word(u16::overflowing_add)),
            ops::ADD_ADDR_REG_WORD => (false, self.math_addr_reg_word(u16::overflowing_add)),
            ops::ADD_ADDR_ADDR_WORD => (false, self.math_addr_addr_word(u16::overflowing_add)),
            ops::SUB_REG_NUM_BYTE => (false, self.math_reg_num_byte(u8::overflowing_sub)),
            ops::SUB_REG_REG_BYTE => (false, self.math_reg_reg_byte(u8::overflowing_sub)),
            ops::SUB_REG_ADDR_BYTE => (false, self.math_reg_addr_byte(u8::overflowing_sub)),
            ops::SUB_REG_NUM_WORD => (false, self.math_reg_num_word(u16::overflowing_sub)),
            ops::SUB_REG_REG_WORD => (false, self.math_reg_reg_word(u16::overflowing_sub)),
            ops::SUB_REG_ADDR_WORD => (false, self.math_reg_addr_word(u16::overflowing_sub)),
            ops::SUB_ADDR_NUM_BYTE => (false, self.math_addr_num_byte(u8::overflowing_sub)),
            ops::SUB_ADDR_REG_BYTE => (false, self.math_addr_reg_byte(u8::overflowing_sub)),
            ops::SUB_ADDR_ADDR_BYTE => (false, self.math_addr_addr_byte(u8::overflowing_sub)),
            ops::SUB_ADDR_NUM_WORD => (false, self.math_addr_num_word(u16::overflowing_sub)),
            ops::SUB_ADDR_REG_WORD => (false, self.math_addr_reg_word(u16::overflowing_sub)),
            ops::SUB_ADDR_ADDR_WORD => (false, self.math_addr_addr_word(u16::overflowing_sub)),
            ops::MUL_REG_NUM_BYTE => (false, self.math_reg_num_byte(u8::overflowing_mul)),
            ops::MUL_REG_REG_BYTE => (false, self.math_reg_reg_byte(u8::overflowing_mul)),
            ops::MUL_REG_ADDR_BYTE => (false, self.math_reg_addr_byte(u8::overflowing_mul)),
            ops::MUL_REG_NUM_WORD => (false, self.math_reg_num_word(u16::overflowing_mul)),
            ops::MUL_REG_REG_WORD => (false, self.math_reg_reg_word(u16::overflowing_mul)),
            ops::MUL_REG_ADDR_WORD => (false, self.math_reg_addr_word(u16::overflowing_mul)),
            ops::MUL_ADDR_NUM_BYTE => (false, self.math_addr_num_byte(u8::overflowing_mul)),
            ops::MUL_ADDR_REG_BYTE => (false, self.math_addr_reg_byte(u8::overflowing_mul)),
            ops::MUL_ADDR_ADDR_BYTE => (false, self.math_addr_addr_byte(u8::overflowing_mul)),
            ops::MUL_ADDR_NUM_WORD => (false, self.math_addr_num_word(u16::overflowing_mul)),
            ops::MUL_ADDR_REG_WORD => (false, self.math_addr_reg_word(u16::overflowing_mul)),
            ops::MUL_ADDR_ADDR_WORD => (false, self.math_addr_addr_word(u16::overflowing_mul)),
            ops::CPY_REG_NUM_BYTE => (false, self.cpy_reg_num_byte()),
            ops::CPY_REG_REG_BYTE => (false, self.cpy_reg_reg_byte()),
            ops::CPY_REG_ADDR_BYTE => (false, self.cpy_reg_addr_byte()),
            ops::CPY_REG_NUM_WORD => (false, self.cpy_reg_num_word()),
            ops::CPY_REG_REG_WORD => (false, self.cpy_reg_reg_word()),
            ops::CPY_REG_ADDR_WORD => (false, self.cpy_reg_addr_word()),
            ops::CPY_ADDR_NUM_BYTE => (false, self.cpy_addr_num_byte()),
            ops::CPY_ADDR_REG_BYTE => (false, self.cpy_addr_reg_byte()),
            ops::CPY_ADDR_ADDR_BYTE => (false, self.cpy_addr_addr_byte()),
            ops::CPY_ADDR_NUM_WORD => (false, self.cpy_addr_num_word()),
            ops::CPY_ADDR_REG_WORD => (false, self.cpy_addr_reg_word()),
            ops::CPY_ADDR_ADDR_WORD => (false, self.cpy_addr_addr_word()),
            ops::JMP_ADDR => self.jmp_addr(),
            ops::JE_ADDR => self.je_addr(),
            ops::JNE_ADDR => self.jne_addr(),
            ops::JG_ADDR => self.jg_addr(),
            ops::JL_ADDR => self.jl_addr(),
            ops::JGE_ADDR => self.jge_addr(),
            ops::JLE_ADDR => self.jle_addr(),
            ops::JMP_REG => self.jmp_reg(),
            ops::JE_REG => self.je_reg(),
            ops::JNE_REG => self.jne_reg(),
            ops::JG_REG => self.jg_reg(),
            ops::JL_REG => self.jl_reg(),
            ops::JGE_REG => self.jge_reg(),
            ops::JLE_REG => self.jle_reg(),
            ops::JRF_BYTE => {
                self.pc = self.pc.wrapping_add(self.read_arg_byte() as u16);
                (true, 1)
            }
            ops::JRB_BYTE => {
                self.pc = self.pc.wrapping_sub(self.read_arg_byte() as u16);
                (true, 1)
            }
            ops::CMP_REG_NUM_BYTE => (false, self.cmp_reg_num_byte(false)),
            ops::CMPS_REG_NUM_BYTE => (false, self.cmp_reg_num_byte(true)),
            ops::CMP_REG_NUM_WORD => (false, self.cmp_reg_num_word(false)),
            ops::CMPS_REG_NUM_WORD => (false, self.cmp_reg_num_word(true)),
            ops::MEM_CPY_ADDR_ADDR_BYTE => (false, self.mem_addr_addr_byte(VM::copy_mem)),
            ops::MEM_CPY_ADDR_ADDR_REG => (false, self.mem_addr_addr_reg(VM::copy_mem)),
            ops::MEM_CPY_ADDR_REG_BYTE => (false, self.mem_addr_reg_byte(VM::copy_mem)),
            ops::MEM_CPY_ADDR_REG_REG => (false, self.mem_addr_reg_reg(VM::copy_mem)),
            ops::MEM_CPY_REG_ADDR_BYTE => (false, self.mem_reg_addr_byte(VM::copy_mem)),
            ops::MEM_CPY_REG_ADDR_REG => (false, self.mem_reg_addr_reg(VM::copy_mem)),
            ops::MEM_CPY_REG_REG_BYTE => (false, self.mem_reg_reg_byte(VM::copy_mem)),
            ops::MEM_CPY_REG_REG_REG => (false, self.mem_reg_reg_reg(VM::copy_mem)),
            ops::NOT_REG_BYTE => (false, self.not_reg_byte()),
            ops::NOT_REG_WORD => (false, self.not_reg_word()),
            ops::OR_REG_REG_BYTE => (false, self.bl_reg_reg_byte(u8::bitor)),
            ops::OR_REG_REG_WORD => (false, self.bl_reg_reg_word(u16::bitor)),
            ops::OR_REG_NUM_BYTE => (false, self.bl_reg_num_byte(u8::bitor)),
            ops::OR_REG_NUM_WORD => (false, self.bl_reg_num_word(u16::bitor)),
            ops::XOR_REG_REG_BYTE => (false, self.bl_reg_reg_byte(u8::bitxor)),
            ops::XOR_REG_REG_WORD => (false, self.bl_reg_reg_word(u16::bitxor)),
            ops::XOR_REG_NUM_BYTE => (false, self.bl_reg_num_byte(u8::bitxor)),
            ops::XOR_REG_NUM_WORD => (false, self.bl_reg_num_word(u16::bitxor)),
            ops::AND_REG_REG_BYTE => (false, self.bl_reg_reg_byte(u8::bitand)),
            ops::AND_REG_REG_WORD => (false, self.bl_reg_reg_word(u16::bitand)),
            ops::AND_REG_NUM_BYTE => (false, self.bl_reg_num_byte(u8::bitand)),
            ops::AND_REG_NUM_WORD => (false, self.bl_reg_num_word(u16::bitand)),
            ops::PUSH_REG_BYTE => (false, self.push_reg_byte()),
            ops::PUSH_REG_WORD => (false, self.push_reg_word()),
            ops::PUSH_NUM_BYTE => (false, self.push_num_byte()),
            ops::PUSH_NUM_WORD => (false, self.push_num_word()),
            ops::POP_REG_BYTE => (false, self.pop_reg_byte()),
            ops::POP_REG_WORD => (false, self.pop_reg_word()),
            ops::MSWP_ADDR_ADDR_BYTE => (false, self.mem_addr_addr_byte(VM::swap_mem)),
            ops::MSWP_ADDR_ADDR_REG => (false, self.mem_addr_addr_reg(VM::swap_mem)),
            ops::MSWP_ADDR_REG_BYTE => (false, self.mem_addr_reg_byte(VM::swap_mem)),
            ops::MSWP_ADDR_REG_REG => (false, self.mem_addr_reg_reg(VM::swap_mem)),
            ops::MSWP_REG_ADDR_BYTE => (false, self.mem_reg_addr_byte(VM::swap_mem)),
            ops::MSWP_REG_ADDR_REG => (false, self.mem_reg_addr_reg(VM::swap_mem)),
            ops::MSWP_REG_REG_BYTE => (false, self.mem_reg_reg_byte(VM::swap_mem)),
            ops::MSWP_REG_REG_REG => (false, self.mem_reg_reg_reg(VM::swap_mem)),
            ops::JBC_REG_REG => self.jb_reg_reg(false),
            ops::JBC_ADDR_REG => self.jb_addr_reg(false),
            ops::JBC_REG_NUM => self.jb_reg_num(false),
            ops::JBC_ADDR_NUM => self.jb_addr_num(false),
            ops::JBS_REG_REG => self.jb_reg_reg(true),
            ops::JBS_ADDR_REG => self.jb_addr_reg(true),
            ops::JBS_REG_NUM => self.jb_reg_num(true),
            ops::JBS_ADDR_NUM => self.jb_addr_num(true),
            ops::ADDC_REG_NUM_BYTE => (false, self.mathc_reg_num_byte(u8_carrying_add)),
            ops::ADDC_REG_REG_BYTE => (false, self.mathc_reg_reg_byte(u8_carrying_add)),
            ops::ADDC_REG_ADDR_BYTE => (false, self.mathc_reg_addr_byte(u8_carrying_add)),
            ops::ADDC_REG_NUM_WORD => (false, self.mathc_reg_num_word(u16_carrying_add)),
            ops::ADDC_REG_REG_WORD => (false, self.mathc_reg_reg_word(u16_carrying_add)),
            ops::ADDC_REG_ADDR_WORD => (false, self.mathc_reg_addr_word(u16_carrying_add)),
            ops::ADDC_ADDR_NUM_BYTE => (false, self.mathc_addr_num_byte(u8_carrying_add)),
            ops::ADDC_ADDR_REG_BYTE => (false, self.mathc_addr_reg_byte(u8_carrying_add)),
            ops::ADDC_ADDR_ADDR_BYTE => (false, self.mathc_addr_addr_byte(u8_carrying_add)),
            ops::ADDC_ADDR_NUM_WORD => (false, self.mathc_addr_num_word(u16_carrying_add)),
            ops::ADDC_ADDR_REG_WORD => (false, self.mathc_addr_reg_word(u16_carrying_add)),
            ops::ADDC_ADDR_ADDR_WORD => (false, self.mathc_addr_addr_word(u16_carrying_add)),
            ops::SUBC_REG_NUM_BYTE => (false, self.mathc_reg_num_byte(u8_carrying_sub)),
            ops::SUBC_REG_REG_BYTE => (false, self.mathc_reg_reg_byte(u8_carrying_sub)),
            ops::SUBC_REG_ADDR_BYTE => (false, self.mathc_reg_addr_byte(u8_carrying_sub)),
            ops::SUBC_REG_NUM_WORD => (false, self.mathc_reg_num_word(u16_carrying_sub)),
            ops::SUBC_REG_REG_WORD => (false, self.mathc_reg_reg_word(u16_carrying_sub)),
            ops::SUBC_REG_ADDR_WORD => (false, self.mathc_reg_addr_word(u16_carrying_sub)),
            ops::SUBC_ADDR_NUM_BYTE => (false, self.mathc_addr_num_byte(u8_carrying_sub)),
            ops::SUBC_ADDR_REG_BYTE => (false, self.mathc_addr_reg_byte(u8_carrying_sub)),
            ops::SUBC_ADDR_ADDR_BYTE => (false, self.mathc_addr_addr_byte(u8_carrying_sub)),
            ops::SUBC_ADDR_NUM_WORD => (false, self.mathc_addr_num_word(u16_carrying_sub)),
            ops::SUBC_ADDR_REG_WORD => (false, self.mathc_addr_reg_word(u16_carrying_sub)),
            ops::SUBC_ADDR_ADDR_WORD => (false, self.mathc_addr_addr_word(u16_carrying_sub)),
            ops::MULS_REG_NUM_BYTE => (false, self.maths_reg_num_byte(u8_muls)),
            ops::MULS_REG_REG_BYTE => (false, self.maths_reg_reg_byte(u8_muls)),
            ops::MULS_REG_ADDR_BYTE => (false, self.maths_reg_addr_byte(u8_muls)),
            ops::MULS_REG_NUM_WORD => (false, self.maths_reg_num_word(u16_muls)),
            ops::MULS_REG_REG_WORD => (false, self.maths_reg_reg_word(u16_muls)),
            ops::MULS_REG_ADDR_WORD => (false, self.maths_reg_addr_word(u16_muls)),
            ops::MULS_ADDR_NUM_BYTE => (false, self.maths_addr_num_byte(u8_muls)),
            ops::MULS_ADDR_REG_BYTE => (false, self.maths_addr_reg_byte(u8_muls)),
            ops::MULS_ADDR_ADDR_BYTE => (false, self.maths_addr_addr_byte(u8_muls)),
            ops::MULS_ADDR_NUM_WORD => (false, self.maths_addr_num_word(u16_muls)),
            ops::MULS_ADDR_REG_WORD => (false, self.maths_addr_reg_word(u16_muls)),
            ops::MULS_ADDR_ADDR_WORD => (false, self.maths_addr_addr_word(u16_muls)),
            ops::DIVS_REG_NUM_BYTE => (false, self.maths_reg_num_byte(u8_divs)),
            ops::DIVS_REG_REG_BYTE => (false, self.maths_reg_reg_byte(u8_divs)),
            ops::DIVS_REG_ADDR_BYTE => (false, self.maths_reg_addr_byte(u8_divs)),
            ops::DIVS_REG_NUM_WORD => (false, self.maths_reg_num_word(u16_divs)),
            ops::DIVS_REG_REG_WORD => (false, self.maths_reg_reg_word(u16_divs)),
            ops::DIVS_REG_ADDR_WORD => (false, self.maths_reg_addr_word(u16_divs)),
            ops::DIVS_ADDR_NUM_BYTE => (false, self.maths_addr_num_byte(u8_divs)),
            ops::DIVS_ADDR_REG_BYTE => (false, self.maths_addr_reg_byte(u8_divs)),
            ops::DIVS_ADDR_ADDR_BYTE => (false, self.maths_addr_addr_byte(u8_divs)),
            ops::DIVS_ADDR_NUM_WORD => (false, self.maths_addr_num_word(u16_divs)),
            ops::DIVS_ADDR_REG_WORD => (false, self.maths_addr_reg_word(u16_divs)),
            ops::DIVS_ADDR_ADDR_WORD => (false, self.maths_addr_addr_word(u16_divs)),
            _ => {
                self.fail(format!("Unsupported op: {:02X}", op));
                (false, 0)
            }
        })
    }
}
