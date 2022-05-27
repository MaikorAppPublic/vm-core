use crate::offset;
use crate::single::{test_op, test_op_init};
use maikor_platform::op_params::{INDIRECT, IND_POST_INC};
use maikor_platform::ops::*;
use maikor_platform::registers::flags::*;
use maikor_platform::registers::id;

#[test]
fn addc_reg_num_byte() {
    test_op(
        "ADDC.B BH, 10",
        &[ADDC_REG_NUM_BYTE, id::BH as u8, 10],
        &[(offset::BH, 10), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op_init(
        "ADDC.B CH, 15",
        &[ADDC_REG_NUM_BYTE, id::CH as u8, 15],
        &[(offset::CH, 3)],
        &[],
        &[(offset::CH, 18), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op_init(
        "ADDC.B CH, 15",
        &[ADDC_REG_NUM_BYTE, id::CH as u8, 255],
        &[(offset::CH, 1)],
        &[],
        &[(offset::CH, 0), (offset::FLAGS, INTERRUPTS | CARRY | ZERO)],
        &[],
    );
    test_op_init(
        "ADDC.B DH, 200",
        &[ADDC_REG_NUM_BYTE, id::DH as u8, 200],
        &[(offset::DH, 100)],
        &[],
        &[(offset::DH, 44), (offset::FLAGS, INTERRUPTS | CARRY)],
        &[],
    );
    test_op_init(
        "ADDC.B (DX), 1",
        &[ADDC_REG_NUM_BYTE, id::DX as u8 | INDIRECT, 1],
        &[(offset::DH, 15), (offset::DL, 15)],
        &[(3855, 255)],
        &[
            (offset::DH, 15),
            (offset::DL, 15),
            (offset::FLAGS, INTERRUPTS | CARRY | ZERO | OVERFLOW),
        ],
        &[(3855, 0)],
    );
    test_op_init(
        "ADDC.B (DX)+, 200",
        &[ADDC_REG_NUM_BYTE, id::DX as u8 | IND_POST_INC, 200],
        &[(offset::DH, 15), (offset::DL, 10)],
        &[(3855, 255)],
        &[
            (offset::DH, 15),
            (offset::DL, 12),
            (offset::FLAGS, INTERRUPTS | SIGNED | OVERFLOW),
        ],
        &[(3850, 200)],
    );

    test_op_init(
        "ADDC.B BH, 10",
        &[ADDC_REG_NUM_BYTE, id::BH as u8, 10],
        &[(offset::FLAGS, INTERRUPTS | CARRY)],
        &[],
        &[(offset::BH, 11), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op_init(
        "ADDC.B CH, 15",
        &[ADDC_REG_NUM_BYTE, id::CH as u8, 15],
        &[(offset::CH, 3), (offset::FLAGS, INTERRUPTS | CARRY)],
        &[],
        &[(offset::CH, 19), (offset::FLAGS, INTERRUPTS)],
        &[],
    );
    test_op_init(
        "ADDC.B CH, 15",
        &[ADDC_REG_NUM_BYTE, id::CH as u8, 255],
        &[(offset::CH, 1), (offset::FLAGS, INTERRUPTS | CARRY)],
        &[],
        &[(offset::CH, 1), (offset::FLAGS, CARRY | INTERRUPTS)],
        &[],
    );
    test_op_init(
        "ADDC.B DH, 200",
        &[ADDC_REG_NUM_BYTE, id::DH as u8, 200],
        &[(offset::DH, 100), (offset::FLAGS, INTERRUPTS | CARRY)],
        &[],
        &[(offset::DH, 45), (offset::FLAGS, INTERRUPTS | CARRY)],
        &[],
    );
    test_op_init(
        "ADDC.B (DX), 1",
        &[ADDC_REG_NUM_BYTE, id::DX as u8 | INDIRECT, 1],
        &[
            (offset::DH, 15),
            (offset::DL, 15),
            (offset::FLAGS, INTERRUPTS | CARRY),
        ],
        &[(3855, 255)],
        &[
            (offset::DH, 15),
            (offset::DL, 15),
            (offset::FLAGS, INTERRUPTS | CARRY | OVERFLOW),
        ],
        &[(3855, 1)],
    );
    test_op_init(
        "ADDC.B (DX)+, 200",
        &[ADDC_REG_NUM_BYTE, id::DX as u8 | IND_POST_INC, 200],
        &[
            (offset::DH, 15),
            (offset::DL, 10),
            (offset::FLAGS, INTERRUPTS | CARRY),
        ],
        &[(3855, 255)],
        &[
            (offset::DH, 15),
            (offset::DL, 12),
            (offset::FLAGS, INTERRUPTS | SIGNED | OVERFLOW),
        ],
        &[(3850, 201)],
    );
}

#[test]
fn addc_addr_num_byte() {
    test_op(
        "ADDC.B $45, 10",
        &[ADDC_ADDR_NUM_BYTE, 0, 45, 10],
        &[(offset::FLAGS, INTERRUPTS)],
        &[(45, 10)],
    );
    test_op_init(
        "ADDC.B $xF, 15",
        &[ADDC_ADDR_NUM_BYTE, 0, 15, 15],
        &[],
        &[(15, 3)],
        &[(offset::FLAGS, INTERRUPTS)],
        &[(15, 18)],
    );

    test_op_init(
        "ADDC.B $45, 10",
        &[ADDC_ADDR_NUM_BYTE, 0, 45, 10],
        &[(offset::FLAGS, CARRY | INTERRUPTS)],
        &[],
        &[(offset::FLAGS, INTERRUPTS)],
        &[(45, 11)],
    );
    test_op_init(
        "ADDC.B $xF, 15",
        &[ADDC_ADDR_NUM_BYTE, 0, 15, 15],
        &[(offset::FLAGS, CARRY | INTERRUPTS)],
        &[(15, 3)],
        &[(offset::FLAGS, INTERRUPTS)],
        &[(15, 19)],
    );
}

#[test]
fn addc_reg_reg_byte() {
    test_op_init(
        "ADDC.B AH, AL",
        &[ADDC_REG_REG_BYTE, id::AH as u8, id::AL as u8],
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
        "ADDC.B AH, AL",
        &[ADDC_REG_REG_BYTE, id::AH as u8, id::AL as u8],
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
        "ADDC.B AH, AL",
        &[ADDC_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[(offset::AH, 160), (offset::AL, 160)],
        &[],
        &[
            (offset::AH, 64),
            (offset::AL, 160),
            (offset::FLAGS, INTERRUPTS | CARRY | OVERFLOW),
        ],
        &[],
    );

    test_op_init(
        "ADDC.B AH, AL",
        &[ADDC_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[(offset::AL, 2), (offset::FLAGS, CARRY | INTERRUPTS)],
        &[],
        &[
            (offset::AH, 3),
            (offset::AL, 2),
            (offset::FLAGS, INTERRUPTS),
        ],
        &[],
    );
    test_op_init(
        "ADDC.B AH, AL",
        &[ADDC_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[
            (offset::AH, 1),
            (offset::AL, 2),
            (offset::FLAGS, CARRY | INTERRUPTS),
        ],
        &[],
        &[
            (offset::AH, 4),
            (offset::AL, 2),
            (offset::FLAGS, INTERRUPTS),
        ],
        &[],
    );
    test_op_init(
        "ADDC.B AH, AL",
        &[ADDC_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[
            (offset::AH, 160),
            (offset::AL, 160),
            (offset::FLAGS, CARRY | INTERRUPTS),
        ],
        &[],
        &[
            (offset::AH, 65),
            (offset::AL, 160),
            (offset::FLAGS, INTERRUPTS | CARRY | OVERFLOW),
        ],
        &[],
    );
}

#[test]
fn addc_addr_reg_byte() {
    test_op_init(
        "ADDC.B $45, AL",
        &[ADDC_ADDR_REG_BYTE, 0, 45, id::CL as u8],
        &[(offset::CL, 2)],
        &[],
        &[(offset::CL, 2), (offset::FLAGS, INTERRUPTS)],
        &[(45, 2)],
    );
    test_op_init(
        "ADDC.B $xFF, AL",
        &[ADDC_ADDR_REG_BYTE, 0, 255, id::AL as u8],
        &[(offset::AL, 2)],
        &[(255, 10)],
        &[(offset::AL, 2), (offset::FLAGS, INTERRUPTS)],
        &[(255, 12)],
    );

    test_op_init(
        "ADDC.B $45, AL",
        &[ADDC_ADDR_REG_BYTE, 0, 45, id::CL as u8],
        &[(offset::CL, 2), (offset::FLAGS, CARRY | INTERRUPTS)],
        &[],
        &[(offset::CL, 2), (offset::FLAGS, INTERRUPTS)],
        &[(45, 3)],
    );
    test_op_init(
        "ADDC.B $xFF, AL",
        &[ADDC_ADDR_REG_BYTE, 0, 255, id::AL as u8],
        &[(offset::AL, 2), (offset::FLAGS, CARRY | INTERRUPTS)],
        &[(255, 10)],
        &[(offset::AL, 2), (offset::FLAGS, INTERRUPTS)],
        &[(255, 13)],
    );
}
