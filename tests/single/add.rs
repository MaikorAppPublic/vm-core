mod positive_zero {
    use crate::offset;
    use crate::single::test_op;
    use maikor_platform::ops::*;
    use maikor_platform::registers::flags::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_add_reg_num_byte() {
        test_op(
            "ADD.B AH, 1",
            &[ADD_REG_NUM_BYTE, id::AH, 1],
            &[(offset::AH, 1), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_num_word() {
        test_op(
            "ADD.W AX, 1",
            &[ADD_REG_NUM_WORD, id::AX, 0, 1],
            &[(offset::AL, 1), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_num_word_word() {
        test_op(
            "ADD.W AX, x100",
            &[ADD_REG_NUM_WORD, id::AX, 1, 0],
            &[(offset::AH, 1), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_reg_byte() {
        test_op(
            "ADD.B AH, AL",
            &[ADD_REG_REG_BYTE, id::AH, id::AL],
            &[],
            &[],
        );
    }

    #[test]
    fn test_add_reg_reg_word() {
        test_op(
            "ADD.W AX, BX",
            &[ADD_REG_REG_WORD, id::AX, id::BX],
            &[],
            &[],
        );
    }

    #[test]
    fn test_add_addr_num_byte() {
        test_op("ADD.B $100, 0", &[ADD_ADDR_NUM_BYTE, 0, 100, 0], &[], &[]);
    }

    #[test]
    fn test_add_addr_num_word() {
        test_op(
            "ADD.W $100, 0",
            &[ADD_ADDR_NUM_WORD, 0, 100, 0, 0],
            &[],
            &[],
        );
    }

    #[test]
    fn test_add_addr_reg_byte() {
        test_op(
            "ADD.B $100, AH",
            &[ADD_ADDR_REG_BYTE, 0, 100, id::AH],
            &[],
            &[],
        );
    }

    #[test]
    fn test_add_addr_reg_word() {
        test_op(
            "ADD.W $100, AX",
            &[ADD_ADDR_REG_WORD, 0, 100, id::AX],
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
    fn test_add_reg_num_byte() {
        test_op_init(
            "ADD.B AH, 0",
            &[ADD_REG_NUM_BYTE, id::AH, 0],
            &[(offset::AH, 1)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_num_word() {
        test_op_init(
            "ADD.W AX, 0",
            &[ADD_REG_NUM_WORD, id::AX, 0, 0],
            &[(offset::AL, 1)],
            &[],
            &[(offset::AL, 1), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_num_word_word() {
        test_op_init(
            "ADD.W AX, x100",
            &[ADD_REG_NUM_WORD, id::AX, 0, 0],
            &[(offset::AH, 1), (offset::AL, 1)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_reg_byte() {
        test_op_init(
            "ADD.B AH, AL",
            &[ADD_REG_REG_BYTE, id::AH, id::AL],
            &[(offset::AH, 1)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_reg_word() {
        test_op_init(
            "ADD.W AX, BX",
            &[ADD_REG_REG_WORD, id::AX, id::BX],
            &[(offset::AL, 1)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_addr_num_byte() {
        test_op_init(
            "ADD.B $100, 0",
            &[ADD_ADDR_NUM_BYTE, 0, 100, 0],
            &[],
            &[(100, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_addr_num_word() {
        test_op_init(
            "ADD.W $100, 0",
            &[ADD_ADDR_NUM_WORD, 0, 100, 0, 0],
            &[],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_addr_num_word_word() {
        test_op_init(
            "ADD.W $100, x101",
            &[ADD_ADDR_NUM_WORD, 0, 100, 1, 1],
            &[],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 1), (101, 2)],
        );
    }

    #[test]
    fn test_add_addr_reg_byte() {
        test_op_init(
            "ADD.B $100, AH",
            &[ADD_ADDR_REG_BYTE, 0, 100, id::AH],
            &[],
            &[(100, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_addr_reg_word() {
        test_op_init(
            "ADD.W $100, AX",
            &[ADD_ADDR_REG_WORD, 0, 100, id::AX],
            &[],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
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
    fn test_add_reg_num_byte() {
        test_op_init(
            "ADD.B AH, 1",
            &[ADD_REG_NUM_BYTE, id::AH, 1],
            &[(offset::AH, 1)],
            &[],
            &[(offset::AH, 2), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_num_word() {
        test_op_init(
            "ADD.W AX, 1",
            &[ADD_REG_NUM_WORD, id::AX, 0, 1],
            &[(offset::AL, 1)],
            &[],
            &[(offset::AL, 2), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_reg_byte() {
        test_op_init(
            "ADD.B AH, AL",
            &[ADD_REG_REG_BYTE, id::AH, id::AL],
            &[(offset::AH, 1), (offset::AL, 1)],
            &[],
            &[(offset::AH, 2), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_reg_word() {
        test_op_init(
            "ADD.W AX, BX",
            &[ADD_REG_REG_WORD, id::AX, id::BX],
            &[(offset::AL, 1), (offset::BL, 1)],
            &[],
            &[(offset::AL, 2), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_addr_num_byte() {
        test_op_init(
            "ADD.B $100, 0",
            &[ADD_ADDR_NUM_BYTE, 0, 100, 1],
            &[],
            &[(100, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 2)],
        );
    }

    #[test]
    fn test_add_addr_num_word() {
        test_op_init(
            "ADD.W $100, 0",
            &[ADD_ADDR_NUM_WORD, 0, 100, 0, 0],
            &[],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_addr_reg_byte() {
        test_op_init(
            "ADD.B $100, AH",
            &[ADD_ADDR_REG_BYTE, 0, 100, id::AH],
            &[],
            &[(100, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_addr_reg_word() {
        test_op_init(
            "ADD.W $100, AX",
            &[ADD_ADDR_REG_WORD, 0, 100, id::AX],
            &[],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_addr_reg_word_word() {
        test_op_init(
            "ADD.W $100, AX",
            &[ADD_ADDR_REG_WORD, 0, 100, id::AX],
            &[(offset::AH, 1), (offset::AL, 15)],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 1), (101, 16)],
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
    fn test_add_reg_num_byte() {
        test_op_init(
            "ADD.B AH, 1",
            &[ADD_REG_NUM_BYTE, id::AH, 1],
            &[(offset::AH, 255)],
            &[],
            &[
                (offset::AH, 0),
                (offset::FLAGS, ZERO | INTERRUPTS | CARRY | OVERFLOW),
            ],
            &[],
        );
    }

    #[test]
    fn test_add_reg_num_word() {
        test_op_init(
            "ADD.W AX, 1",
            &[ADD_REG_NUM_WORD, id::AX, 0, 1],
            &[(offset::AH, 255), (offset::AL, 255)],
            &[],
            &[
                (offset::AH, 0),
                (offset::AL, 0),
                (offset::FLAGS, ZERO | INTERRUPTS | CARRY | OVERFLOW),
            ],
            &[],
        );
    }

    #[test]
    fn test_add_reg_num_word_word() {
        test_op_init(
            "ADD.W AX, xFF01",
            &[ADD_REG_NUM_WORD, id::AX, 255, 1],
            &[(offset::AL, 255)],
            &[],
            &[
                (offset::AH, 0),
                (offset::AL, 0),
                (offset::FLAGS, ZERO | INTERRUPTS | CARRY),
            ],
            &[],
        );
    }

    #[test]
    fn test_add_reg_reg_byte() {
        test_op_init(
            "ADD.B AH, AL",
            &[ADD_REG_REG_BYTE, id::AH, id::AL],
            &[(offset::AH, 240), (offset::AL, 17)],
            &[],
            &[
                (offset::AH, 1),
                (offset::FLAGS, CARRY | OVERFLOW | INTERRUPTS),
            ],
            &[],
        );
    }

    #[test]
    fn test_add_reg_reg_word() {
        test_op_init(
            "ADD.W AX, BX",
            &[ADD_REG_REG_WORD, id::AX, id::BX],
            &[
                (offset::AH, 200),
                (offset::AL, 200),
                (offset::BH, 100),
                (offset::BL, 100),
            ],
            &[],
            &[
                (offset::AH, 45),
                (offset::AL, 44),
                (offset::FLAGS, CARRY | OVERFLOW | INTERRUPTS),
            ],
            &[],
        );
    }

    #[test]
    fn test_add_addr_num_byte() {
        test_op_init(
            "ADD.B $100, 241",
            &[ADD_ADDR_NUM_BYTE, 0, 100, 241],
            &[],
            &[(100, 255)],
            &[(offset::FLAGS, CARRY | SIGNED | INTERRUPTS)],
            &[(100, 240)],
        );
    }

    #[test]
    fn test_add_addr_num_word() {
        test_op_init(
            "ADD.W $100, 1",
            &[ADD_ADDR_NUM_WORD, 0, 100, 0, 1],
            &[],
            &[(100, 255), (101, 255)],
            &[(offset::FLAGS, ZERO | CARRY | OVERFLOW | INTERRUPTS)],
            &[(100, 0), (101, 0)],
        );
    }

    #[test]
    fn test_add_addr_num_word_word() {
        test_op_init(
            "ADD.W $100, x101",
            &[ADD_ADDR_NUM_WORD, 0, 100, 1, 1],
            &[],
            &[(101, 1)],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(100, 1), (101, 2)],
        );
    }

    #[test]
    fn test_add_addr_reg_byte() {
        test_op_init(
            "ADD.B $100, AH",
            &[ADD_ADDR_REG_BYTE, 0, 100, id::AH],
            &[(offset::AH, 20)],
            &[(100, 255)],
            &[(offset::FLAGS, INTERRUPTS | CARRY | OVERFLOW)],
            &[(100, 19)],
        );
    }

    #[test]
    fn test_add_addr_reg_word() {
        test_op_init(
            "ADD.W $100, AX",
            &[ADD_ADDR_REG_WORD, 0, 100, id::AX],
            &[(offset::AH, 100), (offset::AL, 100)],
            &[(100, 164), (101, 255)],
            &[(offset::FLAGS, INTERRUPTS | CARRY | OVERFLOW)],
            &[(100, 9), (101, 99)],
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
    fn test_add_reg_num_pre_inc() {
        test_op_init(
            "ADD.B +AL, 1",
            &[ADD_REG_NUM_BYTE, id::AL | PRE_INC, 1],
            &[(offset::AL, 50)],
            &[],
            &[(offset::AL, 52), (offset::FLAGS, INTERRUPTS)],
            &[],
        );
    }

    #[test]
    fn test_add_reg_num_byte_ind() {
        test_op_init(
            "ADD.B (AX), 1",
            &[ADD_REG_NUM_BYTE, id::AX | INDIRECT, 1],
            &[(offset::AL, 50)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(50, 1)],
        );
    }

    #[test]
    fn test_add_reg_num_byte_ind_post_inc() {
        test_op_init(
            "ADD.B (AX+), 1",
            &[ADD_REG_NUM_BYTE, id::AX | IND_POST_INC, 1],
            &[(offset::AL, 50)],
            &[],
            &[(offset::AL, 51), (offset::FLAGS, INTERRUPTS)],
            &[(50, 1)],
        );
    }

    #[test]
    fn test_add_reg_num_byte_ind_pre_inc() {
        test_op_init(
            "ADD.B (-AX), 1",
            &[ADD_REG_NUM_BYTE, id::AX | IND_PRE_DEC, 1],
            &[(offset::AL, 50)],
            &[],
            &[(offset::AL, 49), (offset::FLAGS, INTERRUPTS)],
            &[(49, 1)],
        );
    }

    #[test]
    fn test_add_reg_num_word_ind() {
        test_op_init(
            "ADD.W (AX), x102",
            &[ADD_REG_NUM_WORD, id::AX | INDIRECT, 1, 2],
            &[(offset::AL, 50)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(50, 1), (51, 2)],
        );
    }

    #[test]
    fn test_add_reg_num_word_ind_post_inc() {
        test_op_init(
            "ADD.W (AX+), 1",
            &[ADD_REG_NUM_WORD, id::AX | IND_POST_INC, 1, 0],
            &[(offset::AL, 50)],
            &[],
            &[(offset::AL, 52), (offset::FLAGS, INTERRUPTS)],
            &[(50, 1)],
        );
    }

    #[test]
    fn test_add_reg_num_word_ind_pre_inc() {
        test_op_init(
            "ADD.W (-AX), 1",
            &[ADD_REG_NUM_WORD, id::AX | IND_PRE_DEC, 0, 1],
            &[(offset::AL, 50)],
            &[],
            &[(offset::AL, 48), (offset::FLAGS, INTERRUPTS)],
            &[(49, 1)],
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
    fn test_add_reg_num_byte_offset_num() {
        test_op_init(
            "ADD.B (AX+20), 1",
            &[ADD_REG_NUM_BYTE, id::AX | IND_OFFSET_NUM, 1, 0, 20],
            &[(offset::AL, 50)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(70, 1)],
        );
    }

    #[test]
    fn test_add_reg_num_byte_offset_reg() {
        test_op_init(
            "ADD.B (AX+CH), 1",
            &[ADD_REG_NUM_BYTE, id::AX | IND_OFFSET_REG, 1, id::CH],
            &[(offset::AL, 50), (offset::CH, 5)],
            &[],
            &[(offset::FLAGS, INTERRUPTS)],
            &[(55, 1)],
        );
    }
}
