mod positive {
    use crate::offset;
    use crate::single::test_op;
    use maikor_platform::ops::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_dec_reg_byte() {
        test_op(
            "DEC.B AL",
            &[DEC_REG_BYTE, id::AL],
            &[(offset::AL, 255)],
            &[],
        );
    }

    #[test]
    fn test_dec_reg_word() {
        test_op(
            "DEC.W BX",
            &[DEC_REG_WORD, id::BX],
            &[(offset::BH, 255), (offset::BL, 255)],
            &[],
        );
    }

    #[test]
    fn test_dec_addr_byte() {
        test_op("DEC.B $50", &[DEC_ADDR_BYTE, 0, 50], &[], &[(50, 255)]);
    }

    #[test]
    fn test_dec_addr_word() {
        test_op(
            "DEC.W $50",
            &[DEC_ADDR_WORD, 0, 50],
            &[],
            &[(50, 255), (51, 255)],
        );
    }
}

mod ppid {
    use crate::offset;
    use crate::single::test_op;
    use maikor_platform::op_params::{INDIRECT, IND_POST_DEC};
    use maikor_platform::ops::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_dec_reg_byte() {
        test_op(
            "DEC.B (BX)",
            &[DEC_REG_BYTE, id::BX | INDIRECT],
            &[],
            &[(0, 255)],
        );
    }

    #[test]
    fn test_dec_addr_word() {
        test_op(
            "DEC.W (AX+)",
            &[DEC_REG_WORD, id::AX | IND_POST_DEC],
            &[(offset::AH, 255), (offset::AL, 254)],
            &[(0, 255), (1, 255)],
        );
    }
}

mod indexed {
    use crate::single::test_op;
    use maikor_platform::op_params::IND_OFFSET_NUM;
    use maikor_platform::ops::*;
    use maikor_platform::registers::id;

    #[test]
    fn test_dec_reg_word() {
        test_op(
            "DEC.W (BX+56)",
            &[DEC_REG_WORD, id::BX | IND_OFFSET_NUM, 0, 56],
            &[],
            &[(56, 255), (57, 255)],
        );
    }
}
