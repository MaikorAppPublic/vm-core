use maikor_platform::mem::{address, sizes};
use maikor_vm_core::VM;

#[test]
fn test_swapping_banks() {
    let mut vm = VM::new_test();
    vm.ram_banks = vec![
        [2; sizes::RAM_BANK as usize],
        [3; sizes::RAM_BANK as usize],
        [4; sizes::RAM_BANK as usize],
    ];

    vm.atlas_banks = vec![
        [12; sizes::ATLAS as usize],
        [13; sizes::ATLAS as usize],
        [14; sizes::ATLAS as usize],
        [15; sizes::ATLAS as usize],
        [16; sizes::ATLAS as usize],
    ];

    vm.code_banks = vec![
        [90; sizes::CODE_BANK as usize],
        [91; sizes::CODE_BANK as usize],
    ];

    vm.save_banks = vec![
        [100; sizes::SAVE_BANK as usize],
        [101; sizes::SAVE_BANK as usize],
        [102; sizes::SAVE_BANK as usize],
        [103; sizes::SAVE_BANK as usize],
        [104; sizes::SAVE_BANK as usize],
        [105; sizes::SAVE_BANK as usize],
        [106; sizes::SAVE_BANK as usize],
        [107; sizes::SAVE_BANK as usize],
        [108; sizes::SAVE_BANK as usize],
        [109; sizes::SAVE_BANK as usize],
        [110; sizes::SAVE_BANK as usize],
        [111; sizes::SAVE_BANK as usize],
        [112; sizes::SAVE_BANK as usize],
        [113; sizes::SAVE_BANK as usize],
        [114; sizes::SAVE_BANK as usize],
        [115; sizes::SAVE_BANK as usize],
    ];

    vm.init();

    assert_eq!(vm.memory[address::RAM_BANK_ID as usize], 0);
    assert_eq!(vm.memory[address::CODE_BANK_ID as usize], 0);
    assert_eq!(vm.memory[address::SAVE_BANK_ID as usize], 0);
    assert_eq!(vm.memory[address::ATLAS1_BANK_ID as usize], 0);
    assert_eq!(vm.memory[address::ATLAS2_BANK_ID as usize], 1);
    assert_eq!(vm.memory[address::ATLAS3_BANK_ID as usize], 2);
    assert_eq!(vm.memory[address::ATLAS4_BANK_ID as usize], 3);
    assert_eq!(vm.memory[address::RAM_BANK as usize], 2);
    assert_eq!(vm.memory[address::CODE_BANK as usize], 90);
    assert_eq!(vm.memory[address::SAVE_BANK as usize], 100);
    assert_eq!(vm.memory[address::ATLAS1 as usize], 12);
    assert_eq!(vm.memory[address::ATLAS2 as usize], 13);
    assert_eq!(vm.memory[address::ATLAS3 as usize], 14);
    assert_eq!(vm.memory[address::ATLAS4 as usize], 15);

    vm.debug_set_mem(address::RAM_BANK_ID, 2);
    assert_eq!(vm.memory[address::RAM_BANK_ID as usize], 2);
    assert_eq!(vm.memory[address::RAM_BANK as usize], 4);

    vm.debug_set_mem(address::CODE_BANK_ID, 1);
    assert_eq!(vm.memory[address::CODE_BANK_ID as usize], 1);
    assert_eq!(vm.memory[address::CODE_BANK as usize], 91);

    vm.debug_set_mem(address::SAVE_BANK_ID, 6);
    assert_eq!(vm.memory[address::SAVE_BANK_ID as usize], 6);
    assert_eq!(vm.memory[address::SAVE_BANK as usize], 106);

    vm.debug_set_mem(address::ATLAS1_BANK_ID, 4);
    assert_eq!(vm.memory[address::ATLAS1_BANK_ID as usize], 4);
    assert_eq!(vm.memory[address::ATLAS1 as usize], 16);

    vm.debug_set_mem(address::ATLAS4_BANK_ID, 2);
    assert_eq!(vm.memory[address::ATLAS4_BANK_ID as usize], 2);
    assert_eq!(vm.memory[address::ATLAS4 as usize], 14);

    assert!(!vm.save_dirty_flag[6]);
    vm.debug_set_mem(address::SAVE_BANK, 1);
    assert!(vm.save_dirty_flag[6]);
}
