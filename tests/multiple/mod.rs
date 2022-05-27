use crate::{compare_memory, compare_registers};
use maikor_platform::ops::{
    ADD_REG_NUM_BYTE, CPY_ADDR_REG_WORD, CPY_REG_REG_BYTE, HALT, INC_REG_BYTE, JBC_ADDR_NUM,
    JBS_ADDR_NUM, JMP_ADDR, SWAP_REG_REG_BYTE,
};
use maikor_platform::registers::flags::*;
use maikor_platform::registers::id;
use maikor_vm_core::VM;

#[test]
fn test_simple() {
    let mut vm = VM::new_test();
    #[rustfmt::skip]
    vm.debug_set_mem_range(0, &[
        INC_REG_BYTE, id::AL,
        INC_REG_BYTE, id::AH,
        CPY_REG_REG_BYTE, id::BH,id::AH,
        CPY_ADDR_REG_WORD,  1, 0, id::BX,
        ADD_REG_NUM_BYTE, id::CL, 1,
        ADD_REG_NUM_BYTE, id::CL, 1,
        SWAP_REG_REG_BYTE, id::CH, id::CL,
        HALT,
    ]);
    let mut mem = vm.memory;
    mem[256] = 1;
    while !vm.halted {
        vm.step();
    }
    assert!(vm.error.is_none());
    assert_eq!(vm.pc, 20);
    compare_registers(
        "multiple::test_simple",
        &[1, 1, 1, 0, 2, 0, 0, 0, INTERRUPTS],
        &vm.registers,
    );
    compare_memory("multiple::test_simple", &mem, &vm.memory);
}

#[test]
#[rustfmt::skip]
fn test_jumping() {
    let mut vm = VM::new_test();
    vm.debug_set_mem_range(0, &[
       JMP_ADDR, 3, 232
    ]);
    vm.debug_set_mem_range(1000, &[
        JBC_ADDR_NUM, 0, 50, INTERRUPTS,
        JBS_ADDR_NUM, 0, 30, INTERRUPTS,
    ]);
    // while !vm.halted {
    vm.step();
    vm.step();
    vm.step();
    // }
    assert_eq!(vm.pc, 30)
}
