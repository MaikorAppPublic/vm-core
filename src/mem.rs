pub mod sizes {
    use maikor_language::mem::sizes;

    pub const CODE_BANK: usize = sizes::CODE_BANK as usize;
    pub const RAM_BANK: usize = sizes::RAM_BANK as usize;
    pub const CODE: usize = CODE_BANK * 2;
    pub const RAM: usize = RAM_BANK * 2;
    pub const SOUND: usize = sizes::SOUND as usize;
    //Byte 0 is direction, byte 1 is action
    pub const INPUT: usize = sizes::INPUT as usize;
    pub const CODE_BANK_ID: usize = 1;
    pub const RAM_BANK_ID: usize = 1;
    pub const SAVE_BANK_ID: usize = 1;
    pub const SAVE_BANK: usize = sizes::SAVE_BANK as usize;
    pub const SPRITE: usize = sizes::SPRITE as usize;
    //255 sprites, each taking 4 bytes
    //(8b X, 8b Y, 9b tile id, 1b flipV, 1b flipH, 2b palette, 1b src size, 2b order, [1b half alpha, 1b double dst size, 2b rotation])
    pub const SPRITE_TABLE: usize = sizes::SPRITE_TABLE as usize;
    //4 layers, each header is made of 3 bytes (8b X, 8b Y, 1b visible, 7b?)
    pub const LAYERS_HEADER: usize = sizes::LAYERS_HEADER as usize;
    //4 layers, each made of 1320 (44x30) tiles, each made of 2 bytes (9b tile id, 1b flipV, 1b flipH, 2b palette, 1b half alpha, 2b rotation)
    pub const LAYERS_CONTENT: usize = sizes::LAYERS_CONTENT as usize;
    //4 palettes, each made of 15 colors, each color is 3 bytes
    pub const PALETTE: usize = sizes::PALETTE as usize;
    pub const PALETTES_TOTAL: usize = sizes::PALETTES_TOTAL as usize;
    //25x20 (tiles) 200x160 (pixels) 100x160 (bytes) atlas of palette index (two colour IDs per byte)
    pub const ATLAS: usize = sizes::ATLAS as usize;
    pub const ATLAS1_BANK_ID: usize = 1;
    pub const ATLAS2_BANK_ID: usize = 1;
    pub const STACK: usize = sizes::STACK as usize;
    pub const SP: usize = sizes::SP as usize;
    pub const FP: usize = sizes::FP as usize;
    pub const TOTAL: usize = sizes::TOTAL as usize;
}

pub mod address {
    use crate::types::Address;
    use maikor_language::mem::address;

    pub const CODE: Address = Address(address::CODE);
    pub const CODE_BANK: Address = Address(address::CODE_BANK);
    pub const RAM: Address = Address(address::RAM);
    pub const RAM_BANK: Address = Address(address::RAM_BANK);
    pub const INPUT: Address = Address(address::INPUT);
    pub const SOUND: Address = Address(address::SOUND);
    pub const SAVE_BANK_ID: Address = Address(address::SAVE_BANK_ID);
    pub const SAVE_BANK: Address = Address(address::SAVE_BANK);
    pub const ATLAS1: Address = Address(address::ATLAS1);
    pub const ATLAS2: Address = Address(address::ATLAS2);
    pub const PALETTES: Address = Address(address::PALETTES);
    pub const SPRITE_TABLE: Address = Address(address::SPRITE_TABLE);
    pub const LAYER_HEADERS: Address = Address(address::LAYER_HEADERS);
    pub const LAYERS: Address = Address(address::LAYERS);
    pub const CODE_BANK_ID: Address = Address(address::CODE_BANK_ID);
    pub const RAM_BANK_ID: Address = Address(address::RAM_BANK_ID);
    pub const ATLAS1_BANK_ID: Address = Address(address::ATLAS1_BANK_ID);
    pub const ATLAS2_BANK_ID: Address = Address(address::ATLAS2_BANK_ID);
    pub const SP: Address = Address(address::SP);
    pub const FP: Address = Address(address::FP);
    pub const RESERVED: Address = Address(address::RESERVED);
    pub const STACK: Address = Address(address::STACK);
}
