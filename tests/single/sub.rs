use crate::offset;
use crate::single::{test_op, test_op_init};
use maikor_language::ops::*;
use maikor_language::registers::flags::*;
use maikor_language::registers::id;

#[test]
fn sub_addr_num_byte() {
    test_op(
        &format!("SUB.B $56 10"),
        &[SUB_ADDR_NUM_BYTE, 0, 56, 10],
        &[],
        &[(56, 246)],
    );
    test_op_init(
        &format!("SUB.B $x11 3"),
        &[SUB_ADDR_NUM_BYTE, 0, 17, 3],
        &[],
        &[(17, 15)],
        &[],
        &[(17, 12)],
    );
}

#[test]
fn sub_reg_num_byte() {
    test_op(
        &format!("SUB.B BH 10"),
        &[SUB_REG_NUM_BYTE, id::BH as u8, 10],
        &[
            (offset::BH, 246),
            (offset::FLAGS, INTERRUPTS | SIGNED | OVERFLOW | CARRY),
        ],
        &[],
    );
    test_op_init(
        &format!("SUB.B CH 3"),
        &[SUB_REG_NUM_BYTE, id::CH as u8, 3],
        &[(offset::CH, 15)],
        &[],
        &[(offset::CH, 12), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
}
