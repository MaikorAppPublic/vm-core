use crate::offset;
use crate::single::{test_op, test_op_init};
use maikor_language::op_params::{INDIRECT, IND_POST_INC};
use maikor_language::ops::*;
use maikor_language::registers::flags::*;
use maikor_language::registers::id;

#[test]
fn add_reg_num_byte() {
    test_op(
        &format!("ADD.B BH 10"),
        &[ADD_REG_NUM_BYTE, id::BH as u8, 10],
        &[(offset::BH, 10), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op_init(
        &format!("ADD.B CH 15"),
        &[ADD_REG_NUM_BYTE, id::CH as u8, 15],
        &[(offset::CH, 3)],
        &[],
        &[(offset::CH, 18), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op_init(
        &format!("ADD.B DH 200"),
        &[ADD_REG_NUM_BYTE, id::DH as u8, 200],
        &[(offset::DH, 100)],
        &[],
        &[(offset::DH, 44), (offset::FLAGS, INTERRUPTS | CARRY)],
        &[],
    );
    test_op_init(
        &format!("ADD.B (DX) 1"),
        &[ADD_REG_NUM_BYTE, id::DX as u8 | INDIRECT, 1],
        &[(offset::DH, 15), (offset::DL, 15)],
        &[(3855, 255)],
        &[
            (offset::DH, 15),
            (offset::DL, 15),
            (offset::FLAGS, INTERRUPTS | ZERO),
        ],
        &[(3855, 0)],
    );
    test_op_init(
        &format!("ADD.B (DX)+ 200"),
        &[ADD_REG_NUM_BYTE, id::DX as u8 | IND_POST_INC, 200],
        &[(offset::DH, 15), (offset::DL, 10)],
        &[(3855, 255)],
        &[
            (offset::DH, 15),
            (offset::DL, 12),
            (offset::FLAGS, INTERRUPTS | ZERO),
        ],
        &[(3850, 200)],
    );
}

#[test]
fn add_addr_num_byte() {
    test_op(
        &format!("ADD.B $45 10"),
        &[ADD_ADDR_NUM_BYTE, 0, 45, 10],
        &[],
        &[(45, 10)],
    );
    test_op_init(
        &format!("ADD.B $xF 15"),
        &[ADD_ADDR_NUM_BYTE, 0, 15, 15],
        &[],
        &[(15, 3)],
        &[],
        &[(15, 18)],
    );
}

#[test]
fn add_reg_reg_byte() {
    test_op_init(
        &format!("ADD.B AH AL"),
        &[ADD_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[(offset::AL, 2)],
        &[],
        &[
            (offset::AH, 2),
            (offset::AL, 2),
            (offset::FLAGS, INTERRUPTS),
        ],
        &[],
    );
    test_op_init(
        &format!("ADD.B AH AL"),
        &[ADD_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[(offset::AH, 1), (offset::AL, 2)],
        &[],
        &[
            (offset::AH, 3),
            (offset::AL, 2),
            (offset::FLAGS, INTERRUPTS),
        ],
        &[],
    );
    test_op_init(
        &format!("ADD.B AH AL"),
        &[ADD_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[(offset::AH, 160), (offset::AL, 160)],
        &[],
        &[
            (offset::AH, 64),
            (offset::AL, 160),
            (offset::FLAGS, INTERRUPTS | CARRY | OVERFLOW),
        ],
        &[],
    );
}

#[test]
fn add_addr_reg_byte() {
    test_op_init(
        &format!("ADD.B $45 AL"),
        &[ADD_ADDR_REG_BYTE, 0, 45, id::CL as u8],
        &[(offset::CL, 2)],
        &[],
        &[(offset::CL, 2), (offset::FLAGS, INTERRUPTS | ZERO)],
        &[(45, 2)],
    );
    test_op_init(
        &format!("ADD.B $xFF AL"),
        &[ADD_ADDR_REG_BYTE, 0, 255, id::AL as u8],
        &[(offset::AL, 2)],
        &[(255, 10)],
        &[(offset::AL, 2), (offset::FLAGS, INTERRUPTS | ZERO)],
        &[(255, 12)],
    );
}
