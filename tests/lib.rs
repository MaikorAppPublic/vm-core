use maikor_language::registers::flags::*;

mod multiple;
mod single;

pub mod offset {
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
                output.push_str(",");
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

pub fn compare_registers(text: &str, lhs: &[u8; 9], rhs: &[u8; 9]) {
    let mut mismatches = String::new();
    for (i, &actual) in lhs.iter().enumerate() {
        let expected = rhs[i];
        if expected != actual {
            if i < 8 {
                mismatches.push_str(&format!("{}: {} != {}\n", i, expected, actual));
            } else {
                mismatches.push_str(&format!(
                    "FLG: '{}' != '{}'",
                    flags_to_str(expected),
                    flags_to_str(actual)
                ));
            }
        }
    }
    if !mismatches.is_empty() {
        panic!("Register comparison failed for {}:\n{}", text, mismatches)
    }
}

pub fn compare_memory(text: &str, lhs: &[u8], rhs: &[u8]) {
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
