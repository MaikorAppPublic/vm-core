mod positive {
    use crate::offset;
    use crate::single::test_op;
    use maikor_platform::ops::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_inc_reg_byte() {
        test_op("INC.B AL", &[INC_REG_BYTE, id::AL], &[(offset::AL, 1)], &[]);
    }

    #[test]
    fn test_inc_reg_word() {
        test_op("INC.W BX", &[INC_REG_WORD, id::BX], &[(offset::BL, 1)], &[]);
    }

    #[test]
    fn test_inc_addr_byte() {
        test_op("INC.B $50", &[INC_ADDR_BYTE, 0, 50], &[], &[(50, 1)]);
    }

    #[test]
    fn test_inc_addr_word() {
        test_op("INC.W $50", &[INC_ADDR_WORD, 0, 50], &[], &[(51, 1)]);
    }
}

mod ppid {
    use crate::offset;
    use crate::single::test_op;
    use maikor_platform::op_params::{INDIRECT, IND_POST_INC};
    use maikor_platform::ops::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_inc_reg_byte() {
        test_op(
            "INC.B (BX)",
            &[INC_REG_BYTE, id::BX | INDIRECT],
            &[],
            &[(0, 1)],
        );
    }

    #[test]
    fn test_inc_addr_word() {
        test_op(
            "INC.W (AX+)",
            &[INC_REG_WORD, id::AX | IND_POST_INC],
            &[(offset::AL, 2)],
            &[(1, 1)],
        );
    }
}

mod indexed {
    use crate::single::test_op;
    use maikor_platform::op_params::IND_OFFSET_NUM;
    use maikor_platform::ops::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_inc_reg_word() {
        test_op(
            "INC.W (BX+56)",
            &[INC_REG_WORD, id::BX | IND_OFFSET_NUM, 0, 56],
            &[],
            &[(57, 1)],
        );
    }
}
