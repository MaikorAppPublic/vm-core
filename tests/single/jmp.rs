use crate::{compare_memory, compare_registers};
use maikor_platform::ops::*;
use maikor_platform::registers::flags::{GREATER_THAN, LESS_THAN};
use maikor_vm_core::VM;

#[test]
pub fn test_relative_jumps() {
    let mut vm = VM::new_test();
    assert_eq!(vm.pc, 0);
    vm.execute_op(&[JRF_BYTE, 15]);
    assert_eq!(vm.pc, 15);
    vm.execute_op(&[JRB_BYTE, 6]);
    assert_eq!(vm.pc, 9);
}

#[test]
pub fn test_jmp_addr() {
    test_jump_addr("jmp", JMP_ADDR, false, false, 256);
    test_jump_addr("jmp", JMP_ADDR, true, false, 256);
    test_jump_addr("jmp", JMP_ADDR, false, true, 256);
    test_jump_addr("jmp", JMP_ADDR, true, true, 256);
    test_jump_addr("je", JE_ADDR, false, false, 256);
    test_jump_addr("jne", JNE_ADDR, true, false, 256);
    test_jump_addr("jne", JNE_ADDR, false, true, 256);
    test_jump_addr("jne", JNE_ADDR, true, true, 256);
    test_jump_addr("jg", JG_ADDR, false, true, 256);
    test_jump_addr("jg", JG_ADDR, true, true, 256);
    test_jump_addr("jge", JGE_ADDR, false, true, 256);
    test_jump_addr("jge", JGE_ADDR, false, false, 256);
    test_jump_addr("jl", JL_ADDR, true, false, 256);
    test_jump_addr("jl", JL_ADDR, true, true, 256);
    test_jump_addr("jle", JLE_ADDR, true, false, 256);
    test_jump_addr("jle", JLE_ADDR, false, false, 256);

    test_jump_addr("je", JE_ADDR, true, false, 3);
    test_jump_addr("je", JE_ADDR, false, true, 3);
    test_jump_addr("je", JE_ADDR, true, true, 3);
    test_jump_addr("jne", JNE_ADDR, false, false, 3);
    test_jump_addr("jg", JG_ADDR, false, false, 3);
    test_jump_addr("jg", JG_ADDR, true, false, 3);
    test_jump_addr("jge", JGE_ADDR, true, false, 3);
    test_jump_addr("jl", JL_ADDR, false, false, 3);
    test_jump_addr("jl", JL_ADDR, false, true, 3);
    test_jump_addr("jle", JLE_ADDR, false, true, 3);
}

fn test_jump_addr(name: &str, op: u8, less: bool, greater: bool, pc: u16) {
    let desc = format!("addr {} {} {} {}", name, less, greater, pc);
    let mut vm = VM::new_test();
    if less {
        vm.set_flag(LESS_THAN);
    }
    if greater {
        vm.set_flag(GREATER_THAN);
    }
    let registers = vm.registers;
    vm.memory[0] = op;
    vm.memory[1] = 1;
    let mem = vm.memory;
    vm.step();
    assert!(!vm.halted, "{}", desc);
    assert_eq!(vm.pc, pc, "{}", desc);
    compare_registers(&desc, &registers, &vm.registers);
    compare_memory(&desc, &mem, &vm.memory)
}

#[test]
pub fn test_jmp_reg() {
    test_jump_reg("jmp", JMP_REG, false, false, 256);
    test_jump_reg("jmp", JMP_REG, true, false, 256);
    test_jump_reg("jmp", JMP_REG, false, true, 256);
    test_jump_reg("jmp", JMP_REG, true, true, 256);
    test_jump_reg("je", JE_REG, false, false, 256);
    test_jump_reg("jne", JNE_REG, true, false, 256);
    test_jump_reg("jne", JNE_REG, false, true, 256);
    test_jump_reg("jne", JNE_REG, true, true, 256);
    test_jump_reg("jg", JG_REG, false, true, 256);
    test_jump_reg("jg", JG_REG, true, true, 256);
    test_jump_reg("jge", JGE_REG, false, true, 256);
    test_jump_reg("jge", JGE_REG, false, false, 256);
    test_jump_reg("jl", JL_REG, true, false, 256);
    test_jump_reg("jl", JL_REG, true, true, 256);
    test_jump_reg("jle", JLE_REG, true, false, 256);
    test_jump_reg("jle", JLE_REG, false, false, 256);

    test_jump_reg("je", JE_REG, true, false, 2);
    test_jump_reg("je", JE_REG, false, true, 2);
    test_jump_reg("je", JE_REG, true, true, 2);
    test_jump_reg("jne", JNE_REG, false, false, 2);
    test_jump_reg("jg", JG_REG, false, false, 2);
    test_jump_reg("jg", JG_REG, true, false, 2);
    test_jump_reg("jge", JGE_REG, true, false, 2);
    test_jump_reg("jl", JL_REG, false, false, 2);
    test_jump_reg("jl", JL_REG, false, true, 2);
    test_jump_reg("jle", JLE_REG, false, true, 2);
}

fn test_jump_reg(name: &str, op: u8, less: bool, greater: bool, pc: u16) {
    let desc = format!("reg {} {} {} {}", name, less, greater, pc);
    let mut vm = VM::new_test();
    if less {
        vm.set_flag(LESS_THAN);
    }
    if greater {
        vm.set_flag(GREATER_THAN);
    }
    vm.registers[0] = 1;
    let registers = vm.registers;
    vm.memory[0] = op;
    let mem = vm.memory;
    vm.step();
    assert!(!vm.halted, "{}", desc);
    assert_eq!(vm.pc, pc, "{}", desc);
    compare_registers(&desc, &registers, &vm.registers);
    compare_memory(&desc, &mem, &vm.memory)
}
