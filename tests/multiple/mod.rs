use crate::{compare_memory, compare_registers};
use maikor_platform::ops::{CPY_ADDR_REG_WORD, CPY_REG_REG_BYTE, INC_REG_BYTE};
use maikor_platform::registers::flags::*;
use maikor_platform::registers::id;
use maikor_vm_core::VM;

#[test]
fn test_simple() {
    let mut vm = VM::new();
    assert_eq!(vm.pc, 0);
    vm.memory[0] = INC_REG_BYTE;
    vm.memory[1] = id::AL as u8;
    vm.memory[2] = INC_REG_BYTE;
    vm.memory[3] = id::AH as u8;
    vm.memory[4] = CPY_REG_REG_BYTE;
    vm.memory[5] = id::BH as u8;
    vm.memory[6] = id::AH as u8;
    vm.memory[7] = CPY_ADDR_REG_WORD;
    vm.memory[8] = 1;
    vm.memory[9] = 0;
    vm.memory[10] = id::BX as u8;
    let mut mem = vm.memory;
    mem[256] = 1;
    vm.step();
    vm.step();
    vm.step();
    vm.step();
    assert!(!vm.halted);
    assert_eq!(vm.pc, 11);
    compare_registers(
        "multiple::test_simple",
        &[1, 1, 1, 0, 0, 0, 0, 0, ZERO | INTERRUPTS],
        &vm.registers,
    );
    compare_memory("multiple::test_simple", &mem, &vm.memory);
}
