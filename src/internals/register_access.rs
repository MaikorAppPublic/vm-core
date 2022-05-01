use crate::internals::memory_access::MemoryAccess;
use crate::internals::register_access::traits::DirectRegisterAccess;
use crate::internals::WordRegisterErrorReason;
use crate::register::Register;
use crate::types::{Address, AsByte, AsWord, Byte, Word};
use crate::VM;
use std::fmt::Debug;

mod traits {
    use std::fmt::Debug;

    pub trait DirectRegisterAccess<T: Copy + Clone + Debug + PartialEq + Eq> {
        fn read_register(&self, addr: usize) -> T;
        fn write_register(&mut self, addr: usize, value: T);
    }
}

pub trait WrappedRegisterAccess<T: Copy + Clone + Debug + PartialEq + Eq>:
    DirectRegisterAccess<T>
{
    fn write(&mut self, op_name: &'static str, dst: &Register, value: T);
    fn read(&mut self, op_name: &'static str, src: &Register) -> T;
}

impl DirectRegisterAccess<Byte> for VM {
    fn read_register(&self, addr: usize) -> Byte {
        self.registers[addr].as_byte()
    }

    fn write_register(&mut self, addr: usize, value: Byte) {
        self.registers[addr] = value.0;
    }
}
impl DirectRegisterAccess<Word> for VM {
    fn read_register(&self, addr: usize) -> Word {
        u16::from_be_bytes([self.registers[addr], self.registers[addr + 1]]).as_word()
    }

    fn write_register(&mut self, addr: usize, value: Word) {
        let bytes = value.0.to_be_bytes();
        self.registers[addr] = bytes[0];
        self.registers[addr + 1] = bytes[1];
    }
}
impl WrappedRegisterAccess<Byte> for VM {
    fn write(&mut self, op_name: &'static str, dst: &Register, value: Byte) {
        if dst.is_indirect {
            self.required_extended_register(op_name, WordRegisterErrorReason::Indirect, dst);
            self.write_mem(self.read_register_as_addr(dst.addr), value)
        } else {
            self.write_register(dst.addr, value)
        }
    }

    fn read(&mut self, op_name: &'static str, src: &Register) -> Byte {
        if src.is_indirect {
            self.required_extended_register(op_name, WordRegisterErrorReason::Indirect, src);
            self.read_mem(self.read_register_as_addr(src.addr))
        } else {
            self.read_register(src.addr)
        }
    }
}
impl WrappedRegisterAccess<Word> for VM {
    fn write(&mut self, op_name: &'static str, dst: &Register, value: Word) {
        self.required_extended_register(op_name, WordRegisterErrorReason::Required, dst);
        if dst.is_indirect {
            self.write_mem(self.read_register_as_addr(dst.addr), value)
        } else {
            self.write_register(dst.addr, value)
        }
    }

    fn read(&mut self, op_name: &'static str, src: &Register) -> Word {
        self.required_extended_register(op_name, WordRegisterErrorReason::Required, src);
        if src.is_indirect {
            self.read_mem(self.read_register_as_addr(src.addr))
        } else {
            self.read_register(src.addr)
        }
    }
}

impl VM {
    fn read_register_as_addr(&self, offset: usize) -> Address {
        let addr_num: Word = self.read_register(offset);
        addr_num.to_address()
    }
}
