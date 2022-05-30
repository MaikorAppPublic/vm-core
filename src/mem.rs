pub mod sizes {
    use maikor_platform::mem::sizes;

    pub const MAIN_CODE: usize = sizes::MAIN_CODE as usize;
    pub const CODE_BANK: usize = sizes::CODE_BANK as usize;
    pub const RAM_BANK: usize = sizes::RAM_BANK as usize;
    pub const SOUND: usize = sizes::SOUND as usize;
    pub const WAVE_TABLE: usize = sizes::WAVE_TABLE as usize;
    pub const SAVE_BANK: usize = sizes::SAVE_BANK as usize;
    pub const ATLAS: usize = sizes::ATLAS as usize;
    pub const CONTROLLER_GRAPHICS: usize = sizes::CONTROLLER_GRAPHICS as usize;
    pub const TOTAL: usize = sizes::TOTAL;
}

pub mod address {
    use maikor_platform::mem::address;

    pub const CODE_BANK_1: usize = address::CODE_BANK_1 as usize;
    pub const RAM_BANK_1: usize = address::RAM_BANK_1 as usize;
    pub const CODE_BANK_2: usize = address::CODE_BANK_2 as usize;
    pub const RAM_BANK_2: usize = address::RAM_BANK_2 as usize;
    pub const SOUND: usize = address::SOUND as usize;
    pub const WAVE_TABLE: usize = address::WAVE_TABLE as usize;
    pub const SAVE_BANK_ID: usize = address::SAVE_BANK_ID as usize;
    pub const SAVE_BANK: usize = address::SAVE_BANK as usize;
    pub const ATLAS1: usize = address::ATLAS1 as usize;
    pub const ATLAS2: usize = address::ATLAS2 as usize;
    pub const ATLAS3: usize = address::ATLAS3 as usize;
    pub const ATLAS4: usize = address::ATLAS4 as usize;
    pub const CODE_BANK_1_ID: usize = address::CODE_BANK_1_ID as usize;
    pub const RAM_BANK_1_ID: usize = address::RAM_BANK_1_ID as usize;
    pub const CODE_BANK_2_ID: usize = address::CODE_BANK_2_ID as usize;
    pub const RAM_BANK_2_ID: usize = address::RAM_BANK_2_ID as usize;
    pub const ATLAS1_BANK_ID: usize = address::ATLAS1_BANK_ID as usize;
    pub const ATLAS2_BANK_ID: usize = address::ATLAS2_BANK_ID as usize;
    pub const ATLAS3_BANK_ID: usize = address::ATLAS3_BANK_ID as usize;
    pub const ATLAS4_BANK_ID: usize = address::ATLAS4_BANK_ID as usize;
    pub const CONTROLLER_TYPE: usize = address::CONTROLLER_TYPE as usize;
    pub const CONTROLLER_GRAPHICS: usize = address::CONTROLLER_GRAPHICS as usize;
    pub const IRQ_CONTROL: usize = address::IRQ_CONTROL as usize;
    pub const IRQ_REG_DUMP: usize = address::IRQ_REG_DUMP as usize;
    pub const IRQ_REG_ADDR: usize = address::IRQ_RET_ADDR as usize;
    pub const SP: usize = address::SP as usize;
    pub const FP: usize = address::FP as usize;
    pub const RESERVED: usize = address::RESERVED as usize;
    pub const STACK: usize = address::STACK as usize;
}
