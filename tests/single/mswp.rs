mod positive {
    use crate::compare_memory;
    use maikor_platform::ops::MSWP_ADDR_ADDR_BYTE;
    use maikor_vm_core::VM;

    #[test]
    fn test_simple_swap() {
        let mut vm = VM::new_test();
        for i in 0..=255 {
            vm.memory[i] = i as u8;
        }
        let mut mem = vm.memory;
        vm.execute_op(&[MSWP_ADDR_ADDR_BYTE, 0, 10, 0, 40, 4]);
        mem[10] = 40;
        mem[11] = 41;
        mem[12] = 42;
        mem[13] = 43;
        mem[40] = 10;
        mem[41] = 11;
        mem[42] = 12;
        mem[43] = 13;
        compare_memory("test_simple_swap", &mem, &vm.memory);
    }

    #[test]
    fn test_overlapping_swap() {
        let mut vm = VM::new_test();
        for i in 0..=255 {
            vm.memory[i] = i as u8;
        }
        let mut mem = vm.memory;
        vm.execute_op(&[MSWP_ADDR_ADDR_BYTE, 0, 10, 0, 14, 6]);
        mem[10] = 14;
        mem[11] = 15;
        mem[12] = 16;
        mem[13] = 17;
        mem[14] = 18;
        mem[15] = 19;
        mem[16] = 12;
        mem[17] = 13;
        mem[18] = 10;
        mem[19] = 11;
        compare_memory("test_simple_swap", &mem, &vm.memory);
    }
}
