use crate::{address, sizes, VM};
use maikor_platform::mem::address::is_special_memory;

/// Word memory access
impl VM {
    /// Get number in memory at address
    /// Returns the value and cycles used
    pub fn read_word_mem(&self, addr: u16) -> (u16, usize) {
        let mut value = self.memory[addr as usize] as u16;
        value <<= 8;
        value += self.memory[(addr + 1) as usize] as u16;
        (value, 2)
    }

    pub fn write_word_mem(&mut self, addr: u16, value: u16) -> usize {
        let cost1 = self.write_byte_mem(addr, ((value >> 8) & 0xFF) as u8);
        let cost2 = self.write_byte_mem(addr + 1, (value & 0xFF) as u8);
        cost1 + cost2
    }
}

/// Byte memory access
impl VM {
    /// Get number in memory at address
    /// Returns the value and cycles used
    #[inline(always)]
    pub fn read_byte_mem(&self, addr: u16) -> (u8, usize) {
        (self.memory[addr as usize], 1)
    }

    pub fn write_byte_mem(&mut self, addr: u16, value: u8) -> usize {
        let addr = addr as usize;
        self.memory[addr] = value;
        let bank_update_cost = self.write_mem_change_to_bank(addr, value);
        let bank_load_cost = if is_special_memory(addr as u16) {
            self.load_banks(addr, value as usize)
        } else {
            0
        };
        1 + bank_load_cost + bank_update_cost
    }

    #[inline]
    fn load_bank(&mut self, address: usize, size: usize, bank: *const u8) {
        unsafe {
            let dst = self.get_memory_mut(address, size).as_mut_ptr();
            std::ptr::copy_nonoverlapping(bank, dst, size);
        }
    }

    fn load_banks(&mut self, addr: usize, value: usize) -> usize {
        match addr {
            address::CODE_BANK_ID => {
                if value < self.code_banks.len() {
                    self.load_bank(
                        address::CODE_BANK,
                        sizes::CODE_BANK,
                        self.code_banks[value].as_ptr(),
                    );
                    return 20;
                }
            }
            address::RAM_BANK_ID => {
                if value < self.ram_banks.len() {
                    self.load_bank(
                        address::RAM_BANK,
                        sizes::RAM_BANK,
                        self.ram_banks[value].as_ptr(),
                    );
                    return 20;
                }
            }
            address::SAVE_BANK_ID => {
                if value < self.save_banks.len() {
                    self.load_bank(
                        address::SAVE_BANK,
                        sizes::SAVE_BANK,
                        self.save_banks[value].as_ptr(),
                    );
                    return 20;
                }
            }
            address::ATLAS1_BANK_ID => {
                if value < self.atlas_banks.len() {
                    self.load_bank(
                        address::ATLAS1,
                        sizes::ATLAS,
                        self.atlas_banks[value].as_ptr(),
                    );
                    return 20;
                }
            }
            address::ATLAS2_BANK_ID => {
                if value < self.atlas_banks.len() {
                    self.load_bank(
                        address::ATLAS2,
                        sizes::ATLAS,
                        self.atlas_banks[value].as_ptr(),
                    );
                    return 20;
                }
            }
            address::ATLAS3_BANK_ID => {
                if value < self.atlas_banks.len() {
                    self.load_bank(
                        address::ATLAS3,
                        sizes::ATLAS,
                        self.atlas_banks[value].as_ptr(),
                    );
                    return 20;
                }
            }
            address::ATLAS4_BANK_ID => {
                if value < self.atlas_banks.len() {
                    self.load_bank(
                        address::ATLAS4,
                        sizes::ATLAS,
                        self.atlas_banks[value].as_ptr(),
                    );
                    return 20;
                }
            }
            _ => {}
        }

        0
    }

    #[inline(always)]
    pub fn get_memory_mut(&mut self, start: usize, len: usize) -> &mut [u8] {
        &mut self.memory[start..start + len]
    }

