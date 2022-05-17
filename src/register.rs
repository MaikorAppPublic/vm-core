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
    pub fn from(byte: u8) -> Result<Register, String> {
        let (size, start) = Register::read_reg(byte)?;
        let mut reg = Register::read_meta(byte)?;
        reg.addr = start;
        reg.size = size;
        Ok(reg)
    }

    pub fn id(&self) -> usize {
        match (self.addr, self.size) {
            (offset::AH, 2) => id::AX,
            (offset::AL, 1) => id::AL,
            (offset::AH, 1) => id::AH,
            (offset::BH, 2) => id::BX,
            (offset::BL, 1) => id::BL,
            (offset::BH, 1) => id::BH,
            (offset::CH, 2) => id::CX,
            (offset::CL, 1) => id::CL,
            (offset::CH, 1) => id::CH,
            (offset::DH, 2) => id::DX,
            (offset::DL, 1) => id::DL,
            (offset::DH, 1) => id::DH,
            _ => id::FLAGS,
        }
    }

    fn read_reg(byte: u8) -> Result<(usize, usize), String> {
        let reg_id = (byte & 0x0F) as usize;
        let result = match reg_id {
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
            _ => return Err(format!("invalid reg param: {byte}")),
        };
        Ok(result)
    }

    fn read_meta(byte: u8) -> Result<Register, String> {
        use maikor_language::op_params::*;
        let reg = match byte & 0xF0 {
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
            _ => return Err(format!("invalid reg meta: {} -> {}", byte, byte & 0xF0)),
        };
        Ok(reg)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_reg_from() {
        assert_eq!(
            Register::from(id::AL as u8).unwrap(),
            Register {
                is_indirect: false,
                is_calc: false,
                is_post: false,
                is_inc: false,
                size: 1,
                addr: 1
            }
        );
        assert_eq!(
            Register::from(id::AH as u8).unwrap(),
            Register {
                is_indirect: false,
                is_calc: false,
                is_post: false,
                is_inc: false,
                size: 1,
                addr: 0
            }
        );
        assert_eq!(
            Register::from(id::AX as u8).unwrap(),
            Register {
                is_indirect: false,
                is_calc: false,
                is_post: false,
                is_inc: false,
                size: 2,
                addr: 0
            }
        );
    }
}
