use maikor_language::registers;

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Register {
    pub is_indirect: bool,
    pub is_calc: bool,
    pub is_post: bool,
    pub is_inc: bool,
    pub size: usize,
    pub addr: usize,
}

impl Register {
    pub fn from(byte: u8) -> Register {
        let (size, start) = Register::read_reg(byte);
        let mut reg = Register::read_meta(byte);
        reg.addr = start;
        reg.size = size;
        reg
    }

    fn read_reg(byte: u8) -> (usize, usize) {
        let reg_id = (byte & 0x0F) as usize;
        return match reg_id {
            registers::id::AL => (1, registers::offset::AL),
            registers::id::BL => (1, registers::offset::BL),
            registers::id::CL => (1, registers::offset::CL),
            registers::id::DL => (1, registers::offset::DL),
            registers::id::AH => (1, registers::offset::AH),
            registers::id::BH => (1, registers::offset::BH),
            registers::id::CH => (1, registers::offset::CH),
            registers::id::DH => (1, registers::offset::DH),
            registers::id::AX => (2, registers::offset::AX),
            registers::id::BX => (2, registers::offset::BX),
            registers::id::CX => (2, registers::offset::CX),
            registers::id::DX => (2, registers::offset::DX),
            registers::id::FLAGS => (1, registers::offset::FLAGS),
            _ => panic!("invalid reg param: {byte}"),
        };
    }

    fn read_meta(byte: u8) -> Register {
        use maikor_language::op_params::*;
        return match byte & 0xF0 {
            REGISTER => Register {
                ..Register::default()
            },
            POST_INC => Register {
                is_calc: true,
                is_post: true,
                is_inc: true,
                ..Register::default()
            },
            PRE_INC => Register {
                is_calc: true,
                is_inc: true,
                ..Register::default()
            },
            POST_DEC => Register {
                is_calc: true,
                is_post: true,
                ..Register::default()
            },
            PRE_DEC => Register {
                is_calc: true,
                ..Register::default()
            },
            INDIRECT => Register {
                is_indirect: true,
                ..Register::default()
            },
            IND_POST_INC => Register {
                is_calc: true,
                is_post: true,
                is_inc: true,
                is_indirect: true,
                ..Register::default()
            },
            IND_PRE_INC => Register {
                is_calc: true,
                is_inc: true,
                is_indirect: true,
                ..Register::default()
            },
            IND_POST_DEC => Register {
                is_calc: true,
                is_post: true,
                is_indirect: true,
                ..Register::default()
            },
            IND_PRE_DEC => Register {
                is_calc: true,
                is_indirect: true,
                ..Register::default()
            },
            _ => panic!("impossible: {} -> {}", byte, byte & 0xF0),
        };
    }
}
