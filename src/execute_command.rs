use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::ops;
use maikor_language::ops::{MAY_JMP_OPS, MUST_JMP_OPS};
use std::collections::VecDeque;

#[rustfmt::skip]
impl VM {
    /// Execute op with params
    /// Returns true if op has adjusted PC (and so VM shouldn't automatically advance)
    pub fn execute(&mut self, op: u8, mut params: VecDeque<u8>) -> Result<bool, String> {
        self.op_executed += 1;
        if MUST_JMP_OPS.contains(&op) {
            return self.execute_must_jmp_op(op, params);
        } else if MAY_JMP_OPS.contains(&op) {
            return self.execute_may_jmp_op(op, params);
        }
        match op {
            ops::NOP => self.nop(),
            ops::HALT => {
                self.halted = true
            },
            ops::EHALT => self.fail(format!("Halted by program at ${:04X}", self.pc)),
            ops::INC_REG_BYTE => self.inc_reg_byte(params.register()?),
            ops::INC_REG_WORD => self.inc_reg_word(params.register()?),
            ops::INC_ADDR_BYTE => self.inc_addr_byte(params.address()),
            ops::INC_ADDR_WORD => self.inc_addr_word(params.address()),
            ops::DEC_REG_BYTE => self.dec_reg_byte(params.register()?),
            ops::DEC_REG_WORD => self.dec_reg_word(params.register()?),
            ops::DEC_ADDR_BYTE => self.dec_addr_byte(params.address()),
            ops::DEC_ADDR_WORD => self.dec_addr_word(params.address()),
            ops::ADD_REG_REG_BYTE => self.add_reg_reg_byte(params.register()?, params.register()?),
            ops::ADD_REG_REG_WORD => self.add_reg_reg_word(params.register()?, params.register()?),
            ops::ADD_REG_NUM_BYTE => self.add_reg_num_byte(params.register()?, params.byte()),
            ops::ADD_REG_NUM_WORD => self.add_reg_num_word(params.register()?, params.word()),
            ops::ADD_REG_ADDR_BYTE => self.add_reg_addr_byte(params.register()?, params.address()),
            ops::ADD_REG_ADDR_WORD => self.add_reg_addr_word(params.register()?, params.address()),
            ops::ADD_ADDR_REG_BYTE => self.add_addr_reg_byte(params.address(), params.register()?),
            ops::ADD_ADDR_ADDR_BYTE => self.add_addr_addr_byte(params.address(), params.address()),
            ops::ADD_ADDR_NUM_BYTE => self.add_addr_num_byte(params.address(), params.byte()),
            ops::ADD_ADDR_REG_WORD => self.add_addr_reg_word(params.address(), params.register()?),
            ops::ADD_ADDR_ADDR_WORD => self.add_addr_addr_word(params.address(), params.address()),
            ops::ADD_ADDR_NUM_WORD => self.add_addr_num_word(params.address(), params.word()),
            ops::ADDC_REG_REG_BYTE => self.addc_reg_reg_byte(params.register()?, params.register()?),
            ops::ADDC_REG_REG_WORD => self.addc_reg_reg_word(params.register()?, params.register()?),
            ops::ADDC_REG_NUM_BYTE => self.addc_reg_num_byte(params.register()?, params.byte()),
            ops::ADDC_REG_NUM_WORD => self.addc_reg_num_word(params.register()?, params.word()),
            ops::ADDC_REG_ADDR_BYTE => self.addc_reg_addr_byte(params.register()?, params.address()),
            ops::ADDC_REG_ADDR_WORD => self.addc_reg_addr_word(params.register()?, params.address()),
            ops::ADDC_ADDR_REG_BYTE => self.addc_addr_reg_byte(params.address(), params.register()?),
            ops::ADDC_ADDR_ADDR_BYTE => self.addc_addr_addr_byte(params.address(), params.address()),
            ops::ADDC_ADDR_NUM_BYTE => self.addc_addr_num_byte(params.address(), params.byte()),
            ops::ADDC_ADDR_REG_WORD => self.addc_addr_reg_word(params.address(), params.register()?),
            ops::ADDC_ADDR_ADDR_WORD => self.addc_addr_addr_word(params.address(), params.address()),
            ops::ADDC_ADDR_NUM_WORD => self.addc_addr_num_word(params.address(), params.word()),
            ops::CPY_REG_REG_BYTE => self.cpy_reg_reg_byte(params.register()?, params.register()?),
            ops::CPY_REG_NUM_BYTE => self.cpy_reg_num_byte(params.register()?, params.byte()),
            ops::CPY_REG_REG_WORD => self.cpy_reg_reg_word(params.register()?, params.register()?),
            ops::CPY_REG_ADDR_BYTE => self.cpy_reg_addr_byte(params.register()?, params.address()),
            ops::CPY_REG_ADDR_WORD => self.cpy_reg_addr_word(params.register()?, params.address()),
            ops::CPY_ADDR_REG_BYTE => self.cpy_addr_reg_byte(params.address(), params.register()?),
            ops::CPY_ADDR_ADDR_BYTE => self.cpy_addr_addr_byte(params.address(), params.address()),
            ops::CPY_ADDR_NUM_BYTE => self.cpy_addr_num_byte(params.address(), params.byte()),
            ops::CPY_ADDR_REG_WORD => self.cpy_addr_reg_word(params.address(), params.register()?),
            ops::CPY_ADDR_ADDR_WORD => self.cpy_addr_addr_word(params.address(), params.address()),
            ops::CPY_ADDR_NUM_WORD => self.cpy_addr_num_word(params.address(), params.word()),
            ops::CPY_REG_NUM_WORD => self.cpy_reg_num_word(params.register()?, params.word()),
            ops::SUB_REG_REG_BYTE => self.sub_reg_reg_byte(params.register()?, params.register()?),
            ops::SUB_REG_REG_WORD => self.sub_reg_reg_word(params.register()?, params.register()?),
            ops::SUB_REG_NUM_BYTE => self.sub_reg_num_byte(params.register()?, params.byte()),
            ops::SUB_REG_NUM_WORD => self.sub_reg_num_word(params.register()?, params.word()),
            ops::SUB_REG_ADDR_BYTE => self.sub_reg_addr_byte(params.register()?, params.address()),
            ops::SUB_REG_ADDR_WORD => self.sub_reg_addr_word(params.register()?, params.address()),
            ops::SUB_ADDR_REG_BYTE => self.sub_addr_reg_byte(params.address(), params.register()?),
            ops::SUB_ADDR_ADDR_BYTE => self.sub_addr_addr_byte(params.address(), params.address()),
            ops::SUB_ADDR_NUM_BYTE => self.sub_addr_num_byte(params.address(), params.byte()),
            ops::SUB_ADDR_REG_WORD => self.sub_addr_reg_word(params.address(), params.register()?),
            ops::SUB_ADDR_ADDR_WORD => self.sub_addr_addr_word(params.address(), params.address()),
            ops::SUB_ADDR_NUM_WORD => self.sub_addr_num_word(params.address(), params.word()),
            ops::SUBC_REG_REG_BYTE => self.subc_reg_reg_byte(params.register()?, params.register()?),
            ops::SUBC_REG_REG_WORD => self.subc_reg_reg_word(params.register()?, params.register()?),
            ops::SUBC_REG_NUM_BYTE => self.subc_reg_num_byte(params.register()?, params.byte()),
            ops::SUBC_REG_NUM_WORD => self.subc_reg_num_word(params.register()?, params.word()),
            ops::SUBC_REG_ADDR_BYTE => self.subc_reg_addr_byte(params.register()?, params.address()),
            ops::SUBC_REG_ADDR_WORD => self.subc_reg_addr_word(params.register()?, params.address()),
            ops::SUBC_ADDR_REG_BYTE => self.subc_addr_reg_byte(params.address(), params.register()?),
            ops::SUBC_ADDR_ADDR_BYTE => self.subc_addr_addr_byte(params.address(), params.address()),
            ops::SUBC_ADDR_NUM_BYTE => self.subc_addr_num_byte(params.address(), params.byte()),
            ops::SUBC_ADDR_REG_WORD => self.subc_addr_reg_word(params.address(), params.register()?),
            ops::SUBC_ADDR_ADDR_WORD => self.subc_addr_addr_word(params.address(), params.address()),
            ops::SUBC_ADDR_NUM_WORD => self.subc_addr_num_word(params.address(), params.word()),
            ops::NOT_REG_BYTE => self.not_reg_byte(params.register()?),
            ops::NOT_REG_WORD => self.not_reg_word(params.register()?),
            ops::AND_REG_REG_BYTE => self.and_reg_reg_byte(params.register()?, params.register()?),
            ops::AND_REG_REG_WORD => self.and_reg_reg_word(params.register()?, params.register()?),
            ops::AND_REG_NUM_BYTE => self.and_reg_num_byte(params.register()?, params.byte()),
            ops::AND_REG_NUM_WORD => self.and_reg_num_word(params.register()?, params.word()),
            ops::XOR_REG_REG_BYTE => self.xor_reg_reg_byte(params.register()?, params.register()?),
            ops::XOR_REG_REG_WORD => self.xor_reg_reg_word(params.register()?, params.register()?),
            ops::XOR_REG_NUM_BYTE => self.xor_reg_num_byte(params.register()?, params.byte()),
            ops::XOR_REG_NUM_WORD => self.xor_reg_num_word(params.register()?, params.word()),
            ops::OR_REG_REG_BYTE => self.or_reg_reg_byte(params.register()?, params.register()?),
            ops::OR_REG_REG_WORD => self.or_reg_reg_word(params.register()?, params.register()?),
            ops::OR_REG_NUM_BYTE => self.or_reg_num_byte(params.register()?, params.byte()),
            ops::OR_REG_NUM_WORD => self.or_reg_num_word(params.register()?, params.word()),
            ops::SWAP_REG_REG_BYTE => self.swap_reg_reg_byte(params.register()?, params.register()?),
            ops::SWAP_REG_REG_WORD => self.swap_reg_reg_word(params.register()?, params.register()?),
            ops::MUL_REG_REG_BYTE => self.mul_reg_reg_byte(params.register()?, params.register()?),
            ops::MUL_REG_REG_WORD => self.mul_reg_reg_word(params.register()?, params.register()?),
            ops::MUL_REG_NUM_BYTE => self.mul_reg_num_byte(params.register()?, params.byte()),
            ops::MUL_REG_NUM_WORD => self.mul_reg_num_word(params.register()?, params.word()),
            ops::MUL_REG_ADDR_BYTE => self.mul_reg_addr_byte(params.register()?, params.address()),
            ops::MUL_REG_ADDR_WORD => self.mul_reg_addr_word(params.register()?, params.address()),
            ops::MUL_ADDR_REG_BYTE => self.mul_addr_reg_byte(params.address(), params.register()?),
            ops::MUL_ADDR_REG_WORD => self.mul_addr_reg_word(params.address(), params.register()?),
            ops::MUL_ADDR_NUM_BYTE => self.mul_addr_num_byte(params.address(), params.byte()),
            ops::MUL_ADDR_NUM_WORD => self.mul_addr_num_word(params.address(), params.word()),
            ops::MUL_ADDR_ADDR_BYTE => self.mul_addr_addr_byte(params.address(), params.address()),
            ops::MUL_ADDR_ADDR_WORD => self.mul_addr_addr_word(params.address(), params.address()),
            ops::MULS_REG_REG_BYTE => self.muls_reg_reg_byte(params.register()?, params.register()?),
            ops::MULS_REG_REG_WORD => self.muls_reg_reg_word(params.register()?, params.register()?),
            ops::MULS_REG_NUM_BYTE => self.muls_reg_num_byte(params.register()?, params.byte()),
            ops::MULS_REG_NUM_WORD => self.muls_reg_num_word(params.register()?, params.word()),
            ops::MULS_REG_ADDR_BYTE => self.muls_reg_addr_byte(params.register()?, params.address()),
            ops::MULS_REG_ADDR_WORD => self.muls_reg_addr_word(params.register()?, params.address()),
            ops::MULS_ADDR_REG_BYTE => self.muls_addr_reg_byte(params.address(), params.register()?),
            ops::MULS_ADDR_REG_WORD => self.muls_addr_reg_word(params.address(), params.register()?),
            ops::MULS_ADDR_NUM_BYTE => self.muls_addr_num_byte(params.address(), params.byte()),
            ops::MULS_ADDR_NUM_WORD => self.muls_addr_num_word(params.address(), params.word()),
            ops::MULS_ADDR_ADDR_BYTE => self.muls_addr_addr_byte(params.address(), params.address()),
            ops::MULS_ADDR_ADDR_WORD => self.muls_addr_addr_word(params.address(), params.address()),
            ops::DIV_REG_REG_BYTE => self.div_reg_reg_byte(params.register()?, params.register()?),
            ops::DIV_REG_REG_WORD => self.div_reg_reg_word(params.register()?, params.register()?),
            ops::DIV_REG_NUM_BYTE => self.div_reg_num_byte(params.register()?, params.byte()),
            ops::DIV_REG_NUM_WORD => self.div_reg_num_word(params.register()?, params.word()),
            ops::DIV_REG_ADDR_BYTE => self.div_reg_addr_byte(params.register()?, params.address()),
            ops::DIV_REG_ADDR_WORD => self.div_reg_addr_word(params.register()?, params.address()),
            ops::DIV_ADDR_REG_BYTE => self.div_addr_reg_byte(params.address(), params.register()?),
            ops::DIV_ADDR_REG_WORD => self.div_addr_reg_word(params.address(), params.register()?),
            ops::DIV_ADDR_NUM_BYTE => self.div_addr_num_byte(params.address(), params.byte()),
            ops::DIV_ADDR_NUM_WORD => self.div_addr_num_word(params.address(), params.word()),
            ops::DIV_ADDR_ADDR_BYTE => self.div_addr_addr_byte(params.address(), params.address()),
            ops::DIV_ADDR_ADDR_WORD => self.div_addr_addr_word(params.address(), params.address()),
            ops::DIVS_REG_REG_BYTE => self.divs_reg_reg_byte(params.register()?, params.register()?),
            ops::DIVS_REG_REG_WORD => self.divs_reg_reg_word(params.register()?, params.register()?),
            ops::DIVS_REG_NUM_BYTE => self.divs_reg_num_byte(params.register()?, params.byte()),
            ops::DIVS_REG_NUM_WORD => self.divs_reg_num_word(params.register()?, params.word()),
            ops::DIVS_REG_ADDR_BYTE => self.divs_reg_addr_byte(params.register()?, params.address()),
            ops::DIVS_REG_ADDR_WORD => self.divs_reg_addr_word(params.register()?, params.address()),
            ops::DIVS_ADDR_REG_BYTE => self.divs_addr_reg_byte(params.address(), params.register()?),
            ops::DIVS_ADDR_REG_WORD => self.divs_addr_reg_word(params.address(), params.register()?),
            ops::DIVS_ADDR_NUM_BYTE => self.divs_addr_num_byte(params.address(), params.byte()),
            ops::DIVS_ADDR_NUM_WORD => self.divs_addr_num_word(params.address(), params.word()),
            ops::DIVS_ADDR_ADDR_BYTE => self.divs_addr_addr_byte(params.address(), params.address()),
            ops::DIVS_ADDR_ADDR_WORD => self.divs_addr_addr_word(params.address(), params.address()),
            ops::MEM_CPY_REG_REG_BYTE => self.mem_cpy_reg_reg_byte(params.register()?, params.register()?, params.byte()),
            ops::MEM_CPY_REG_ADDR_BYTE => self.mem_cpy_reg_addr_byte(params.register()?, params.address(), params.byte()),
            ops::MEM_CPY_ADDR_REG_BYTE => self.mem_cpy_addr_reg_byte(params.address(), params.register()?, params.byte()),
            ops::MEM_CPY_ADDR_ADDR_BYTE => self.mem_cpy_addr_addr_byte(params.address(), params.address(), params.byte()),
            ops::MEM_CPY_REG_REG_REG => self.mem_cpy_reg_reg_byte(params.register()?, params.register()?, params.byte()),
            ops::MEM_CPY_REG_ADDR_REG => self.mem_cpy_reg_addr_reg(params.register()?, params.address(), params.register()?),
            ops::MEM_CPY_ADDR_REG_REG => self.mem_cpy_addr_reg_reg(params.address(), params.register()?, params.register()?),
            ops::MEM_CPY_ADDR_ADDR_REG => self.mem_cpy_addr_addr_reg(params.address(), params.address(), params.register()?),
            ops::CMP_REG_REG_BYTE => self.cmp_reg_reg_byte(params.register()?, params.register()?),
            ops::CMP_REG_REG_WORD => self.cmp_reg_reg_word(params.register()?, params.register()?),
            ops::CMP_REG_NUM_BYTE => self.cmp_reg_num_byte(params.register()?, params.byte()),
            ops::CMP_REG_NUM_WORD => self.cmp_reg_num_word(params.register()?, params.word()),
            ops::CMP_REG_ADDR_BYTE => self.cmp_reg_num_byte(params.register()?, params.byte()),
            ops::CMP_REG_ADDR_WORD => self.cmp_reg_num_word(params.register()?, params.word()),
            ops::CMPS_REG_REG_BYTE => self.cmps_reg_reg_byte(params.register()?, params.register()?),
            ops::CMPS_REG_REG_WORD => self.cmps_reg_reg_word(params.register()?, params.register()?),
            ops::CMPS_REG_NUM_BYTE => self.cmps_reg_num_byte(params.register()?, params.byte()),
            ops::CMPS_REG_NUM_WORD => self.cmps_reg_num_word(params.register()?, params.word()),
            ops::CMPS_REG_ADDR_BYTE => self.cmps_reg_num_byte(params.register()?, params.byte()),
            ops::CMPS_REG_ADDR_WORD => self.cmps_reg_num_word(params.register()?, params.word()),
            // ops::ASL_REG_NUM_BYTE => self.asl_reg_byte_byte(params.register()?, params.byte()),
            // ops::ASL_REG_NUM_WORD => self.asl_reg_byte_word(params.register()?, params.byte()),
            // ops::ASL_ADDR_BYTE => self.asl_addr_byte_byte(params.address(), params.byte()),
            // ops::ASL_ADDR_WORD => self.asl_addr_byte_word(params.address(), params.byte()),
            // ops::ASL_REG_BYTE => self.asl_reg_byte(params.register()?),
            // ops::ASL_REG_WORD => self.asl_reg_word(params.register()?),
            // ops::ASL_REG_REG_BYTE => self.asl_reg_reg_byte(params.register()?, params.register()?),
            // ops::ASL_REG_REG_WORD => self.asl_reg_reg_word(params.register()?, params.register()?),
            // ops::ASR_REG_NUM_BYTE => self.asr_reg_byte_byte(params.register()?, params.byte()),
            // ops::ASR_REG_NUM_WORD => self.asr_reg_byte_word(params.register()?, params.byte()),
            // ops::ASR_ADDR_BYTE => self.asr_addr_byte_byte(params.address(), params.byte()),
            // ops::ASR_ADDR_WORD => self.asr_addr_byte_word(params.address(), params.byte()),
            // ops::ASR_REG_BYTE => self.asr_reg_byte(params.register()?),
            // ops::ASR_REG_WORD => self.asr_reg_word(params.register()?),
            // ops::ASR_REG_REG_BYTE => self.asr_reg_reg_byte(params.register()?, params.register()?),
            // ops::ASR_REG_REG_WORD => self.asr_reg_reg_word(params.register()?, params.register()?),
            // ops::LSL_REG_NUM_BYTE => self.lsl_reg_byte_byte(params.register()?, params.byte()),
            // ops::LSL_REG_NUM_WORD => self.lsl_reg_byte_word(params.register()?, params.byte()),
            // ops::LSL_ADDR_BYTE => self.lsl_addr_byte_byte(params.address(), params.byte()),
            // ops::LSL_ADDR_WORD => self.lsl_addr_byte_word(params.address(), params.byte()),
            // ops::LSL_REG_BYTE => self.lsl_reg_byte(params.register()?),
            // ops::LSL_REG_WORD => self.lsl_reg_word(params.register()?),
            // ops::LSL_REG_REG_BYTE => self.lsl_reg_reg_byte(params.register()?, params.register()?),
            // ops::LSL_REG_REG_WORD => self.lsl_reg_reg_word(params.register()?, params.register()?),
            // ops::LSR_REG_NUM_BYTE => self.lsr_reg_byte_byte(params.register()?, params.byte()),
            // ops::LSR_REG_NUM_WORD => self.lsr_reg_byte_word(params.register()?, params.byte()),
            // ops::LSR_ADDR_BYTE => self.lsr_addr_byte_byte(params.address(), params.byte()),
            // ops::LSR_ADDR_WORD => self.lsr_addr_byte_word(params.address(), params.byte()),
            // ops::LSR_REG_BYTE => self.lsr_reg_byte(params.register()?),
            // ops::LSR_REG_WORD => self.lsr_reg_word(params.register()?),
            // ops::LSR_REG_REG_BYTE => self.lsr_reg_reg_byte(params.register()?, params.register()?),
            // ops::LSR_REG_REG_WORD => self.lsr_reg_reg_word(params.register()?, params.register()?),
            // ops::ROL_REG_NUM_BYTE => self.rol_reg_byte_byte(params.register()?, params.byte()),
            // ops::ROL_REG_NUM_WORD => self.rol_reg_byte_word(params.register()?, params.byte()),
            // ops::ROL_ADDR_BYTE => self.rol_addr_byte_byte(params.address(), params.byte()),
            // ops::ROL_ADDR_WORD => self.rol_addr_byte_word(params.address(), params.byte()),
            // ops::ROL_REG_BYTE => self.rol_reg_byte(params.register()?),
            // ops::ROL_REG_WORD => self.rol_reg_word(params.register()?),
            // ops::ROL_REG_REG_BYTE => self.rol_reg_reg_byte(params.register()?, params.register()?),
            // ops::ROL_REG_REG_WORD => self.rol_reg_reg_word(params.register()?, params.register()?),
            // ops::ROR_REG_NUM_BYTE => self.ror_reg_byte_byte(params.register()?, params.byte()),
            // ops::ROR_REG_NUM_WORD => self.ror_reg_byte_word(params.register()?, params.byte()),
            // ops::ROR_ADDR_BYTE => self.ror_addr_byte_byte(params.address(), params.byte()),
            // ops::ROR_ADDR_WORD => self.ror_addr_byte_word(params.address(), params.byte()),
            // ops::ROR_REG_BYTE => self.ror_reg_byte(params.register()?),
            // ops::ROR_REG_WORD => self.ror_reg_word(params.register()?),
            // ops::ROR_REG_REG_BYTE => self.ror_reg_reg_byte(params.register()?, params.register()?),
            // ops::ROR_REG_REG_WORD => self.ror_reg_reg_word(params.register()?, params.register()?),
            _ => return Err(format!("Unknown op: {:02X}", op)),
        };
        Ok(false)
    }
    
    fn execute_must_jmp_op(&mut self, op: u8, mut params: VecDeque<u8>) -> Result<bool, String> {
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
    
    fn execute_may_jmp_op(&mut self, op: u8, mut params: VecDeque<u8>) -> Result<bool, String> {
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
}

trait ReadParams {
    fn byte(&mut self) -> Byte;
    fn word(&mut self) -> Word;
    fn register(&mut self) -> Result<Register, String>;
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

    fn register(&mut self) -> Result<Register, String> {
        Register::from(self.pop_front().unwrap())
    }

    fn address(&mut self) -> Address {
        Address(u16::from_be_bytes([
            self.pop_front().unwrap(),
            self.pop_front().unwrap(),
        ]))
    }
}
