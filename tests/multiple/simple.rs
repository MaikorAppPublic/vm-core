use crate::compare_memory;
use maikor_language::op_params::IND_POST_DEC;
use maikor_language::ops::*;
use maikor_language::registers::flags::{INTERRUPTS, ZERO};
use maikor_language::registers::id;
use maikor_vm_core::VM;

#[test]
fn test_simple_program() {
    let mut vm = VM::new();
    let mut mem = vm.memory;
    vm.execute_op(&[INC_REG_BYTE, id::AL as u8]);
    assert_eq!(vm.registers, [0, 1, 0, 0, 0, 0, 0, 0, INTERRUPTS]);
    vm.execute_op(&[INC_REG_BYTE, id::AH as u8]);
    assert_eq!(vm.registers, [1, 1, 0, 0, 0, 0, 0, 0, INTERRUPTS]);
    vm.execute_op(&[ADD_REG_NUM_WORD, id::AX as u8, 0, 16]);
    assert_eq!(vm.registers, [1, 17, 0, 0, 0, 0, 0, 0, INTERRUPTS]);
    vm.execute_op(&[SUB_REG_NUM_WORD, id::AX as u8, 1, 17]);
    assert_eq!(vm.registers, [0, 0, 0, 0, 0, 0, 0, 0, INTERRUPTS | ZERO]);
    vm.execute_op(&[CPY_REG_NUM_BYTE, id::BL as u8, 60]);
    assert_eq!(vm.registers, [0, 0, 0, 60, 0, 0, 0, 0, INTERRUPTS]);
    vm.execute_op(&[INC_REG_BYTE, id::AX as u8 | IND_POST_DEC]);
    assert_eq!(vm.registers, [255, 254, 0, 60, 0, 0, 0, 0, INTERRUPTS]);
    mem[0] = 1;
    compare_memory("INC.B (AX)-", &vm.memory, &mem);
}
