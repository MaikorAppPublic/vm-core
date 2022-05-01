use maikor_vm_core::constants::{mem, registers, SAVE_COUNT};
use maikor_vm_core::VM;

mod simple;

type Registers = [u8; registers::SIZE];
type Memory = [u8; mem::TOTAL];
pub mod direct {
    use crate::make_reg;
    use maikor_vm_core::constants::op_params::values::REGISTER;
    use maikor_vm_core::constants::registers::id;

    pub const AH: u8 = make_reg(id::AH, REGISTER);
    pub const AL: u8 = make_reg(id::AL, REGISTER);
    pub const BH: u8 = make_reg(id::BH, REGISTER);
    pub const BL: u8 = make_reg(id::BL, REGISTER);
    pub const DL: u8 = make_reg(id::DL, REGISTER);
    pub const DH: u8 = make_reg(id::DH, REGISTER);
    pub const CL: u8 = make_reg(id::CL, REGISTER);
    pub const CH: u8 = make_reg(id::CH, REGISTER);
    pub const AX: u8 = make_reg(id::AX, REGISTER);
    pub const BX: u8 = make_reg(id::BX, REGISTER);
    pub const CX: u8 = make_reg(id::CX, REGISTER);
    pub const DX: u8 = make_reg(id::DX, REGISTER);
}

pub mod indirect {
    use crate::make_reg;
    use maikor_vm_core::constants::op_params::values::INDIRECT;
    use maikor_vm_core::constants::registers::id;

    pub const AH: u8 = make_reg(id::AH, INDIRECT);
    pub const AL: u8 = make_reg(id::AL, INDIRECT);
    pub const DH: u8 = make_reg(id::DH, INDIRECT);
    pub const DL: u8 = make_reg(id::DL, INDIRECT);
    pub const CH: u8 = make_reg(id::CH, INDIRECT);
    pub const CL: u8 = make_reg(id::CL, INDIRECT);
    pub const AX: u8 = make_reg(id::AX, INDIRECT);
    pub const BX: u8 = make_reg(id::BX, INDIRECT);
    pub const CX: u8 = make_reg(id::CX, INDIRECT);
    pub const DX: u8 = make_reg(id::DX, INDIRECT);
}

pub fn setup_vm() -> VM {
    let vm = VM::new();
    assert_eq!(vm.pc, 0);
    assert_eq!(vm.save_dirty_flag, [false; SAVE_COUNT]);
    assert!(vm.code_banks.is_empty());
    assert!(vm.ram_banks.is_empty());
    assert!(vm.atlas_banks.is_empty());

    vm
}

#[derive(Debug, Default, Clone)]
pub struct VMDump {
    pub registers: [u8; registers::SIZE],
    pub pc: Option<u16>,
    pub sp: u16,
    pub fp: u16,
}

pub const fn make_reg(id: usize, param: u8) -> u8 {
    (id as u8) + param
}

pub fn test_op(vm: &mut VM, text: &str, bytes: &[u8], expected: VMDump, memory: Option<&Memory>) {
    vm.execute_op(bytes);
    assert_eq!(vm.registers, expected.registers, "{} registers", text);
    assert_eq!(vm.get_sp(), expected.sp, "{} sp", text);
    assert_eq!(vm.get_fp(), expected.fp, "{} fp", text);
    if let Some(pc) = expected.pc {
        assert_eq!(vm.pc, pc, "{} pc", text);
    }
    if let Some(bytes) = memory {
        compare_memory(text, &vm.memory, bytes);
    }
}

fn compare_memory(text: &str, lhs: &[u8], rhs: &[u8]) {
    if lhs.len() != rhs.len() {
        panic!(
            "Memory comparison failed, different sizes: {} != {}",
            lhs.len(),
            rhs.len()
        );
    }
    let mut mismatches = String::new();
    for (i, &actual) in lhs.iter().enumerate() {
        let expected = rhs[i];
        if expected != actual {
            mismatches.push_str(&format!(
                "{:04X}: {:02X} != {:02X} | {}: {} != {}\n",
                i, expected, actual, i, expected, actual
            ));
        }
    }
    if !mismatches.is_empty() {
        panic!("Memory comparison failed for {}:\n{}", text, mismatches)
    }
}

pub fn mem_delta(memory: &mut Memory, offset: usize, new_value: u8) -> Memory {
    memory[offset] = new_value;
    memory.clone()
}

pub fn mem_delta_w(memory: &mut Memory, offset: usize, new_value: u16) -> Memory {
    let bytes = new_value.to_be_bytes();
    memory[offset] = bytes[0];
    memory[offset + 1] = bytes[1];
    memory.clone()
}

pub fn reg_delta(registers: &mut Registers, offset: usize, new_value: u8) -> Registers {
    registers[offset] = new_value;
    registers.clone()
}

pub fn reg_delta_w(registers: &mut Registers, offset: usize, new_value: u16) -> Registers {
    let bytes = new_value.to_be_bytes();
    registers[offset] = bytes[0];
    registers[offset + 1] = bytes[1];
    registers.clone()
}

fn test_single_op(
    vm: &mut VM,
    i: usize,
    op: &str,
    params: &str,
    bytes: &[u8],
    registers: Registers,
) {
    let dump = VMDump {
        registers,
        ..VMDump::default()
    };
    test_op(
        vm,
        &format!("{}) {} ({}): {:?}", i, op, params, bytes),
        bytes,
        dump,
        None,
    );
}

fn test_single_op_m(
    vm: &mut VM,
    i: usize,
    op: &str,
    params: &str,
    bytes: &[u8],
    registers: Registers,
    mem: Memory,
) {
    let dump = VMDump {
        registers,
        ..VMDump::default()
    };
    test_op(
        vm,
        &format!("{}) {} ({}): {:?}", i, op, params, bytes),
        bytes,
        dump,
        Some(&mem),
    );
}
