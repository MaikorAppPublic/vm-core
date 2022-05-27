use crate::register::Register;
use crate::VM;

/// Word register access
impl VM {
    /// Get number in word reg, ignoring any addressing
    /// Returns the value and cycles used
    #[must_use]
    pub fn read_word_reg_value(&mut self, reg: &Register) -> (u16, usize) {
        let mut value = self.registers[reg.addr] as u16;
        value <<= 8;
        value += self.registers[reg.addr + 1] as u16;
        (value, 2)
    }

    /// Write number in word reg, ignoring any addressing
    /// Returns the cycles used
    #[must_use]
    pub fn write_word_reg_value(&mut self, reg: &Register, value: u16) -> usize {
        self.registers[reg.addr] = ((value >> 8) & 0xFF) as u8;
        self.registers[reg.addr + 1] = (value & 0xFF) as u8;
        2
    }

    /// Get number from word reg, either
    ///   if direct, the value in the reg
    ///   if indirect, the value in memory at the address in the reg
    /// Returns the value and cycles used
    #[must_use]
    pub fn read_word_reg(&mut self, reg: &Register, offset: i16) -> (u16, usize) {
        let value = self.read_word_reg_value(reg);
        if reg.is_indirect {
            let result = self.read_word_mem(index(value.0, offset));
            (result.0, value.1 + result.1)
        } else {
            value
        }
    }

    /// Write to register, either
    ///   if direct, the value in the reg
    ///   if indirect, the value in memory at the address in the reg
    /// Returns the cycles used
    #[must_use]
    pub fn write_word_reg(&mut self, reg: &Register, offset: i16, value: u16) -> usize {
        if reg.is_indirect {
            let (addr, cost) = self.read_word_reg_value(reg);
            let mem_cost = self.write_word_mem(index(addr, offset), value);
            cost + mem_cost
        } else {
            self.write_word_reg_value(reg, value)
        }
    }
}

/// Byte register access
impl VM {
    /// Get number in word reg, ignoring any addressing
    /// Returns the value and cycles used
    #[must_use]
    pub fn read_byte_reg_value(&mut self, reg: &Register) -> (u8, usize) {
        (self.registers[reg.addr], 1)
    }

    /// Write number in word reg, ignoring any addressing
    /// Returns the cycles used
    #[must_use]
    pub fn write_byte_reg_value(&mut self, reg: &Register, value: u8) -> usize {
        self.registers[reg.addr] = value;
        1
    }

    /// Get number from word reg, either
    ///   if direct, the value in the reg
    ///   if indirect, the value in memory at the address in the reg
    /// Returns the value and cycles used
    #[must_use]
    pub fn read_byte_reg(&mut self, reg: &Register, offset: i16) -> (u8, usize) {
        if reg.is_indirect {
            let addr = self.read_word_reg_value(reg);
            let result = self.read_byte_mem(index(addr.0, offset));
            (result.0, addr.1 + result.1)
        } else {
            self.read_byte_reg_value(reg)
        }
    }

    /// Write to register, either
    ///   if direct, the value in the reg
    ///   if indirect, the value in memory at the address in the reg
    /// Returns the cycles used
    #[must_use]
    pub fn write_byte_reg(&mut self, reg: &Register, offset: i16, value: u8) -> usize {
        if reg.is_indirect {
            let (addr, cost) = self.read_word_reg_value(reg);
            let mem_cost = self.write_byte_mem(index(addr, offset), value);
            cost + mem_cost
        } else {
            self.registers[reg.addr] = value;
            1
        }
    }
}

fn index(addr: u16, offset: i16) -> u16 {
    (addr as i16 + offset) as u16
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::offset;
    use maikor_platform::op_params::INDIRECT;
    use maikor_platform::registers::{id, FLG_DEFAULT};

    #[test]
    fn byte_reg_write() {
        let mut vm = VM::new_test();

        let cost1 = vm.write_byte_reg(&Register::from(id::AL as u8), 0, 10);
        let cost2 = vm.write_byte_reg(&Register::from(id::DH as u8), 0, 19);

        assert_eq!(vm.registers, [0, 10, 0, 0, 0, 0, 19, 0, FLG_DEFAULT]);
        assert_eq!(cost1, 1);
        assert_eq!(cost2, 1);
    }

    #[test]
    fn byte_ind_reg_write() {
        let mut vm = VM::new_test();

        vm.registers[offset::AH] = 1;
        vm.registers[offset::DL] = 50;
        let cost1 = vm.write_byte_reg(&Register::from(id::AX as u8 | INDIRECT), 0, 10);
        let cost2 = vm.write_byte_reg(&Register::from(id::DX as u8 | INDIRECT), 0, 19);

        assert_eq!(vm.memory[256], 10);
        assert_eq!(vm.memory[50], 19);
        assert_eq!(cost1, 3);
        assert_eq!(cost2, 3);
    }

    #[test]
    fn word_reg_write() {
        let mut vm = VM::new_test();

        let cost1 = vm.write_word_reg(&Register::from(id::BX as u8), 0, 256);
        let cost2 = vm.write_word_reg(&Register::from(id::DX as u8), 0, 12563);

        assert_eq!(vm.registers, [0, 0, 1, 0, 0, 0, 49, 19, FLG_DEFAULT]);
        assert_eq!(cost1, 2);
        assert_eq!(cost2, 2);
    }

    #[test]
    fn word_ind_reg_write() {
        let mut vm = VM::new_test();

        vm.registers[offset::AH] = 1;
        vm.registers[offset::DL] = 50;
        let cost1 = vm.write_word_reg(&Register::from(id::AX as u8 | INDIRECT), 0, 10);
        let cost2 = vm.write_word_reg(&Register::from(id::DX as u8 | INDIRECT), 0, 19);

        assert_eq!(vm.memory[257], 10);
        assert_eq!(vm.memory[51], 19);
        assert_eq!(cost1, 4);
        assert_eq!(cost2, 4);
    }

    #[test]
    fn byte_reg_read() {
        let mut vm = VM::new_test();

        vm.registers[offset::AH] = 4;
        let (value, cost) = vm.read_byte_reg(&Register::from(id::AH as u8), 0);

        assert_eq!(value, 4);
        assert_eq!(cost, 1);
    }

    #[test]
    fn word_reg_read() {
        let mut vm = VM::new_test();

        vm.registers[offset::AH] = 2;
        vm.registers[offset::AL] = 2;
        let (value, cost) = vm.read_word_reg(&Register::from(id::AX as u8), 0);

        assert_eq!(value, 514);
        assert_eq!(cost, 2);
    }

    #[test]
    fn byte_ind_reg_read() {
        let mut vm = VM::new_test();

        vm.registers[offset::AL] = 4;
        vm.memory[4] = 15;
        let (value, cost) = vm.read_byte_reg(&Register::from(id::AX as u8 | INDIRECT), 0);

        assert_eq!(value, 15);
        assert_eq!(cost, 3);
    }

    #[test]
    fn word_ind_reg_read() {
        let mut vm = VM::new_test();

        vm.registers[offset::AH] = 2;
        vm.registers[offset::AL] = 2;
        vm.memory[514] = 1;
        vm.memory[515] = 2;
        let (value, cost) = vm.read_word_reg(&Register::from(id::AX as u8 | INDIRECT), 0);

        assert_eq!(value, 258);
        assert_eq!(cost, 4);
    }
}
