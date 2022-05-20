use crate::offset;
use crate::single::{test_op, test_op_init};
use maikor_platform::ops::*;
use maikor_platform::registers::flags::*;
use maikor_platform::registers::id;

#[test]
fn mul_reg_num_byte() {
    test_op(
        "MUL.B AH 10",
        &[MUL_REG_NUM_BYTE, id::AH as u8, 10],
        &[(offset::FLAGS, INTERRUPTS | ZERO)],
        &[],
    );
    test_op_init(
        "MUL.B AL 3",
        &[MUL_REG_NUM_BYTE, id::AL as u8, 3],
        &[(offset::AL, 2)],
        &[],
        &[(offset::AL, 6), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
}

#[test]
fn mul_reg_reg_byte() {
    test_op(
        "MUL.B AH AL",
        &[MUL_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[(offset::FLAGS, INTERRUPTS | ZERO)],
        &[],
    );
    test_op_init(
        "MUL.B AH AL",
        &[MUL_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[(offset::AL, 2), (offset::AH, 3)],
        &[],
        &[(offset::AH, 6), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
}

#[test]
fn mul_reg_num_word() {
    test_op(
        "MUL.W AX 10",
        &[MUL_REG_NUM_WORD, id::AH as u8, 0, 10],
        &[(offset::FLAGS, INTERRUPTS | ZERO)],
        &[],
    );
    test_op_init(
        "MUL.B AX 3",
        &[MUL_REG_NUM_WORD, id::AX as u8, 0, 3],
        &[(offset::AL, 2)],
        &[],
        &[(offset::AL, 6), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
}

#[test]
fn mul_reg_reg_word() {
    test_op(
        "MUL.W AX BX",
        &[MUL_REG_NUM_WORD, id::AX as u8, id::BX as u8],
        &[(offset::FLAGS, INTERRUPTS | ZERO)],
        &[],
    );
    test_op_init(
        "MUL.W AX BX",
        &[MUL_REG_NUM_WORD, id::AX as u8, id::BX as u8],
        &[(offset::AL, 2), (offset::BH, 255)],
        &[],
        &[
            (offset::AH, 20),
            (offset::AL, 0),
            (offset::FLAGS, INTERRUPTS),
        ],
        &[],
    );
}
