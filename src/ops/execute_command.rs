use crate::VM;
use maikor_platform::ops;

#[rustfmt::skip]
impl VM {
    /// Execute op with params
    /// Returns true if op has adjusted PC (and so VM shouldn't automatically advance)
    ///     and cycles
    pub fn execute(&mut self, op: u8) -> Result<(bool, usize), String> {
        Ok(match op {
            ops::NOP => (false, 1),
            ops::HALT => {
                self.halted = true;
                (true, 1)
            }
            ops::EHALT => {
                self.halted = true;
                self.error = Some(String::from("Halted by program"));
                (true, 1)
            }
            ops::RETI => {
                self.return_from_interrupt();
                (true, 4)
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
            ops::CMP_REG_NUM_BYTE => (false, self.cmp_reg_num_byte(false)),
            ops::CMPS_REG_NUM_BYTE => (false, self.cmp_reg_num_byte(true)),
            ops::CMP_REG_NUM_WORD => (false, self.cmp_reg_num_word(false)),
            ops::CMPS_REG_NUM_WORD => (false, self.cmp_reg_num_word(true)),
            _ => {
                self.fail(format!("Unsupported op: {:02X}", op));
                (false, 0)
            }
        })
    }
}
