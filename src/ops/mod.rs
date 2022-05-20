use crate::register::Register;
use crate::VM;

mod cmp;
mod cpy;
pub mod execute_command;
mod inc_dec;
mod jump;
mod math;
mod misc;
mod ppid;

// Argument reading
impl VM {
    fn next_byte(&mut self) -> u8 {
        let byte = self.memory[self.arg_ptr as usize];
        self.arg_ptr += 1;
        byte
    }

    #[inline(always)]
    fn byte(&mut self) -> u8 {
        self.next_byte()
    }

    fn word(&mut self) -> u16 {
        let mut value = self.next_byte() as u16;
        value <<= 8;
        value += self.next_byte() as u16;
        value
    }

    #[inline(always)]
    fn register(&mut self) -> Register {
        Register::from(self.next_byte())
    }
}

#[cfg(test)]
mod test {
    use crate::VM;
    use maikor_platform::mem::address::RESERVED;

    pub fn check_cycles(bytes: &[u8], expected_cycles: usize, method: fn(&mut VM) -> usize) {
        let mut vm = VM::new();
        vm.arg_ptr = RESERVED;
        for (i, byte) in bytes.iter().enumerate() {
            vm.memory[RESERVED as usize + i] = *byte;
        }
        assert_eq!(method(&mut vm), expected_cycles)
    }

    pub fn check_jmp_cycles(
        bytes: &[u8],
        expected_cycles: usize,
        method: fn(&mut VM) -> (bool, usize),
    ) {
        let mut vm = VM::new();
        vm.arg_ptr = RESERVED;
        for (i, byte) in bytes.iter().enumerate() {
            vm.memory[RESERVED as usize + i] = *byte;
        }
        assert_eq!(method(&mut vm).1, expected_cycles)
    }
}
