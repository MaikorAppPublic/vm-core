use maikor_language::registers::id;

pub mod offset {
    pub const AH: usize = 0;
    pub const AL: usize = 1;
    pub const BH: usize = 2;
    pub const BL: usize = 3;
    pub const CH: usize = 4;
    pub const CL: usize = 5;
    pub const DH: usize = 6;
    pub const DL: usize = 7;
    pub const FLAGS: usize = 8;
}

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
            id::AL => (1, offset::AL),
            id::BL => (1, offset::BL),
            id::CL => (1, offset::CL),
            id::DL => (1, offset::DL),
            id::AH => (1, offset::AH),
            id::BH => (1, offset::BH),
            id::CH => (1, offset::CH),
            id::DH => (1, offset::DH),
            id::AX => (2, offset::AH),
            id::BX => (2, offset::BH),
            id::CX => (2, offset::CH),
            id::DX => (2, offset::DH),
            id::FLAGS => (1, offset::FLAGS),
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
