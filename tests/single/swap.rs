use crate::offset;
use crate::single::test_op_init;
use maikor_language::op_params::{INDIRECT, IND_POST_INC, IND_PRE_DEC};
use maikor_language::ops::*;
use maikor_language::registers::id;

#[test]
fn mcpy_byte() {
    test_op_init(
        "SWAP.B AL AH",
        &[SWAP_REG_REG_BYTE, id::AL as u8, id::AH as u8],
        &[(offset::AL, 15), (offset::AH, 100)],
        &[],
        &[(offset::AL, 100), (offset::AH, 15)],
        &[],
    );
    test_op_init(
        "SWAP.B AH AL",
        &[SWAP_REG_REG_BYTE, id::AH as u8, id::AL as u8],
        &[(offset::AL, 15), (offset::AH, 100)],
        &[],
        &[(offset::AL, 100), (offset::AH, 15)],
        &[],
    );
}

#[test]
fn mcpy_word() {
    test_op_init(
        "SWAP.W BX DX",
        &[SWAP_REG_REG_WORD, id::BX as u8, id::DX as u8],
        &[(offset::BH, 45), (offset::BL, 12), (offset::DH, 9)],
        &[],
        &[
            (offset::BH, 9),
            (offset::BL, 0),
            (offset::DH, 45),
            (offset::DL, 12),
        ],
        &[],
    );
}

#[test]
fn mcpy_byte_ind() {
    test_op_init(
        "SWAP.B (AX) (CX)",
        &[
            SWAP_REG_REG_BYTE,
            id::AX as u8 | INDIRECT,
            id::CX as u8 | INDIRECT,
        ],
        &[(offset::AL, 45), (offset::CL, 9)],
        &[(45, 100), (9, 20)],
        &[],
        &[(45, 20), (9, 100)],
    );
}

#[test]
fn mcpy_byte_ind_ppid() {
    test_op_init(
        "SWAP.W -(AX) (CX)+",
        &[
            SWAP_REG_REG_WORD,
            id::AX as u8 | IND_PRE_DEC,
            id::CX as u8 | IND_POST_INC,
        ],
        &[(offset::AL, 45), (offset::CL, 9)],
        &[(43, 100), (44, 101), (9, 20)],
        &[(offset::AL, 43), (offset::CL, 11)],
        &[(43, 20), (44, 0), (9, 100), (10, 101)],
    );
}
