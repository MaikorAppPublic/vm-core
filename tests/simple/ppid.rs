//Pre|Post Inc|Dec

use crate::{direct, make_reg, reg_delta, setup_vm, test_single_op, Registers};
use vm_core::constants::op_params::values;
use vm_core::constants::ops::{CPY_REG_REG_BYTE, INC_REG_BYTE};
use vm_core::constants::registers::{id, offset};

#[test]
fn test_ppid() {
    test_with_inc_b();
    test_with_cpy_b();
}

fn test_with_inc_b() {
    let name = "INC.B";
    let prei_al = make_reg(id::AL, values::PRE_INC);
    let posti_al = make_reg(id::AL, values::POST_INC);
    let pred_al = make_reg(id::AL, values::PRE_DEC);
    let postd_al = make_reg(id::AL, values::POST_DEC);
    let mut vm = setup_vm();
    let mut registers = vm.registers;
    let list: Vec<(&str, Vec<u8>, fn(&mut Registers) -> Registers)> = vec![
        ("+R", vec![INC_REG_BYTE, prei_al], |r| {
            reg_delta(r, offset::AL, 2)
        }),
        ("R+", vec![INC_REG_BYTE, posti_al], |r| {
            reg_delta(r, offset::AL, 4)
        }),
        ("-R", vec![INC_REG_BYTE, pred_al], |r| {
            reg_delta(r, offset::AL, 4)
        }),
        ("R-", vec![INC_REG_BYTE, postd_al], |r| {
            reg_delta(r, offset::AL, 4)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op(&mut vm, i, name, op.0, &op.1, op.2(&mut registers))
    }
}

fn test_with_cpy_b() {
    let name = "CPY.B";
    let prei_al = make_reg(id::AL, values::PRE_INC);
    let posti_al = make_reg(id::AL, values::POST_INC);
    let pred_al = make_reg(id::AL, values::PRE_DEC);
    let postd_al = make_reg(id::AL, values::POST_DEC);
    let prei_bl = make_reg(id::BL, values::PRE_INC);
    let posti_bl = make_reg(id::BL, values::POST_INC);
    let pred_bl = make_reg(id::BL, values::PRE_DEC);
    let postd_bl = make_reg(id::BL, values::POST_DEC);
    let mut vm = setup_vm();
    vm.registers[offset::AL] = 4;
    let mut registers = vm.registers;
    let list: Vec<(&str, Vec<u8>, fn(&mut Registers) -> Registers)> = vec![
        //first direct, second changes
        ("R,+R", vec![CPY_REG_REG_BYTE, direct::BL, prei_al], |r| {
            reg_delta(r, offset::BL, 5);
            reg_delta(r, offset::AL, 5)
        }),
        ("R,-R", vec![CPY_REG_REG_BYTE, direct::BL, pred_al], |r| {
            reg_delta(r, offset::BL, 4);
            reg_delta(r, offset::AL, 4)
        }),
        ("R,R-", vec![CPY_REG_REG_BYTE, direct::BL, postd_al], |r| {
            reg_delta(r, offset::BL, 4);
            reg_delta(r, offset::AL, 3)
        }),
        ("R,R+", vec![CPY_REG_REG_BYTE, direct::BL, posti_al], |r| {
            reg_delta(r, offset::BL, 3);
            reg_delta(r, offset::AL, 4)
        }),
        //first changes, second direct
        ("+R,R", vec![CPY_REG_REG_BYTE, prei_bl, direct::AL], |r| {
            reg_delta(r, offset::BL, 4);
            reg_delta(r, offset::AL, 4)
        }),
        ("-R,R", vec![CPY_REG_REG_BYTE, pred_bl, direct::AL], |r| {
            reg_delta(r, offset::BL, 4);
            reg_delta(r, offset::AL, 4)
        }),
        ("R-,R", vec![CPY_REG_REG_BYTE, postd_bl, direct::AL], |r| {
            reg_delta(r, offset::BL, 3);
            reg_delta(r, offset::AL, 4)
        }),
        ("R+,R", vec![CPY_REG_REG_BYTE, posti_bl, direct::AL], |r| {
            reg_delta(r, offset::BL, 5);
            reg_delta(r, offset::AL, 4)
        }),
        //first changes, second always post dec
        ("+R,R-", vec![CPY_REG_REG_BYTE, prei_bl, postd_al], |r| {
            reg_delta(r, offset::BL, 4);
            reg_delta(r, offset::AL, 3)
        }),
        ("-R,R-", vec![CPY_REG_REG_BYTE, pred_bl, postd_al], |r| {
            reg_delta(r, offset::BL, 3);
            reg_delta(r, offset::AL, 2)
        }),
        ("R-,R-", vec![CPY_REG_REG_BYTE, postd_bl, postd_al], |r| {
            reg_delta(r, offset::BL, 1);
            reg_delta(r, offset::AL, 1)
        }),
        ("R+,R-", vec![CPY_REG_REG_BYTE, posti_bl, postd_al], |r| {
            reg_delta(r, offset::BL, 2);
            reg_delta(r, offset::AL, 0)
        }),
        //first changes, second always post inc
        ("+R,R+", vec![CPY_REG_REG_BYTE, prei_bl, posti_al], |r| {
            reg_delta(r, offset::BL, 0);
            reg_delta(r, offset::AL, 1)
        }),
        ("-R,R+", vec![CPY_REG_REG_BYTE, pred_bl, posti_al], |r| {
            reg_delta(r, offset::BL, 1);
            reg_delta(r, offset::AL, 2)
        }),
        ("R-,R+", vec![CPY_REG_REG_BYTE, postd_bl, posti_al], |r| {
            reg_delta(r, offset::BL, 1);
            reg_delta(r, offset::AL, 3)
        }),
        ("R+,R+", vec![CPY_REG_REG_BYTE, posti_bl, posti_al], |r| {
            reg_delta(r, offset::BL, 4);
            reg_delta(r, offset::AL, 4)
        }),
        //first changes, second always pre dec
        ("+R,-R", vec![CPY_REG_REG_BYTE, prei_bl, pred_al], |r| {
            reg_delta(r, offset::BL, 3);
            reg_delta(r, offset::AL, 3)
        }),
        ("-R,-R", vec![CPY_REG_REG_BYTE, pred_bl, pred_al], |r| {
            reg_delta(r, offset::BL, 2);
            reg_delta(r, offset::AL, 2)
        }),
        ("R-,-R", vec![CPY_REG_REG_BYTE, postd_bl, pred_al], |r| {
            reg_delta(r, offset::BL, 0);
            reg_delta(r, offset::AL, 1)
        }),
        ("R+,-R", vec![CPY_REG_REG_BYTE, posti_bl, pred_al], |r| {
            reg_delta(r, offset::BL, 1);
            reg_delta(r, offset::AL, 0)
        }),
        //first changes, second always post dec
        ("+R,+R", vec![CPY_REG_REG_BYTE, prei_bl, prei_al], |r| {
            reg_delta(r, offset::BL, 1);
            reg_delta(r, offset::AL, 1)
        }),
        ("-R,+R", vec![CPY_REG_REG_BYTE, pred_bl, prei_al], |r| {
            reg_delta(r, offset::BL, 2);
            reg_delta(r, offset::AL, 2)
        }),
        ("R-,+R", vec![CPY_REG_REG_BYTE, postd_bl, prei_al], |r| {
            reg_delta(r, offset::BL, 2);
            reg_delta(r, offset::AL, 3)
        }),
        ("R+,+R", vec![CPY_REG_REG_BYTE, posti_bl, prei_al], |r| {
            reg_delta(r, offset::BL, 5);
            reg_delta(r, offset::AL, 4)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op(&mut vm, i, name, op.0, &op.1, op.2(&mut registers))
    }
}