    /// If a mem write has changed a value in a loaded bank then the change must also
    /// be written to the bank in the list
    /// This is needed as when swapping banks the current loaded values are overwritten and not
    /// stored back in the list
    /// In real hardware this isn't needed as the banks are mapped rather than copied
    // TODO think of a way to map banks
    // reading seems easy as you can do `if in bank, read from bank` but a byte might cross boundaries
    // this could be UB but I would prefer to handle it properly
    // also it might be slower, currently reading from mem is just `mem[addr]` if banks were not
    // copied across then it would become `if in code_bank, else if in ram_bank, etc, else mem[addr]`
    // need to perf test but `match` might be ok for speed here
    fn write_mem_change_to_bank(&mut self, addr: usize, value: u8) -> usize {
        if is_inside_code_bank(addr) {
            let code_bank = &mut self.code_banks[self.memory[address::CODE_BANK_ID] as usize];
            let code_bank_addr = addr - address::CODE_BANK;
            code_bank[code_bank_addr] = value;
            1
        } else if is_inside_ram_bank(addr) {
            let ram_bank = &mut self.ram_banks[self.memory[address::RAM_BANK_ID] as usize];
            let ram_bank_addr = addr - address::RAM_BANK;
            ram_bank[ram_bank_addr] = value;
            1
        } else if is_inside_save_bank(addr) {
            let bank_id = self.memory[address::SAVE_BANK_ID] as usize;
            let save_bank = &mut self.save_banks[bank_id];
            let save_bank_addr = addr - address::SAVE_BANK;
            save_bank[save_bank_addr] = value;
            self.save_dirty_flag[bank_id] = true;
            1
        } else if is_inside_atlas1_bank(addr) {
            let atlas_bank = &mut self.atlas_banks[self.memory[address::ATLAS1_BANK_ID] as usize];
            let atlas_bank_addr = addr - address::ATLAS1;
            atlas_bank[atlas_bank_addr] = value;
            1
        } else if is_inside_atlas2_bank(addr) {
            let atlas_bank = &mut self.atlas_banks[self.memory[address::ATLAS2_BANK_ID] as usize];
            let atlas_bank_addr = addr - address::ATLAS2;
            atlas_bank[atlas_bank_addr] = value;
            1
        } else if is_inside_atlas3_bank(addr) {
            let atlas_bank = &mut self.atlas_banks[self.memory[address::ATLAS3_BANK_ID] as usize];
            let atlas_bank_addr = addr - address::ATLAS3;
            atlas_bank[atlas_bank_addr] = value;
            1
        } else if is_inside_atlas4_bank(addr) {
            let atlas_bank = &mut self.atlas_banks[self.memory[address::ATLAS4_BANK_ID] as usize];
            let atlas_bank_addr = addr - address::ATLAS4;
            atlas_bank[atlas_bank_addr] = value;
            1
        } else {
            0
        }
    }
}

#[inline(always)]
fn is_inside_code_bank(addr: usize) -> bool {
    (address::CODE_BANK..(address::CODE_BANK + sizes::CODE_BANK)).contains(&addr)
}

#[inline(always)]
fn is_inside_ram_bank(addr: usize) -> bool {
    (address::RAM_BANK..(address::RAM_BANK + sizes::RAM_BANK)).contains(&addr)
}

#[inline(always)]
fn is_inside_save_bank(addr: usize) -> bool {
    (address::SAVE_BANK..(address::SAVE_BANK + sizes::SAVE_BANK)).contains(&addr)
}

#[inline]
fn is_inside_atlas1_bank(addr: usize) -> bool {
    (address::ATLAS1..(address::ATLAS1 + sizes::ATLAS)).contains(&addr)
}

#[inline]
fn is_inside_atlas2_bank(addr: usize) -> bool {
    (address::ATLAS2..(address::ATLAS2 + sizes::ATLAS)).contains(&addr)
}

#[inline]
fn is_inside_atlas3_bank(addr: usize) -> bool {
    (address::ATLAS3..(address::ATLAS3 + sizes::ATLAS)).contains(&addr)
}

#[inline]
fn is_inside_atlas4_bank(addr: usize) -> bool {
    (address::ATLAS4..(address::ATLAS4 + sizes::ATLAS)).contains(&addr)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn byte_mem_write() {
        let mut vm = VM::new_test();

        let cost1 = vm.write_byte_mem(2, 124);
        let cost2 = vm.write_byte_mem(875, 1);

        assert_eq!(vm.memory[2], 124);
        assert_eq!(vm.memory[875], 1);
        assert_eq!(cost1, 1);
        assert_eq!(cost2, 1);
    }

    #[test]
    fn word_mem_write() {
        let mut vm = VM::new_test();

        let cost = vm.write_word_mem(875, 10);

        assert_eq!(vm.memory[875], 0);
        assert_eq!(vm.memory[876], 10);
        assert_eq!(cost, 2);
    }

    #[test]
    fn byte_mem_read() {
        let mut vm = VM::new_test();

        vm.memory[12] = 56;
        let (value, cost) = vm.read_byte_mem(12);

        assert_eq!(value, 56);
        assert_eq!(cost, 1);
    }

    #[test]
    fn word_mem_read() {
        let mut vm = VM::new_test();

        vm.memory[12] = 1;
        vm.memory[13] = 56;
        let (value, cost) = vm.read_word_mem(12);

        assert_eq!(value, 312);
        assert_eq!(cost, 2);
    }
}
