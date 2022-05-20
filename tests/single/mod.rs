use crate::{compare_memory, compare_registers};
use maikor_vm_core::VM;

mod add;
mod inc;
// mod mcpy;
mod jmp;
mod mul;
mod sub;
mod swap;

//execute op on a new vm, and compare registers and memory afterwards
fn test_op(desc: &str, bytes: &[u8], registers: &[(usize, u8)], memory: &[(usize, u8)]) {
    run_test_op(VM::new(), desc, bytes, registers, memory);
}

//execute op on a new vm setup with initial state for registers and memory,
//and compare registers and memory afterwards
fn test_op_init(
    desc: &str,
    bytes: &[u8],
    init_registers: &[(usize, u8)],
    init_memory: &[(usize, u8)],
    registers: &[(usize, u8)],
    memory: &[(usize, u8)],
) {
    let mut vm = VM::new();
    for (addr, value) in init_registers {
        vm.registers[*addr] = *value;
    }
    for (addr, value) in init_memory {
        vm.memory[*addr] = *value;
    }
    run_test_op(vm, desc, bytes, registers, memory);
}

fn run_test_op(
    mut vm: VM,
    desc: &str,
    bytes: &[u8],
    registers: &[(usize, u8)],
    memory: &[(usize, u8)],
) {
    let mut expected_reg = vm.registers;
    let mut expected_mem = vm.memory;
    for (addr, value) in registers {
        expected_reg[*addr] = *value;
    }
    for (addr, value) in memory {
        expected_mem[*addr] = *value;
    }
    vm.execute_op(bytes);
    assert!(!vm.halted, "halted with {:?}", vm.error);
    compare_registers(desc, &expected_reg, &vm.registers);
    compare_memory(desc, &expected_mem, &vm.memory);
}
