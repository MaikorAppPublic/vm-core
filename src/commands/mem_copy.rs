use crate::internals::memory_access::MemoryAccess;
use crate::internals::register_access::RegisterAccess;
use crate::register::Register;
use crate::types::{Address, Byte, Word};
use crate::VM;
use maikor_language::names::full;
use maikor_language::names::full::MEM_CPY_REG_ADDR_REG;
use maikor_language::names::op::{MEM_CPY_REG_REG_BYTE, MEM_CPY_REG_REG_REG};

impl VM {
    fn duplicate_mem(&mut self, dst: Address, src: Address, count: u8) {
        let dst_ptr = dst.0;
        let src_ptr = src.0;
        for i in 0..count as u16 {
            let value: Byte = self.read_mem(Address(src_ptr.wrapping_add(i)));
            self.write_mem(Address(dst_ptr.wrapping_add(i)), value);
        }
    }

    pub fn mem_cpy_reg_reg_byte(&mut self, dst: Register, src: Register, count: Byte) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let dst_addr: Word = self.read(MEM_CPY_REG_REG_BYTE, &dst);
        let src_addr: Word = self.read(MEM_CPY_REG_REG_BYTE, &src);
        self.duplicate_mem(dst_addr.to_address(), src_addr.to_address(), count.0);
        self.process_arg(&src, true);
        self.process_arg(&dst, true);
    }

    pub fn mem_cpy_reg_reg_reg(&mut self, dst: Register, src: Register, count: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        self.process_arg(&count, false);
        let dst_addr: Word = self.read(MEM_CPY_REG_REG_REG, &dst);
        let src_addr: Word = self.read(MEM_CPY_REG_REG_REG, &src);
        let bytes: Byte = self.read(MEM_CPY_REG_REG_REG, &count);
        self.duplicate_mem(dst_addr.to_address(), src_addr.to_address(), bytes.0);
        self.process_arg(&count, true);
        self.process_arg(&src, true);
        self.process_arg(&dst, true);
    }

    pub fn mem_cpy_reg_addr_byte(&mut self, dst: Register, src: Address, count: Byte) {
        self.process_arg(&dst, false);
        let dst_addr: Word = self.read(MEM_CPY_REG_REG_REG, &dst);
        self.duplicate_mem(dst_addr.to_address(), src, count.0);
        self.process_arg(&dst, true);
    }

    pub fn mem_cpy_reg_addr_reg(&mut self, dst: Register, src: Address, count: Register) {
        self.process_arg(&dst, false);
        self.process_arg(&count, false);
        let dst_addr: Word = self.read(MEM_CPY_REG_ADDR_REG, &dst);
        let bytes: Byte = self.read(MEM_CPY_REG_ADDR_REG, &count);
        self.duplicate_mem(dst_addr.to_address(), src, bytes.0);
        self.process_arg(&count, true);
        self.process_arg(&dst, true);
    }

    pub fn mem_cpy_addr_reg_byte(&mut self, dst: Address, src: Register, count: Byte) {
        self.process_arg(&src, false);
        let src_addr: Word = self.read(full::MEM_CPY_ADDR_REG_BYTE, &src);
        self.duplicate_mem(dst, src_addr.to_address(), count.0);
        self.process_arg(&src, true);
    }

    pub fn mem_cpy_addr_reg_reg(&mut self, dst: Address, src: Register, count: Register) {
        self.process_arg(&count, false);
        self.process_arg(&src, false);
        let src_addr: Word = self.read(full::MEM_CPY_ADDR_REG_REG, &src);
        let bytes: Byte = self.read(full::MEM_CPY_ADDR_REG_REG, &count);
        self.duplicate_mem(dst, src_addr.to_address(), bytes.0);
        self.process_arg(&src, true);
        self.process_arg(&count, true);
    }

    pub fn mem_cpy_addr_addr_byte(&mut self, dst: Address, src: Address, count: Byte) {
        self.duplicate_mem(dst, src, count.0);
    }

    pub fn mem_cpy_addr_addr_reg(&mut self, dst: Address, src: Address, count: Register) {
        self.process_arg(&count, false);
        let bytes: Byte = self.read(full::MEM_CPY_ADDR_REG_REG, &count);
        self.duplicate_mem(dst, src, bytes.0);
        self.process_arg(&count, true);
    }
}
