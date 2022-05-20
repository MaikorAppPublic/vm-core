use maikor_platform::registers::id;

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
    pub is_offset_reg: bool,
    pub is_offset_ext_reg: bool,
    pub is_offset_num: bool,
    pub size: usize,
    pub addr: usize,
}

impl Register {
    pub fn from(byte: u8) -> Register {
        use maikor_platform::op_params::*;
        let (size, addr) = match (byte & 0x0F) as usize {
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
            _ => (1, offset::AH),
        };

        match byte & 0xF0 {
            REGISTER => Register::new_no_calc(false, size, addr),
            INDIRECT => Register::new_no_calc(true, size, addr),
            POST_INC => Register::new_ppid(false, true, true, true, size, addr),
            PRE_INC => Register::new_ppid(false, true, false, true, size, addr),
            POST_DEC => Register::new_ppid(false, true, true, false, size, addr),
            PRE_DEC => Register::new_ppid(false, true, false, false, size, addr),
            IND_POST_INC => Register::new_ppid(true, true, true, true, size, addr),
            IND_PRE_INC => Register::new_ppid(true, true, false, true, size, addr),
            IND_POST_DEC => Register::new_ppid(true, true, true, false, size, addr),
            IND_PRE_DEC => Register::new_ppid(true, true, false, false, size, addr),
            IND_OFFSET_REG => Register::new_offset(true, false, false, size, addr),
            IND_OFFSET_EXT_REG => Register::new_offset(false, true, false, size, addr),
            IND_OFFSET_NUM => Register::new_offset(false, false, true, size, addr),
            _ => Register::new_no_calc(false, size, addr),
        }
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

    pub fn new_no_calc(is_indirect: bool, size: usize, addr: usize) -> Self {
        Self {
            is_indirect,
            is_calc: false,
            is_post: false,
            is_inc: false,
            is_offset_reg: false,
            is_offset_ext_reg: false,
            is_offset_num: false,
            size,
            addr,
        }
    }

    pub fn new_offset(
        is_offset_reg: bool,
        is_offset_ext_reg: bool,
        is_offset_num: bool,
        size: usize,
        addr: usize,
    ) -> Self {
        Self {
            is_indirect: true,
            is_calc: false,
            is_post: false,
            is_inc: false,
            is_offset_reg,
            is_offset_ext_reg,
            is_offset_num,
            size,
            addr,
        }
    }

    pub fn new_ppid(
        is_indirect: bool,
        is_calc: bool,
        is_post: bool,
        is_inc: bool,
        size: usize,
        addr: usize,
    ) -> Self {
        Self {
            is_indirect,
            is_calc,
            is_post,
            is_inc,
            is_offset_reg: false,
            is_offset_ext_reg: false,
            is_offset_num: false,
            size,
            addr,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::register::offset;
    use maikor_platform::op_params::{INDIRECT, IND_OFFSET_NUM, IND_PRE_DEC, POST_INC};

    #[test]
    fn check_reg_from() {
        assert_eq!(
            Register::from(id::AL as u8),
            Register::new_no_calc(false, 1, offset::AL)
        );

        assert_eq!(
            Register::from(id::BX as u8),
            Register::new_no_calc(false, 2, offset::BH)
        );

        assert_eq!(
            Register::from(id::CX as u8 | INDIRECT),
            Register::new_no_calc(true, 2, offset::CH)
        );

        assert_eq!(
            Register::from(id::AL as u8 | POST_INC),
            Register::new_ppid(false, true, true, true, 1, offset::AL)
        );

        assert_eq!(
            Register::from(id::DX as u8 | IND_PRE_DEC),
            Register::new_ppid(true, true, false, false, 2, offset::DH)
        );

        assert_eq!(
            Register::from(id::BX as u8 | IND_OFFSET_NUM),
            Register::new_offset(false, false, true, 2, offset::BH)
        );
    }
}
