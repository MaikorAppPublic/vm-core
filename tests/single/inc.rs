use crate::offset;
use crate::single::{test_op, test_op_init};
use maikor_platform::op_params::{
    INDIRECT, IND_OFFSET_EXT_REG, IND_OFFSET_NUM, IND_OFFSET_REG, IND_POST_INC, IND_PRE_DEC,
    IND_PRE_INC,
};
use maikor_platform::ops::{
    DEC_ADDR_BYTE, DEC_ADDR_WORD, DEC_REG_BYTE, DEC_REG_WORD, INC_ADDR_BYTE, INC_ADDR_WORD,
    INC_REG_BYTE, INC_REG_WORD,
};
use maikor_platform::registers::id;

#[test]
fn inc_reg_byte() {
    test_op(
        "INC.B AL",
        &[INC_REG_BYTE, id::AL as u8],
        &[(offset::AL, 1)],
        &[],
    );
    test_op_init(
        "INC.B CL",
        &[INC_REG_BYTE, id::CL as u8],
        &[(offset::CL, 3)],
        &[],
        &[(offset::CL, 4)],
        &[],
    );
    test_op(
        "INC.B (AX)",
        &[INC_REG_BYTE, id::AX as u8 | INDIRECT],
        &[],
        &[(0, 1)],
    );
    test_op(
        "INC.B -(AX)",
        &[INC_REG_BYTE, id::AX as u8 | IND_PRE_DEC],
        &[(0, 255), (1, 254)],
        &[(65534, 1)],
    );
}

#[test]
fn inc_addr_byte() {
    test_op("INC.B $10", &[INC_ADDR_BYTE, 0, 10], &[], &[(10, 1)]);
    test_op("INC.B $x256", &[INC_ADDR_BYTE, 1, 0], &[], &[(256, 1)]);
}

#[test]
fn inc_addr_word() {
    test_op("INC.W $10", &[INC_ADDR_WORD, 0, 10], &[], &[(11, 1)]);
    test_op("INC.W $x256", &[INC_ADDR_WORD, 1, 0], &[], &[(257, 1)]);
}

#[test]
fn dec_addr_word() {
    test_op(
        "DEC.W $10",
        &[DEC_ADDR_WORD, 0, 10],
        &[],
        &[(10, 255), (11, 255)],
    );
    test_op(
        "DEC.W $x256",
        &[DEC_ADDR_WORD, 1, 0],
        &[],
        &[(256, 255), (257, 255)],
    );
}

#[test]
fn dec_addr_byte() {
    test_op("DEC.B $10", &[DEC_ADDR_BYTE, 0, 10], &[], &[(10, 255)]);
    test_op("DEC.B $x256", &[DEC_ADDR_BYTE, 1, 0], &[], &[(256, 255)]);
}

#[test]
fn dec_reg_byte() {
    test_op(
        "DEC.B AL",
        &[DEC_REG_BYTE, id::AL as u8],
        &[(offset::AL, 255)],
        &[],
    );
    test_op_init(
        "DEC.B DH",
        &[DEC_REG_BYTE, id::DH as u8],
        &[(offset::DH, 56)],
        &[],
        &[(offset::DH, 55)],
        &[],
    );
    test_op(
        "DEC.B (AX)+",
        &[DEC_REG_BYTE, id::AX as u8 | IND_POST_INC],
        &[(1, 2)],
        &[(0, 255)],
    );
}

#[test]
fn inc_reg_word() {
    test_op(
        "INC.W AX",
        &[INC_REG_WORD, id::AX as u8],
        &[(offset::AL, 1)],
        &[],
    );
    test_op_init(
        "INC.W CX",
        &[INC_REG_WORD, id::CX as u8],
        &[(offset::CL, 3)],
        &[],
        &[(offset::CL, 4)],
        &[],
    );
    test_op(
        "INC.W (DX)",
        &[INC_REG_WORD, id::DX as u8 | INDIRECT],
        &[],
        &[(1, 1)],
    );
    test_op(
        "INC.W +(DX)",
        &[INC_REG_WORD, id::DX as u8 | IND_PRE_INC],
        &[(offset::DL, 2)],
        &[(3, 1)],
    );
}

#[test]
fn dec_reg_word() {
    test_op(
        "DEC.W AX",
        &[DEC_REG_WORD, id::AX as u8],
        &[(offset::AH, 255), (offset::AL, 255)],
        &[],
    );
    test_op_init(
        "DEC.W DX",
        &[DEC_REG_WORD, id::DX as u8],
        &[(offset::DL, 56)],
        &[],
        &[(offset::DL, 55)],
        &[],
    );
}

#[test]
fn inc_complex() {
    test_op_init(
        "INC.B (AX+150)",
        &[INC_REG_BYTE, id::AX as u8 | IND_OFFSET_NUM, 0, 150],
        &[(offset::AL, 100)],
        &[(100, 12), (250, 88)],
        &[],
        &[(250, 89)],
    );

    test_op_init(
        "INC.B (AX+CX)",
        &[
            INC_REG_BYTE,
            id::AX as u8 | IND_OFFSET_EXT_REG,
            id::CX as u8,
        ],
        &[(offset::AL, 100), (offset::CH, 1), (offset::CL, 255)],
        &[(100, 12), (611, 78)],
        &[],
        &[(611, 79)],
    );
}

#[test]
fn dec_complex() {
    test_op_init(
        "DEC.B (AX+150)",
        &[DEC_REG_BYTE, id::AX as u8 | IND_OFFSET_NUM, 1, 0],
        &[(offset::AL, 100)],
        &[(100, 12), (356, 88)],
        &[],
        &[(356, 87)],
    );

    test_op_init(
        "DEC.B (AX+cl)",
        &[DEC_REG_BYTE, id::AX as u8 | IND_OFFSET_REG, id::CL as u8],
        &[(offset::AL, 100), (offset::CH, 1), (offset::CL, 255)],
        &[(100, 12), (355, 99), (611, 78)],
        &[],
        &[(355, 98)],
    );
}
