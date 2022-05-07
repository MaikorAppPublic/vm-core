use crate::{
    direct, flags, indirect, mem_change, mem_change_w, reg_change, reg_change_w, setup_vm,
    test_single_op, test_single_op_m, MemoryOp, RegistersOp,
};
use maikor_language::ops::{INC_REG_BYTE, INC_REG_WORD};
use maikor_language::registers::offset;

#[test]
fn test_all_inc() {
    test_inc_b();
    test_inc_w();
    test_inc_b_indirect();
    test_inc_w_indirect();
}

fn test_inc_b() {
    let name = "INC.B";
    let mut vm = setup_vm();
    let mut registers = vm.registers;
    let list: Vec<RegistersOp> = vec![
        ("R", vec![INC_REG_BYTE, direct::AL], |r| {
            reg_change(r, offset::AL, 1);
            reg_change(r, offset::FLAGS, flags::POSITIVE_NUM)
        }),
        ("R", vec![INC_REG_BYTE, direct::AL], |r| {
            reg_change(r, offset::AL, 2);
            reg_change(r, offset::FLAGS, flags::POSITIVE_NUM)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op(&mut vm, i, name, op.0, &op.1, op.2(&mut registers))
    }
}

fn test_inc_w() {
    let name = "INC.W";
    let mut vm = setup_vm();
    let mut registers = vm.registers;
    let list: Vec<RegistersOp> = vec![
        ("R", vec![INC_REG_WORD, direct::BX], |r| {
            reg_change_w(r, offset::BX, 1)
        }),
        ("R", vec![INC_REG_WORD, direct::BX], |r| {
            reg_change_w(r, offset::BX, 2)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op(&mut vm, i, name, op.0, &op.1, op.2(&mut registers))
    }
}

fn test_inc_b_indirect() {
    let name = "INC.B";
    let mut vm = setup_vm();
    vm.registers[offset::AL] = 255;
    let registers = vm.registers;
    let mut memory = vm.memory;
    let list: Vec<MemoryOp> = vec![
        ("Ri", vec![INC_REG_BYTE, indirect::AX], |m| {
            mem_change(m, 255, 1)
        }),
        ("Ri", vec![INC_REG_BYTE, indirect::AX], |m| {
            mem_change(m, 255, 2)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op_m(&mut vm, i, name, op.0, &op.1, registers, op.2(&mut memory))
    }
}

fn test_inc_w_indirect() {
    let name = "INC.W";
    let mut vm = setup_vm();
    vm.registers[offset::BH] = 10;
    vm.registers[offset::BL] = 10;
    let registers = vm.registers;
    let mut memory = vm.memory;
    let list: Vec<MemoryOp> = vec![
        ("Ri", vec![INC_REG_WORD, indirect::BX], |m| {
            mem_change_w(m, 2570, 1)
        }),
        ("Ri", vec![INC_REG_WORD, indirect::BX], |m| {
            mem_change_w(m, 2570, 2)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op_m(&mut vm, i, name, op.0, &op.1, registers, op.2(&mut memory))
    }
}
