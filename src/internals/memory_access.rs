use crate::types::{Address, Byte, Word};
use crate::{address, mem, VM};

impl VM {
    pub fn get_memory(&self, start: Address, length: usize) -> &[u8] {
        &self.memory[start.0 as usize..start.0 as usize + length]
    }

    pub fn get_memory_mut(&mut self, start: Address, length: usize) -> &mut [u8] {
        &mut self.memory[start.0 as usize..start.0 as usize + length]
    }
}

pub trait MemoryAccess<T> {
    fn read_mem(&self, addr: Address) -> T;
    fn write_mem(&mut self, addr: Address, value: T);
}

impl MemoryAccess<Byte> for VM {
    fn read_mem(&self, addr: Address) -> Byte {
        self.memory[addr].into()
    }

    fn write_mem(&mut self, addr: Address, value: Byte) {
        self.memory[addr] = value.0;
        if is_inside_code_bank(addr) {
            let code_bank = &mut self.code_banks[self.memory[address::CODE_BANK_ID] as usize];
            let code_bank_addr = addr - address::CODE_BANK;
            code_bank[code_bank_addr] = value.0;
        }
        if is_inside_ram_bank(addr) {
            let ram_bank = &mut self.ram_banks[self.memory[address::RAM_BANK_ID] as usize];
            let ram_bank_addr = addr - address::RAM_BANK;
            ram_bank[ram_bank_addr] = value.0;
        }
        if is_inside_save_bank(addr) {
            let bank_id = self.memory[address::SAVE_BANK_ID] as usize;
            let save_bank = &mut self.save_banks[bank_id];
            let save_bank_addr = addr - address::SAVE_BANK;
            save_bank[save_bank_addr] = value.0;
            self.save_dirty_flag[bank_id] = true;
        }
        if addr == address::CODE_BANK_ID {
            if value.as_usize() >= self.code_banks.len() {
                self.fail(format!("Invalid code bank: {value}"));
            } else {
                unsafe {
                    let dst = self
                        .get_memory_mut(address::CODE_BANK, mem::sizes::CODE_BANK)
                        .as_mut_ptr();
                    let src = self.code_banks[value.as_usize()].as_ptr();
                    std::ptr::copy_nonoverlapping(src, dst, mem::sizes::CODE_BANK);
                }
            }
        }
        if addr == address::RAM_BANK_ID {
            if value.as_usize() >= self.ram_banks.len() {
                self.fail(format!("Invalid ram bank: {value}"));
            } else {
                unsafe {
                    let dst = self
                        .get_memory_mut(address::RAM_BANK, mem::sizes::RAM_BANK)
                        .as_mut_ptr();
                    let src = self.ram_banks[value.as_usize()].as_ptr();
                    std::ptr::copy_nonoverlapping(src, dst, mem::sizes::RAM_BANK);
                }
            }
        }
        if addr == address::SAVE_BANK_ID {
            unsafe {
                let dst = self
                    .get_memory_mut(address::SAVE_BANK, mem::sizes::SAVE_BANK)
                    .as_mut_ptr();
                let src = self.save_banks[value.as_usize()].as_ptr();
                std::ptr::copy_nonoverlapping(src, dst, mem::sizes::SAVE_BANK);
            }
        }
        if addr == address::ATLAS1_BANK_ID {
            if value.as_usize() >= self.atlas_banks.len() {
                self.fail(format!("Invalid atlas bank: {value}"));
            } else {
                unsafe {
                    let dst = self
                        .get_memory_mut(address::ATLAS1, mem::sizes::ATLAS)
                        .as_mut_ptr();
                    let src = self.atlas_banks[value.as_usize()].as_ptr();
                    std::ptr::copy_nonoverlapping(src, dst, mem::sizes::ATLAS);
                }
            }
        }
        if addr == address::ATLAS2_BANK_ID {
            if value.as_usize() >= self.atlas_banks.len() {
                self.fail(format!("Invalid atlas bank: {value}"));
            } else {
                unsafe {
                    let dst = self
                        .get_memory_mut(address::ATLAS2, mem::sizes::ATLAS)
                        .as_mut_ptr();
                    let src = self.atlas_banks[value.as_usize()].as_ptr();
                    std::ptr::copy_nonoverlapping(src, dst, mem::sizes::ATLAS);
                }
            }
        }
    }
}

impl MemoryAccess<Word> for VM {
    fn read_mem(&self, addr: Address) -> Word {
        u16::from_be_bytes([self.memory[addr], self.memory[addr + Address::ONE]]).into()
    }

    fn write_mem(&mut self, addr: Address, value: Word) {
        let bytes = value.to_be_bytes();
        self.write_mem(addr, bytes[0]);
        self.write_mem(addr + Address::ONE, bytes[1]);
    }
}

pub fn is_inside_code_bank(addr: Address) -> bool {
    addr >= address::CODE_BANK && addr < (address::CODE_BANK + mem::sizes::CODE_BANK)
}

pub fn is_inside_ram_bank(addr: Address) -> bool {
    addr >= address::RAM_BANK && addr < (address::RAM_BANK + mem::sizes::RAM_BANK)
}

pub fn is_inside_save_bank(addr: Address) -> bool {
    addr >= address::SAVE_BANK && addr < (address::SAVE_BANK + mem::sizes::SAVE_BANK)
}
