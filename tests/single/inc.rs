use crate::offset;
use crate::single::{test_op, test_op_init};
use maikor_language::op_params::{INDIRECT, IND_POST_INC, IND_PRE_DEC, IND_PRE_INC};
use maikor_language::ops::{
    DEC_ADDR_BYTE, DEC_ADDR_WORD, DEC_REG_BYTE, DEC_REG_WORD, INC_ADDR_BYTE, INC_ADDR_WORD,
    INC_REG_BYTE, INC_REG_WORD,
};
use maikor_language::registers::flags::{INTERRUPTS, SIGNED};
use maikor_language::registers::id;

#[test]
fn inc_reg_byte() {
    test_op(
        &format!("INC.B AL"),
        &[INC_REG_BYTE, id::AL as u8],
        &[(offset::AL, 1), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op_init(
        &format!("INC.B CL"),
        &[INC_REG_BYTE, id::CL as u8],
        &[(offset::CL, 3)],
        &[],
        &[(offset::CL, 4), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op(
        &format!("INC.B (AX)"),
        &[INC_REG_BYTE, id::AX as u8 | INDIRECT],
        &[],
        &[(0, 1)],
    );
    test_op(
        &format!("INC.B -(AX)"),
        &[INC_REG_BYTE, id::AX as u8 | IND_PRE_DEC],
        &[(0, 255), (1, 254)],
        &[(65534, 1)],
    );
}

#[test]
fn inc_addr_byte() {
    test_op(
        &format!("INC.B $10"),
        &[INC_ADDR_BYTE, 0, 10],
        &[],
        &[(10, 1)],
    );
    test_op(
        &format!("INC.B $x256"),
        &[INC_ADDR_BYTE, 1, 0],
        &[],
        &[(256, 1)],
    );
}

#[test]
fn inc_addr_word() {
    test_op(
        &format!("INC.W $10"),
        &[INC_ADDR_WORD, 0, 10],
        &[],
        &[(11, 1)],
    );
    test_op(
        &format!("INC.W $x256"),
        &[INC_ADDR_WORD, 1, 0],
        &[],
        &[(257, 1)],
    );
}

#[test]
fn dec_addr_word() {
    test_op(
        &format!("DEC.W $10"),
        &[DEC_ADDR_WORD, 0, 10],
        &[],
        &[(10, 255), (11, 255)],
    );
    test_op(
        &format!("DEC.W $x256"),
        &[DEC_ADDR_WORD, 1, 0],
        &[],
        &[(256, 255), (257, 255)],
    );
}

#[test]
fn dec_addr_byte() {
    test_op(
        &format!("DEC.B $10"),
        &[DEC_ADDR_BYTE, 0, 10],
        &[],
        &[(10, 255)],
    );
    test_op(
        &format!("DEC.B $x256"),
        &[DEC_ADDR_BYTE, 1, 0],
        &[],
        &[(256, 255)],
    );
}

#[test]
fn dec_reg_byte() {
    test_op(
        &format!("DEC.B AL"),
        &[DEC_REG_BYTE, id::AL as u8],
        &[(offset::AL, 255), (offset::FLAGS, INTERRUPTS | SIGNED)],
        &[],
    );
    test_op_init(
        &format!("DEC.B DH"),
        &[DEC_REG_BYTE, id::DH as u8],
        &[(offset::DH, 56)],
        &[],
        &[(offset::DH, 55), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op(
        &format!("DEC.B (AX)+"),
        &[DEC_REG_BYTE, id::AX as u8 | IND_POST_INC],
        &[(1, 2)],
        &[(0, 255)],
    );
}

#[test]
fn inc_reg_word() {
    test_op(
        &format!("INC.W AX"),
        &[INC_REG_WORD, id::AX as u8],
        &[(offset::AL, 1), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op_init(
        &format!("INC.W CX"),
        &[INC_REG_WORD, id::CX as u8],
        &[(offset::CL, 3)],
        &[],
        &[(offset::CL, 4), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op(
        &format!("INC.W (DX)"),
        &[INC_REG_WORD, id::DX as u8 | INDIRECT],
        &[],
        &[(1, 1)],
    );
    test_op(
        &format!("INC.W +(DX)"),
        &[INC_REG_WORD, id::DX as u8 | IND_PRE_INC],
        &[(offset::DL, 2)],
        &[(3, 1)],
    );
}

#[test]
fn dec_reg_word() {
    test_op(
        &format!("DEC.W AX"),
        &[DEC_REG_WORD, id::AX as u8],
        &[
            (offset::AH, 255),
            (offset::AL, 255),
            (offset::FLAGS, INTERRUPTS | SIGNED),
        ],
        &[],
    );
    test_op_init(
        &format!("DEC.W DX"),
        &[DEC_REG_WORD, id::DX as u8],
        &[(offset::DL, 56)],
        &[],
        &[(offset::DL, 55), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
}
