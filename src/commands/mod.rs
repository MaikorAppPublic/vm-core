mod add;
mod addc;
mod cmp;
mod copy;
mod div;
mod divs;
mod inc_dec;
mod jump;
mod logic;
mod mem_copy;
mod misc;
mod mul;
mod muls;
mod sub;
mod subc;

use crate::internals::flags::{FirstBitSet, Flags};
use crate::internals::memory_access::MemoryAccess;
use crate::internals::register_access::RegisterAccess;
use crate::register::Register;
use crate::types::Address;
use crate::VM;
use maikor_language::registers::flags;
use std::fmt::Debug;

fn is_overflow_add(dst: bool, src: bool, result: bool) -> bool {
    (dst == src) && (dst != result)
}

fn is_overflow_sub(dst: bool, src: bool, result: bool) -> bool {
    (dst != src) && (dst != result)
}

impl VM {
    pub fn set_reg<T>(&mut self, name: &'static str, dst: &Register, src: T)
    where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: Flags<T>,
    {
        self.write(name, dst, src);
        self.set_flags(src);
    }

    pub fn set_reg_with_reg<T>(&mut self, name: &'static str, dst: Register, src: Register)
    where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let value = self.read(name, &src);
        self.set_reg(name, &dst, value);
        self.process_arg(&src, true);
        self.process_arg(&dst, true);
    }

    pub fn set_reg_with_addr<T>(&mut self, name: &'static str, dst: Register, src: Address)
    where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&dst, false);
        let value = self.read_mem(src);
        self.set_reg(name, &dst, value);
        self.process_arg(&dst, true);
    }

    pub fn set_addr<T>(&mut self, _name: &'static str, dst: Address, src: T)
    where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        self.write_mem(dst, src)
    }

    pub fn set_addr_with_reg<T>(&mut self, name: &'static str, dst: Address, src: Register)
    where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&src, false);
        let value = self.read(name, &src);
        self.write_mem(dst, value);
        self.process_arg(&src, true);
    }

    pub fn set_addr_with_addr<T>(&mut self, _name: &'static str, dst: Address, src: Address)
    where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        self.write_mem(dst, self.read_mem(src));
    }

    pub fn change_reg<T>(
        &mut self,
        name: &'static str,
        dst: &Register,
        src: T,
        method: fn(T, T, bool) -> (T, bool),
    ) where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: Flags<T>,
    {
        let dst_value: T = self.read(name, dst);
        let (result, carried) = method(dst_value, src, self.check_flag(flags::CARRY));
        self.write(name, dst, result);
        let overflowed = is_overflow_add(
            dst_value.is_first_bit_set(),
            src.is_first_bit_set(),
            result.is_first_bit_set(),
        );
        self.set_math_flags(result, carried, overflowed);
    }

    pub fn reduce_reg<T>(
        &mut self,
        name: &'static str,
        dst: &Register,
        src: T,
        method: fn(T, T, bool) -> (T, bool),
    ) where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: Flags<T>,
    {
        let dst_value: T = self.read(name, dst);
        let (result, carried) = method(dst_value, src, self.check_flag(flags::CARRY));
        self.write(name, dst, result);
        let overflowed = is_overflow_sub(
            dst_value.is_first_bit_set(),
            src.is_first_bit_set(),
            result.is_first_bit_set(),
        );
        self.set_math_flags(result, carried, overflowed);
    }

    pub fn reduce_reg_with_reg<T>(
        &mut self,
        name: &'static str,
        dst: Register,
        src: Register,
        method: fn(T, T, bool) -> (T, bool),
    ) where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let value = self.read(name, &src);
        self.change_reg(name, &dst, value, method);
        self.process_arg(&src, true);
        self.process_arg(&dst, true);
    }

    pub fn reduce_reg_with_addr<T>(
        &mut self,
        name: &'static str,
        dst: Register,
        src: Address,
        method: fn(T, T, bool) -> (T, bool),
    ) where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&dst, false);
        let value = self.read_mem(src);
        self.change_reg(name, &dst, value, method);
        self.process_arg(&dst, true);
    }

    pub fn change_reg_with_reg<T>(
        &mut self,
        name: &'static str,
        dst: Register,
        src: Register,
        method: fn(T, T, bool) -> (T, bool),
    ) where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&dst, false);
        self.process_arg(&src, false);
        let value = self.read(name, &src);
        self.change_reg(name, &dst, value, method);
        self.process_arg(&src, true);
        self.process_arg(&dst, true);
    }

    pub fn change_reg_with_addr<T>(
        &mut self,
        name: &'static str,
        dst: Register,
        src: Address,
        method: fn(T, T, bool) -> (T, bool),
    ) where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&dst, false);
        let value = self.read_mem(src);
        self.change_reg(name, &dst, value, method);
        self.process_arg(&dst, true);
    }

    pub fn change_addr<T>(
        &mut self,
        _name: &'static str,
        dst: Address,
        src: T,
        method: fn(T, T, bool) -> (T, bool),
    ) where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: MemoryAccess<T>,
    {
        let dst_value = self.read_mem(dst);
        let (result, _) = method(dst_value, src, self.check_flag(flags::CARRY));
        self.write_mem(dst, result);
    }

    pub fn change_addr_with_reg<T>(
        &mut self,
        name: &'static str,
        dst: Address,
        src: Register,
        method: fn(T, T, bool) -> (T, bool),
    ) where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: RegisterAccess<T>,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        self.process_arg(&src, false);
        let value = self.read(name, &src);
        self.change_addr(name, dst, value, method);
        self.process_arg(&src, false);
    }

    pub fn change_addr_with_addr<T>(
        &mut self,
        name: &'static str,
        dst: Address,
        src: Address,
        method: fn(T, T, bool) -> (T, bool),
    ) where
        T: Copy + Debug + Eq + FirstBitSet,
        VM: MemoryAccess<T>,
        VM: Flags<T>,
    {
        let value = self.read_mem(src);
        self.change_addr(name, dst, value, method);
    }
}
