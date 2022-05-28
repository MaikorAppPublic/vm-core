mod positive_zero {
    use crate::offset;
    use crate::single::{test_op, test_op_init};
    use maikor_platform::ops::*;
    use maikor_platform::registers::flags::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_mul_reg_num_byte() {
        test_op(
            "MUL.B AH, 1",
            &[MUL_REG_NUM_BYTE, id::AH, 1],
            &[(offset::FLAGS, ZERO | INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_word() {
        test_op_init(
            "MUL.W AX, 1",
            &[MUL_REG_NUM_WORD, id::AX, 0, 1],
            &[(offset::AH, 1)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_word_word() {
        test_op(
            "MUL.W AX, x100",
            &[MUL_REG_NUM_WORD, id::AX, 1, 1],
            &[(offset::AH, 0), (offset::FLAGS, ZERO | INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_reg_byte() {
        test_op(
            "MUL.B AH, AL",
            &[MUL_REG_REG_BYTE, id::AH, id::AL],
            &[],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_reg_word() {
        test_op(
            "MUL.W AX, BX",
            &[MUL_REG_REG_WORD, id::AX, id::BX],
            &[],
            &[],
        );
    }

    #[test]
    fn test_mul_addr_num_byte() {
        test_op("MUL.B $100, 0", &[MUL_ADDR_NUM_BYTE, 0, 100, 0], &[], &[]);
    }

    #[test]
    fn test_mul_addr_num_word() {
        test_op(
            "MUL.W $100, 0",
            &[MUL_ADDR_NUM_WORD, 0, 100, 0, 0],
            &[],
            &[],
        );
    }

    #[test]
    fn test_mul_addr_reg_byte() {
        test_op(
            "MUL.B $100, AH",
            &[MUL_ADDR_REG_BYTE, 0, 100, id::AH],
            &[],
            &[],
        );
    }

    #[test]
    fn test_mul_addr_reg_word() {
        test_op(
            "MUL.W $100, AX",
            &[MUL_ADDR_REG_WORD, 0, 100, id::AX],
            &[],
            &[],
        );
    }
}

mod positive_one {
    use crate::offset;
    use crate::single::test_op_init;
    use maikor_platform::ops::*;
    use maikor_platform::registers::flags::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_mul_reg_num_byte() {
        test_op_init(
            "MUL.B AH, 0",
            &[MUL_REG_NUM_BYTE, id::AH, 0],
            &[(offset::AH, 1)],
            &[],
            &[(offset::FLAGS, INTERRUPTS | ZERO), (offset::AH, 0)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_word() {
        test_op_init(
            "MUL.W AX, 0",
            &[MUL_REG_NUM_WORD, id::AX, 0, 0],
            &[(offset::AL, 1)],
            &[],
            &[(offset::AL, 0), (offset::FLAGS, ZERO | INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_word_word() {
        test_op_init(
            "MUL.W AX, x100",
            &[MUL_REG_NUM_WORD, id::AX, 0, 0],
            &[(offset::AH, 2), (offset::AL, 7)],
            &[],
            &[
                (offset::FLAGS, ZERO | INTERRUPTS),
                (offset::AH, 0),
                (offset::AL, 0),
            ],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_reg_byte() {
        test_op_init(
            "MUL.B AH, AL",
            &[MUL_REG_REG_BYTE, id::AH, id::AL],
            &[(offset::AH, 1), (offset::AL, 4)],
            &[],
            &[(offset::FLAGS, INTERRUPTS), (offset::AH, 4)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_reg_word() {
        test_op_init(
            "MUL.W AX, BX",
            &[MUL_REG_REG_WORD, id::AX, id::BX],
            &[(offset::AL, 1), (offset::BH, 2)],
            &[],
            &[
                (offset::FLAGS, INTERRUPTS),
                (offset::AH, 2),
                (offset::AL, 0),
            ],
            &[],
        );
    }

    #[test]
    fn test_mul_addr_num_byte() {
        test_op_init(
            "MUL.B $100, 0",
            &[MUL_ADDR_NUM_BYTE, 0, 100, 0],
            &[],
            &[],
            &[(offset::FLAGS, ZERO | INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_addr_num_word() {
        test_op_init(
            "MUL.W $100, 0",
            &[MUL_ADDR_NUM_WORD, 0, 100, 0, 0],
            &[],
            &[],
            &[(offset::FLAGS, ZERO | INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_addr_num_word_word() {
        test_op_init(
            "MUL.W $100, x101",
            &[MUL_ADDR_NUM_WORD, 0, 100, 1, 1],
            &[],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 1), (101, 1)],
        );
    }

    #[test]
    fn test_mul_addr_reg_byte() {
        test_op_init(
            "MUL.B $100, AH",
            &[MUL_ADDR_REG_BYTE, 0, 100, id::AH],
            &[(offset::AH, 3)],
            &[(100, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 3)],
        );
    }

    #[test]
    fn test_mul_addr_reg_word() {
        test_op_init(
            "MUL.W $100, AX",
            &[MUL_ADDR_REG_WORD, 0, 100, id::AX],
            &[(offset::AL, 2)],
            &[(101, 4)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(101, 8)],
        );
    }
}

mod positive_one_init_one {
    use crate::offset;
    use crate::single::test_op_init;
    use maikor_platform::ops::*;
    use maikor_platform::registers::flags::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_mul_reg_num_byte() {
        test_op_init(
            "MUL.B AH, 1",
            &[MUL_REG_NUM_BYTE, id::AH, 1],
            &[(offset::AH, 1)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_word() {
        test_op_init(
            "MUL.W AX, 1",
            &[MUL_REG_NUM_WORD, id::AX, 0, 3],
            &[(offset::AL, 1)],
            &[],
            &[(offset::AL, 3), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_reg_byte() {
        test_op_init(
            "MUL.B AH, AL",
            &[MUL_REG_REG_BYTE, id::AH, id::AL],
            &[(offset::AH, 5), (offset::AL, 56)],
            &[],
            &[(offset::AH, 24), (offset::FLAGS, INTERRUPTS | CARRY)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_reg_word() {
        test_op_init(
            "MUL.W AX, BX",
            &[MUL_REG_REG_WORD, id::AX, id::BX],
            &[(offset::AL, 9), (offset::BL, 4)],
            &[],
            &[(offset::AL, 36), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_addr_num_byte() {
        test_op_init(
            "MUL.B $100, 0",
            &[MUL_ADDR_NUM_BYTE, 0, 100, 1],
            &[],
            &[(100, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 1)],
        );
    }

    #[test]
    fn test_mul_addr_num_word() {
        test_op_init(
            "MUL.W $100, 0",
            &[MUL_ADDR_NUM_WORD, 0, 100, 0, 0],
            &[],
            &[(101, 1)],
            &[(offset::FLAGS, ZERO | INTERRUPTS)],
            &[(101, 0)],
        );
    }

    #[test]
    fn test_mul_addr_reg_byte() {
        test_op_init(
            "MUL.B $100, AH",
            &[MUL_ADDR_REG_BYTE, 0, 100, id::AH],
            &[(offset::AH, 9)],
            &[(100, 9)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 81)],
        );
    }

    #[test]
    fn test_mul_addr_reg_word() {
        test_op_init(
            "MUL.W $100, AX",
            &[MUL_ADDR_REG_WORD, 0, 100, id::AX],
            &[(offset::AH, 3)],
            &[(101, 6)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 18), (101, 0)],
        );
    }

    #[test]
    fn test_mul_addr_reg_word_word() {
        test_op_init(
            "MUL.W $100, AX",
            &[MUL_ADDR_REG_WORD, 0, 100, id::AX],
            &[(offset::AH, 1), (offset::AL, 15)],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 1), (101, 15)],
        );
    }
}

mod overflow {
    use crate::offset;
    use crate::single::test_op_init;
    use maikor_platform::ops::*;
    use maikor_platform::registers::flags::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_mul_reg_num_byte() {
        test_op_init(
            "MUL.B AH, 1",
            &[MUL_REG_NUM_BYTE, id::AH, 1],
            &[(offset::AH, 255)],
            &[],
            &[(offset::AH, 255), (offset::FLAGS, INTERRUPTS | SIGNED)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_word() {
        test_op_init(
            "MUL.W AX, 1",
            &[MUL_REG_NUM_WORD, id::AX, 0, 1],
            &[(offset::AH, 255), (offset::AL, 255)],
            &[],
            &[
                (offset::AH, 255),
                (offset::AL, 255),
                (offset::FLAGS, INTERRUPTS | SIGNED),
            ],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_word_word() {
        test_op_init(
            "MUL.W AX, xFF01",
            &[MUL_REG_NUM_WORD, id::AX, 255, 1],
            &[(offset::AL, 255)],
            &[],
            &[
                (offset::AH, 1),
                (offset::AL, 255),
                (offset::FLAGS, INTERRUPTS | CARRY),
            ],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_reg_byte() {
        test_op_init(
            "MUL.B AH, AL",
            &[MUL_REG_REG_BYTE, id::AH, id::AL],
            &[(offset::AH, 240), (offset::AL, 17)],
            &[],
            &[
                (offset::AH, 240),
                (offset::FLAGS, CARRY | SIGNED | INTERRUPTS),
            ],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_reg_word() {
        test_op_init(
            "MUL.W AX, BX",
            &[MUL_REG_REG_WORD, id::AX, id::BX],
            &[
                (offset::AH, 200),
                (offset::AL, 200),
                (offset::BH, 100),
                (offset::BL, 100),
            ],
            &[],
            &[
                (offset::AH, 142),
                (offset::AL, 32),
                (offset::FLAGS, CARRY | SIGNED | INTERRUPTS),
            ],
            &[],
        );
    }

    #[test]
    fn test_mul_addr_num_byte() {
        test_op_init(
            "MUL.B $100, 241",
            &[MUL_ADDR_NUM_BYTE, 0, 100, 241],
            &[],
            &[(100, 255)],
            &[(offset::FLAGS, CARRY | OVERFLOW | INTERRUPTS)],
            &[(100, 15)],
        );
    }

    #[test]
    fn test_mul_addr_num_word() {
        test_op_init(
            "MUL.W $100, 1",
            &[MUL_ADDR_NUM_WORD, 0, 100, 0, 1],
            &[],
            &[(100, 255), (101, 255)],
            &[(offset::FLAGS, SIGNED | INTERRUPTS)],
            &[(100, 255), (101, 255)],
        );
    }

    #[test]
    fn test_mul_addr_num_word_word() {
        test_op_init(
            "MUL.W $100, x101",
            &[MUL_ADDR_NUM_WORD, 0, 100, 1, 1],
            &[],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 1), (101, 1)],
        );
    }

    #[test]
    fn test_mul_addr_reg_byte() {
        test_op_init(
            "MUL.B $100, AH",
            &[MUL_ADDR_REG_BYTE, 0, 100, id::AH],
            &[(offset::AH, 20)],
            &[(100, 255)],
            &[(offset::FLAGS, INTERRUPTS | CARRY | SIGNED)],
            &[(100, 236)],
        );
    }

    #[test]
    fn test_mul_addr_reg_word() {
        test_op_init(
            "MUL.W $100, AX",
            &[MUL_ADDR_REG_WORD, 0, 100, id::AX],
            &[(offset::AH, 100), (offset::AL, 100)],
            &[(100, 164), (101, 255)],
            &[(offset::FLAGS, INTERRUPTS | CARRY | OVERFLOW)],
            &[(100, 15), (101, 156)],
        );
    }
}

mod ppid {
    use crate::offset;
    use crate::single::test_op_init;
    use maikor_platform::op_params::*;
    use maikor_platform::ops::*;
    use maikor_platform::registers::flags::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_mul_reg_num_pre_inc() {
        test_op_init(
            "MUL.B +AL, 2",
            &[MUL_REG_NUM_BYTE, id::AL | PRE_INC, 2],
            &[(offset::AL, 50)],
            &[],
            &[(offset::AL, 102), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_byte_ind() {
        test_op_init(
            "MUL.B (AX), 1",
            &[MUL_REG_NUM_BYTE, id::AX | INDIRECT, 1],
            &[(offset::AL, 50)],
            &[(50, 2)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_byte_ind_post_inc() {
        test_op_init(
            "MUL.B (AX+), 1",
            &[MUL_REG_NUM_BYTE, id::AX | IND_POST_INC, 2],
            &[(offset::AL, 50)],
            &[(50, 3)],
            &[(offset::AL, 51), (offset::FLAGS, INTERRUPTS)],
            &[(50, 6)],
        );
    }

    #[test]
    fn test_mul_reg_num_byte_ind_pre_inc() {
        test_op_init(
            "MUL.B (-AX), 1",
            &[MUL_REG_NUM_BYTE, id::AX | IND_PRE_DEC, 2],
            &[(offset::AL, 50)],
            &[(49, 10)],
            &[(offset::AL, 49), (offset::FLAGS, INTERRUPTS)],
            &[(49, 20)],
        );
    }

    #[test]
    fn test_mul_reg_num_word_ind() {
        test_op_init(
            "MUL.W (AX), x102",
            &[MUL_REG_NUM_WORD, id::AX | INDIRECT, 3, 2],
            &[(offset::AL, 50)],
            &[(50, 2)],
            &[(offset::FLAGS, CARRY | INTERRUPTS)],
            &[(50, 4)],
        );
    }

    #[test]
    fn test_mul_reg_num_word_ind_post_inc() {
        test_op_init(
            "MUL.W (AX+), 100",
            &[MUL_REG_NUM_WORD, id::AX | IND_POST_INC, 0, 100],
            &[(offset::AL, 50)],
            &[(50, 77)],
            &[(offset::AL, 52), (offset::FLAGS, CARRY | INTERRUPTS)],
            &[(50, 20)],
        );
    }

    #[test]
    fn test_mul_reg_num_word_ind_pre_inc() {
        test_op_init(
            "MUL.W (-AX), 1",
            &[MUL_REG_NUM_WORD, id::AX | IND_PRE_DEC, 0, 1],
            &[(offset::AL, 50)],
            &[(48, 9)],
            &[(offset::AL, 48), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }
}

mod indexed {
    use crate::offset;
    use crate::single::test_op_init;
    use maikor_platform::op_params::*;
    use maikor_platform::ops::*;
    use maikor_platform::registers::flags::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_mul_reg_num_byte_offset_num() {
        test_op_init(
            "MUL.B (AX+20), 1",
            &[MUL_REG_NUM_BYTE, id::AX | IND_OFFSET_NUM, 1, 0, 20],
            &[(offset::AL, 50)],
            &[(70, 2)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_mul_reg_num_byte_offset_reg() {
        test_op_init(
            "MUL.B (AX+CH), 1",
            &[MUL_REG_NUM_BYTE, id::AX | IND_OFFSET_REG, 2, id::CH],
            &[(offset::AL, 50), (offset::CH, 5)],
            &[(55, 2)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(55, 4)],
        );
    }
}
