use crate::offset;
use crate::single::{test_op, test_op_init};
use maikor_language::ops::*;
use maikor_language::registers::flags::*;
use maikor_language::registers::id;

#[test]
fn mul_reg_num_byte() {
    test_op(
        "MUL.B AX 10",
        &[MUL_REG_NUM_BYTE, id::AX as u8, 10],
        &[(offset::FLAGS, INTERRUPTS | ZERO)],
        &[],
    );
    test_op_init(
        "SUB.B AX 3",
        &[MUL_REG_NUM_BYTE, id::AX as u8, 3],
        &[(offset::AL, 2)],
        &[],
        &[(offset::AL, 6), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
}

#[test]
fn mul_reg_reg_byte() {
    test_op(
        "MUL.B AX AL",
        &[MUL_REG_NUM_BYTE, id::AX as u8, 10],
        &[(offset::FLAGS, INTERRUPTS | ZERO)],
        &[],
    );
    test_op_init(
        "SUB.B AX 3",
        &[MUL_REG_NUM_BYTE, id::AX as u8, 3],
        &[(offset::AL, 2)],
        &[],
        &[(offset::AL, 6), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
}
