use crate::{
    direct, mem_delta, mem_delta_w, reg_delta, reg_delta_w, setup_vm, test_single_op,
    test_single_op_m, MemoryOp, RegistersOp,
};
use maikor_vm_core::constants::ops::{
    CPY_ADDR_ADDR_BYTE, CPY_ADDR_ADDR_WORD, CPY_ADDR_NUM_BYTE, CPY_ADDR_NUM_WORD,
    CPY_ADDR_REG_BYTE, CPY_ADDR_REG_WORD, CPY_REG_ADDR_BYTE, CPY_REG_ADDR_WORD, CPY_REG_NUM_BYTE,
    CPY_REG_NUM_WORD, CPY_REG_REG_BYTE, CPY_REG_REG_WORD,
};
use maikor_vm_core::constants::registers::offset;

#[test]
fn test_all_cpy() {
    test_cpy_b_to_reg();
    test_cpy_b_to_addr();
    test_cpy_w_to_reg();
    test_cpy_w_to_addr();
}

fn test_cpy_b_to_addr() {
    let name = "CPY.B";
    let mut vm = setup_vm();
    vm.memory[513] = 56;
    vm.registers[offset::CL] = 4;
    let registers = vm.registers;
    let mut mem = vm.memory;
    let list: Vec<MemoryOp> = vec![
        ("A,N", vec![CPY_ADDR_NUM_BYTE, 0, 50, 16], |m| {
            mem_delta(m, 50, 16)
        }),
        ("A,R", vec![CPY_ADDR_REG_BYTE, 255, 2, direct::CL], |m| {
            mem_delta(m, 65282, 4)
        }),
        ("A,A", vec![CPY_ADDR_ADDR_BYTE, 1, 1, 255, 2], |m| {
            mem_delta(m, 257, 4)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op_m(&mut vm, i, name, op.0, &op.1, registers, op.2(&mut mem))
    }
}

fn test_cpy_w_to_addr() {
    let name = "CPY.W";
    let mut vm = setup_vm();
    vm.memory[513] = 56;
    vm.registers[offset::CL] = 4;
    let registers = vm.registers;
    let mut mem = vm.memory;
    let list: Vec<MemoryOp> = vec![
        ("A,N", vec![CPY_ADDR_NUM_WORD, 0, 50, 0, 16], |m| {
            mem_delta_w(m, 50, 16)
        }),
        ("A,N", vec![CPY_ADDR_NUM_WORD, 0, 80, 1, 0], |m| {
            mem_delta_w(m, 80, 256)
        }),
        ("A,R", vec![CPY_ADDR_REG_WORD, 255, 2, direct::CX], |m| {
            mem_delta_w(m, 65282, 4)
        }),
        ("A,A", vec![CPY_ADDR_ADDR_WORD, 1, 1, 255, 2], |m| {
            mem_delta_w(m, 257, 4)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op_m(&mut vm, i, name, op.0, &op.1, registers, op.2(&mut mem))
    }
}

fn test_cpy_b_to_reg() {
    let name = "CPY.B";
    let mut vm = setup_vm();
    vm.memory[512] = 56;
    let mut registers = vm.registers;
    let list: Vec<RegistersOp> = vec![
        ("R,N", vec![CPY_REG_NUM_BYTE, direct::AH, 10], |r| {
            reg_delta(r, offset::AH, 10)
        }),
        ("R,R", vec![CPY_REG_REG_BYTE, direct::AL, direct::AH], |r| {
            reg_delta(r, offset::AL, 10)
        }),
        ("R,R", vec![CPY_REG_REG_BYTE, direct::DL, direct::AH], |r| {
            reg_delta(r, offset::DL, 10)
        }),
        ("R,A", vec![CPY_REG_ADDR_BYTE, direct::CL, 2, 0], |r| {
            reg_delta(r, offset::CL, 56)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op(&mut vm, i, name, op.0, &op.1, op.2(&mut registers))
    }
}

fn test_cpy_w_to_reg() {
    let name = "CPY.W";
    let mut vm = setup_vm();
    vm.memory[200] = 255;
    vm.memory[201] = 1;
    let mut registers = vm.registers;
    let list: Vec<RegistersOp> = vec![
        ("R,N", vec![CPY_REG_NUM_WORD, direct::AX, 0, 90], |r| {
            reg_delta_w(r, offset::AX, 90)
        }),
        ("R,N", vec![CPY_REG_NUM_WORD, direct::AX, 4, 10], |r| {
            reg_delta_w(r, offset::AX, 1034)
        }),
        ("R,R", vec![CPY_REG_REG_WORD, direct::BX, direct::AX], |r| {
            reg_delta_w(r, offset::BX, 1034)
        }),
        ("R,A", vec![CPY_REG_ADDR_WORD, direct::CX, 0, 200], |r| {
            reg_delta_w(r, offset::CX, 65281)
        }),
    ];
    for (i, op) in list.iter().enumerate() {
        test_single_op(&mut vm, i, name, op.0, &op.1, op.2(&mut registers))
    }
}
