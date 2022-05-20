extern crate core;

use maikor_platform::mem::{address, sizes};
use maikor_platform::registers::flags::*;
use maikor_platform::registers::id;

mod multiple;
mod single;

mod offset {
    pub const AH: usize = 0;
    pub const AL: usize = 1;
    pub const BH: usize = 2;
    pub const BL: usize = 3;
    pub const CH: usize = 4;
    pub const CL: usize = 5;
    pub const DH: usize = 6;
    pub const DL: usize = 7;
    pub const FLAGS: usize = 8;
}

pub fn flags_to_str(flg: u8) -> String {
    let mut output = String::new();
    let list = [
        (CARRY, "CARRY"),
        (OVERFLOW, "OVERFLOW"),
        (ZERO, "ZERO"),
        (INTERRUPTS, "INTERRUPTS"),
        (LESS_THAN, "LESS_THAN"),
        (GREATER_THAN, "GREATER_THAN"),
        (SIGNED, "SIGNED"),
    ];
    for (id, str) in list {
        if flg & id == id {
            if !output.is_empty() {
                output.push(',');
            }
            output.push_str(str);
        }
    }
    if output.is_empty() {
        String::from("(0) -")
    } else {
        format!("({flg}) {output}")
    }
}

pub fn compare_registers(text: &str, expected_values: &[u8; 9], actual_values: &[u8; 9]) {
    let mut mismatches = String::new();
    for (i, &actual) in actual_values.iter().enumerate() {
        let expected = expected_values[i];
        if expected != actual {
            if i < 8 {
                let name = id::to_name(offset_to_id(i) as u8).unwrap();
                mismatches.push_str(&format!("{} was {} not {}\n", name, actual, expected));
            } else {
                mismatches.push_str(&format!(
                    "FLG: was '{}' not '{}'",
                    flags_to_str(actual),
                    flags_to_str(expected)
                ));
            }
        }
    }
    if !mismatches.is_empty() {
        panic!("Register comparison failed for {}:\n{}", text, mismatches)
    }
}

fn offset_to_id(offset_byte: usize) -> usize {
    match offset_byte {
        offset::AH => id::AH,
        offset::AL => id::AL,
        offset::BH => id::BH,
        offset::BL => id::BL,
        offset::CH => id::CH,
        offset::CL => id::CL,
        offset::DH => id::DH,
        offset::DL => id::DL,
        offset::FLAGS => id::FLAGS,
        _ => panic!("impossible: {offset_byte}"),
    }
}

///Used to compare VM memory to expected results
///Ignores reserved area but include IRQ_RET_ADDR and IRQ_REG_DUMP
pub fn compare_memory(text: &str, expected_mem: &[u8], actual_mem: &[u8]) {
    if expected_mem.len() != actual_mem.len() {
        panic!(
            "Memory comparison failed, different sizes: {} != {}",
            expected_mem.len(),
            actual_mem.len()
        );
    }
    let mut mismatches = String::new();
    for (i, &actual) in actual_mem.iter().enumerate() {
        let addr = i as u16;
        if (address::RESERVED..address::RESERVED + sizes::RESERVED).contains(&addr) {
            continue;
        }
        let expected = expected_mem[i];
        if expected != actual {
            mismatches.push_str(&format!(
                "{:04X} was {:02X} not {:02X} | {} was {} not {}\n",
                i, actual, expected, i, actual, expected
            ));
        }
    }
    if !mismatches.is_empty() {
        panic!("Memory comparison failed for {}:\n{}", text, mismatches)
    }
}
