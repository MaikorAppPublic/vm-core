pub mod graphics {
    pub const SCREEN_WIDTH: usize = 240;
    pub const SCREEN_HEIGHT: usize = 160;
    pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
    pub const SPRITE_COUNT: usize = 255;
    pub const LAYER_COUNT: usize = 4;
    pub const PALETTE_COUNT: usize = 4;
    pub const TILES_PER_ROW: usize = 30;
    pub const TILES_PER_COLUMN: usize = 20;
    pub const TILES_PER_LAYER_ROW: usize = 44;
    pub const TILES_PER_LAYER_COLUMN: usize = 30;
    pub const TILES_PER_ATLAS_ROW: usize = 25;
    pub const TILES_PER_ATLAS_COLUMN: usize = 20;
    pub const TILE_WIDTH: usize = 8;
    pub const TILE_HEIGHT: usize = 8;
    pub const ATLAS_TILE_WIDTH: usize = 4;
    pub const ATLAS_TILE_HEIGHT: usize = 8;
    pub const SPRITE_DISABLED_ID: u8 = 0xFF;
}

pub mod mem {
    pub mod sizes {
        pub const CODE_BANK: usize = 8192;
        pub const RAM_BANK: usize = 8192;
        pub const CODE: usize = CODE_BANK * 2;
        pub const RAM: usize = RAM_BANK * 2;
        pub const SOUND: usize = 25;
        //Byte 0 is direction, byte 1 is action
        pub const INPUT: usize = 2;
        pub const CODE_BANK_ID: usize = 1;
        pub const RAM_BANK_ID: usize = 1;
        pub const SAVE_BANK_ID: usize = 1;
        pub const SAVE_BANK: usize = 4096;
        pub const SPRITE: usize = 4;
        //255 sprites, each taking 4 bytes
        //(8b X, 8b Y, 9b tile id, 1b flipV, 1b flipH, 2b palette, 1b size, 2b order)
        pub const SPRITE_TABLE: usize = 255 * 4;
        //4 layers, each header is made of 3 bytes (8b X, 8b Y, 1b visible)
        pub const LAYERS_HEADER: usize = 3 * 4;
        //4 layers, each made of 1320 (44x30) tiles, each made of 2 bytes (9b tile id, 1b flipV, 1b flipH, 2b palette, 3b ?)
        pub const LAYERS_CONTENT: usize = 1320 * 2 * 4;
        //4 palettes, each made of 15 colors, each color is 3 bytes
        pub const PALETTE: usize = 15 * 3;
        pub const PALETTES_TOTAL: usize = 15 * 3 * 4;
        //25x20 (tiles) 200x160 (pixels) 100x160 (bytes) atlas of palette index (two colour IDs per byte)
        pub const ATLAS: usize = 100 * 80;
        pub const ATLAS1_BANK_ID: usize = 1;
        pub const ATLAS2_BANK_ID: usize = 1;
        pub const STACK: usize = 863;
        pub const SP: usize = 2;
        pub const FP: usize = 2;
        pub const ATLAS_TOTAL: usize = ATLAS + ATLAS + ATLAS1_BANK_ID + ATLAS2_BANK_ID;
        pub const LAYER_TOTAL: usize = LAYERS_CONTENT + LAYERS_HEADER;
        pub const GRAPHICS_TOTAL: usize = LAYER_TOTAL + SPRITE_TABLE + PALETTES_TOTAL + ATLAS_TOTAL;
        pub const SYSTEM_TOTAL: usize = CODE + RAM + CODE_BANK_ID + RAM_BANK_ID + STACK + SP + FP;
        pub const HARDWARE_TOTAL: usize = SOUND + INPUT + SAVE_BANK_ID + SAVE_BANK;
        pub const TOTAL: usize = GRAPHICS_TOTAL + SYSTEM_TOTAL + HARDWARE_TOTAL;
    }

    pub mod address {
        use crate::types::Address;

        pub const CODE: Address = Address(0x0);
        pub const CODE_BANK: Address = Address(0x2000);
        pub const RAM: Address = Address(0x4000);
        pub const RAM_BANK: Address = Address(0x6000);
        pub const INPUT: Address = Address(0x8000);
        pub const SOUND: Address = Address(0x8002);
        pub const SAVE_BANK_ID: Address = Address(0x801B);
        pub const SAVE_BANK: Address = Address(0x801C);
        pub const ATLAS1: Address = Address(0x901C);
        pub const ATLAS2: Address = Address(0xAF5C);
        pub const PALETTES: Address = Address(0xCE9C);
        pub const SPRITE_TABLE: Address = Address(0xCF50);
        pub const LAYER_HEADERS: Address = Address(0xD34C);
        pub const LAYERS: Address = Address(0xD358);
        pub const CODE_BANK_ID: Address = Address(0xFC98);
        pub const RAM_BANK_ID: Address = Address(0xFC99);
        pub const ATLAS1_BANK_ID: Address = Address(0xFC9A);
        pub const ATLAS2_BANK_ID: Address = Address(0xFC9B);
        pub const SP: Address = Address(0xFC9C);
        pub const FP: Address = Address(0xFC9E);
        pub const STACK: Address = Address(0xFCA0);

        pub const MAX: u16 = 0xFFFF;
    }

    pub const TOTAL: usize = 0xFFFF;
}

pub const SAVE_COUNT: usize = 16;
pub const MAIKOR_VER: [u8; 2] = [0, 1];

#[rustfmt::skip]
pub mod input {
    pub const UP: usize =    0b00000001; 
    pub const DOWN: usize =  0b00000010; 
    pub const LEFT: usize =  0b00000100; 
    pub const RIGHT: usize = 0b00001000; 
    
    pub const A: usize =     0b00000001; 
    pub const B: usize =     0b00000010; 
    pub const START: usize = 0b00000100; 
    pub const L: usize =     0b00001000; 
    pub const R: usize =     0b00010000; 
}

#[rustfmt::skip]
pub mod registers {
    pub mod offset {
        pub const AH: usize = 0;
        pub const AL: usize = 1;
        pub const BH: usize = 2;
        pub const BL: usize = 3;
        pub const CH: usize = 4;
        pub const CL: usize = 5;
        pub const DH: usize = 6;
        pub const DL: usize = 7;
        pub const FLAGS: usize = 8;
        pub const AX: usize = AH;
        pub const BX: usize = BH;
        pub const CX: usize = CH;
        pub const DX: usize = DH;
        
        // pub const fn is_word_register(reg: usize) -> bool {
        //     matches!(reg, AX | BX | CX | DX)
        // }
    }
    
    pub mod id {
        pub const AH: usize = 0;
        pub const AL: usize = 1;
        pub const BH: usize = 2;
        pub const BL: usize = 3;
        pub const CH: usize = 4;
        pub const CL: usize = 5;
        pub const DH: usize = 6;
        pub const DL: usize = 7;
        pub const FLAGS: usize = 8;
        pub const AX: usize = 9;
        pub const BX: usize = 10;
        pub const CX: usize = 11;
        pub const DX: usize = 12;

        pub const fn name(value: u8) -> &'static str {
            match value as usize {
                AH => "AH",
                AL => "AL",
                BH => "BH",
                BL => "BL",
                CH => "CH",
                CL => "CL",
                DH => "DH",
                DL => "DL",
                AX => "AX",
                BX => "BX",
                CX => "CX",
                DX => "DX",
                FLAGS => "FLG",
                _ => "?"
            }
        }
    }
    
    pub const SIZE: usize = 9;
    
    pub mod flags {
        pub const CARRY: u8 =      0b10000000;
        pub const ZERO: u8 =       0b01000000;
        pub const SIGNED: u8 =     0b00100000;
        pub const OVERFLOW: u8 =   0b00010000;
        pub const HALF_CARRY: u8 = 0b00001000;
        pub const INTERRUPTS: u8 = 0b00000100;
      //pub const RESERVED: u8 =   0b00000010;
      //pub const RESERVED: u8 =   0b00000001;
        
        pub const DEFAULT: u8 = ZERO + INTERRUPTS;
    }
}

pub mod ops {
    pub fn get_byte_count(op: u8) -> usize {
        //rule is generally
        //each reg +1
        //each addr +2
        //each num +1 if byte, +2 if word
        match op {
            NOP | HALT | RETURN | RETURN_FROM_INTERRUPT => 0,
            INC_REG_BYTE | INC_REG_WORD | DEC_REG_BYTE | DEC_REG_WORD | CALL_REG | NOT_REG_BYTE
            | NOT_REG_WORD => 1,
            CPY_REG_NUM_BYTE | CPY_REG_REG_WORD | CPY_REG_REG_BYTE | INC_ADDR_BYTE
            | INC_ADDR_WORD | DEC_ADDR_BYTE | DEC_ADDR_WORD | ADD_REG_REG_BYTE
            | AND_REG_REG_BYTE | AND_REG_REG_WORD | XOR_REG_REG_BYTE | XOR_REG_REG_WORD
            | SUB_REG_REG_BYTE | SUB_REG_REG_WORD | SUB_REG_NUM_BYTE | ADD_REG_REG_WORD
            | ADD_REG_NUM_BYTE | CALL_ADDR | SWAP_REG_REG_BYTE | SWAP_REG_REG_WORD
            | OR_REG_REG_BYTE | OR_REG_REG_WORD | OR_REG_NUM_BYTE | XOR_REG_NUM_BYTE
            | AND_REG_NUM_BYTE => 2,
            CPY_REG_NUM_WORD | CPY_ADDR_NUM_BYTE | CPY_REG_ADDR_WORD | CPY_REG_ADDR_BYTE
            | CPY_ADDR_REG_WORD | CPY_ADDR_REG_BYTE | ADD_REG_NUM_WORD | ADD_ADDR_REG_BYTE
            | ADD_ADDR_REG_WORD | ADD_ADDR_NUM_BYTE | SUB_REG_NUM_WORD | SUB_ADDR_REG_BYTE
            | SUB_ADDR_REG_WORD | SUB_ADDR_NUM_BYTE | ADD_REG_ADDR_BYTE | SUB_REG_ADDR_BYTE
            | ADD_REG_ADDR_WORD | SUB_REG_ADDR_WORD | OR_REG_NUM_WORD | XOR_REG_NUM_WORD
            | AND_REG_NUM_WORD => 3,
            CPY_ADDR_NUM_WORD | CPY_ADDR_ADDR_BYTE | CPY_ADDR_ADDR_WORD | ADD_ADDR_ADDR_WORD
            | ADD_ADDR_ADDR_BYTE | ADD_ADDR_NUM_WORD | SUB_ADDR_ADDR_WORD | SUB_ADDR_ADDR_BYTE
            | SUB_ADDR_NUM_WORD => 4,
            MEM_CPY_ADDR_ADDR_BYTE => 5,
            _ => {
                if cfg!(debug_assertions) {
                    panic!("Unknown op: {:02X}", op);
                } else {
                    0 //unknown ops are handled inside VM already
                }
            }
        }
    }

    pub const NOP: u8 = 0x00;
    pub const HALT: u8 = 0x01;
    pub const CALL_ADDR: u8 = 0x02;
    pub const CALL_REG: u8 = 0x03;
    pub const RETURN: u8 = 0x04;
    pub const RETURN_FROM_INTERRUPT: u8 = 0x05;
    pub const SWAP_REG_REG_BYTE: u8 = 0x06;
    pub const SWAP_REG_REG_WORD: u8 = 0x07;
    pub const MEM_CPY_ADDR_ADDR_BYTE: u8 = 0x08;
    //unused 7 0x09 - 0x0F

    pub const CPY_REG_REG_BYTE: u8 = 0x10;
    pub const CPY_REG_REG_WORD: u8 = 0x11;
    pub const CPY_ADDR_REG_BYTE: u8 = 0x12;
    pub const CPY_ADDR_REG_WORD: u8 = 0x13;
    pub const CPY_REG_ADDR_BYTE: u8 = 0x14;
    pub const CPY_REG_ADDR_WORD: u8 = 0x15;
    pub const CPY_ADDR_ADDR_BYTE: u8 = 0x16;
    pub const CPY_ADDR_ADDR_WORD: u8 = 0x17;
    pub const CPY_REG_NUM_BYTE: u8 = 0x18;
    pub const CPY_REG_NUM_WORD: u8 = 0x19;
    pub const CPY_ADDR_NUM_BYTE: u8 = 0x1A;
    pub const CPY_ADDR_NUM_WORD: u8 = 0x1B;
    //unused 4 0x1C - 0x1F

    pub const ADD_REG_REG_BYTE: u8 = 0x20;
    pub const ADD_REG_REG_WORD: u8 = 0x21;
    pub const ADD_REG_NUM_BYTE: u8 = 0x22;
    pub const ADD_REG_NUM_WORD: u8 = 0x23;
    pub const ADD_REG_ADDR_BYTE: u8 = 0x24;
    pub const ADD_REG_ADDR_WORD: u8 = 0x25;
    pub const ADD_ADDR_REG_BYTE: u8 = 0x26;
    pub const ADD_ADDR_REG_WORD: u8 = 0x27;
    pub const ADD_ADDR_NUM_BYTE: u8 = 0x28;
    pub const ADD_ADDR_NUM_WORD: u8 = 0x29;
    pub const ADD_ADDR_ADDR_BYTE: u8 = 0x2A;
    pub const ADD_ADDR_ADDR_WORD: u8 = 0x2B;
    pub const INC_REG_BYTE: u8 = 0x2C;
    pub const INC_REG_WORD: u8 = 0x2D;
    pub const INC_ADDR_BYTE: u8 = 0x2E;
    pub const INC_ADDR_WORD: u8 = 0x2F;

    pub const SUB_REG_REG_BYTE: u8 = 0x30;
    pub const SUB_REG_REG_WORD: u8 = 0x31;
    pub const SUB_REG_NUM_BYTE: u8 = 0x32;
    pub const SUB_REG_NUM_WORD: u8 = 0x33;
    pub const SUB_REG_ADDR_BYTE: u8 = 0x34;
    pub const SUB_REG_ADDR_WORD: u8 = 0x35;
    pub const SUB_ADDR_REG_BYTE: u8 = 0x36;
    pub const SUB_ADDR_REG_WORD: u8 = 0x37;
    pub const SUB_ADDR_NUM_BYTE: u8 = 0x38;
    pub const SUB_ADDR_NUM_WORD: u8 = 0x39;
    pub const SUB_ADDR_ADDR_BYTE: u8 = 0x3A;
    pub const SUB_ADDR_ADDR_WORD: u8 = 0x3B;
    pub const DEC_REG_BYTE: u8 = 0x3C;
    pub const DEC_REG_WORD: u8 = 0x3D;
    pub const DEC_ADDR_BYTE: u8 = 0x3E;
    pub const DEC_ADDR_WORD: u8 = 0x3F;

    pub const NOT_REG_BYTE: u8 = 0x40;
    pub const NOT_REG_WORD: u8 = 0x41;
    pub const OR_REG_REG_BYTE: u8 = 0x42;
    pub const OR_REG_REG_WORD: u8 = 0x43;
    pub const OR_REG_NUM_BYTE: u8 = 0x44;
    pub const OR_REG_NUM_WORD: u8 = 0x45;
    pub const XOR_REG_REG_BYTE: u8 = 0x46;
    pub const XOR_REG_REG_WORD: u8 = 0x47;
    pub const XOR_REG_NUM_BYTE: u8 = 0x48;
    pub const XOR_REG_NUM_WORD: u8 = 0x49;
    pub const AND_REG_REG_BYTE: u8 = 0x4A;
    pub const AND_REG_REG_WORD: u8 = 0x4B;
    pub const AND_REG_NUM_BYTE: u8 = 0x4C;
    pub const AND_REG_NUM_WORD: u8 = 0x4D;
    //unused 2 0x4E - 0x4F

    pub const ALL: [u8; 67] = [
        NOP,
        HALT,
        CALL_ADDR,
        CALL_REG,
        RETURN,
        CPY_REG_REG_BYTE,
        CPY_REG_REG_WORD,
        CPY_ADDR_REG_BYTE,
        CPY_ADDR_REG_WORD,
        CPY_REG_ADDR_BYTE,
        CPY_REG_ADDR_WORD,
        CPY_ADDR_ADDR_BYTE,
        CPY_ADDR_ADDR_WORD,
        CPY_REG_NUM_BYTE,
        CPY_REG_NUM_WORD,
        CPY_ADDR_NUM_BYTE,
        CPY_ADDR_NUM_WORD,
        ADD_REG_REG_BYTE,
        ADD_REG_REG_WORD,
        ADD_REG_NUM_BYTE,
        ADD_REG_NUM_WORD,
        ADD_REG_ADDR_BYTE,
        ADD_REG_ADDR_WORD,
        ADD_ADDR_REG_BYTE,
        ADD_ADDR_REG_WORD,
        ADD_ADDR_NUM_BYTE,
        ADD_ADDR_NUM_WORD,
        ADD_ADDR_ADDR_BYTE,
        ADD_ADDR_ADDR_WORD,
        INC_REG_BYTE,
        INC_REG_WORD,
        INC_ADDR_BYTE,
        INC_ADDR_WORD,
        SUB_REG_REG_BYTE,
        SUB_REG_REG_WORD,
        SUB_REG_NUM_BYTE,
        SUB_REG_NUM_WORD,
        SUB_REG_ADDR_BYTE,
        SUB_REG_ADDR_WORD,
        SUB_ADDR_REG_BYTE,
        SUB_ADDR_REG_WORD,
        SUB_ADDR_NUM_BYTE,
        SUB_ADDR_NUM_WORD,
        SUB_ADDR_ADDR_BYTE,
        SUB_ADDR_ADDR_WORD,
        DEC_REG_BYTE,
        DEC_REG_WORD,
        DEC_ADDR_BYTE,
        DEC_ADDR_WORD,
        RETURN_FROM_INTERRUPT,
        SWAP_REG_REG_BYTE,
        SWAP_REG_REG_WORD,
        MEM_CPY_ADDR_ADDR_BYTE,
        NOT_REG_BYTE,
        NOT_REG_WORD,
        OR_REG_REG_BYTE,
        OR_REG_REG_WORD,
        OR_REG_NUM_BYTE,
        OR_REG_NUM_WORD,
        XOR_REG_REG_BYTE,
        XOR_REG_REG_WORD,
        XOR_REG_NUM_BYTE,
        XOR_REG_NUM_WORD,
        AND_REG_REG_BYTE,
        AND_REG_REG_WORD,
        AND_REG_NUM_BYTE,
        AND_REG_NUM_WORD,
    ];
}

#[rustfmt::skip]
pub mod op_params {
    pub const MASK: u8 = 0xF0;
    pub mod bits {
        pub const INDIRECT: usize = 7;
        pub const OFFSET: usize = 6;
        pub const PRE_POST: usize = 5;
        pub const INC_DEC: usize = 4;
    }
    pub mod values {
        pub const REGISTER: u8 =     0b00000000;
        //pub const RESERVED: u8 =   0b00010000;
        //pub const RESERVED: u8 =   0b00100000;
        //pub const RESERVED: u8 =   0b00110000;
        pub const POST_INC: u8 =     0b01000000;
        pub const POST_DEC: u8 =     0b01010000;
        pub const PRE_INC: u8 =      0b01100000;
        pub const PRE_DEC: u8 =      0b01110000;
        pub const INDIRECT: u8 =     0b10000000;
        //pub const RESERVED: u8 =   0b10010000;
        //pub const RESERVED: u8 =   0b10100000;
        //pub const RESERVED: u8 =   0b10110000;
        pub const IND_POST_INC: u8 = 0b11000000;
        pub const IND_POST_DEC: u8 = 0b11010000;
        pub const IND_PRE_INC: u8 =  0b11100000;
        pub const IND_PRE_DEC: u8 =  0b11110000;
    }
}

#[cfg(test)]
mod test {
    use crate::constants::mem::{address, sizes};
    use crate::constants::ops;
    use crate::types::Address;

    #[test]
    fn check_totals() {
        assert_eq!(sizes::LAYER_TOTAL, 10572);
        assert_eq!(sizes::PALETTES_TOTAL, 180);
        assert_eq!(sizes::SPRITE_TABLE, 1020);
        assert_eq!(sizes::ATLAS_TOTAL, 16002);
        assert_eq!(sizes::GRAPHICS_TOTAL, 27774);
        assert_eq!(sizes::SYSTEM_TOTAL, 33637);
        assert_eq!(sizes::HARDWARE_TOTAL, 4124);
        assert_eq!(sizes::TOTAL, 65535);
    }

    #[test]
    fn check_addresses() {
        assert_eq!(address::CODE, Address::ZERO);
        assert_eq!(address::CODE_BANK, address::CODE + sizes::CODE_BANK);
        assert_eq!(address::RAM, address::CODE_BANK + sizes::CODE_BANK);
        assert_eq!(address::RAM_BANK, address::RAM + sizes::RAM_BANK);
        assert_eq!(address::INPUT, address::RAM_BANK + sizes::RAM_BANK);
        assert_eq!(address::SOUND, address::INPUT + sizes::INPUT);
        assert_eq!(address::SAVE_BANK_ID, address::SOUND + sizes::SOUND);
        assert_eq!(
            address::SAVE_BANK,
            address::SAVE_BANK_ID + sizes::SAVE_BANK_ID
        );
        assert_eq!(address::ATLAS1, address::SAVE_BANK + sizes::SAVE_BANK);
        assert_eq!(address::ATLAS2, address::ATLAS1 + sizes::ATLAS);
        assert_eq!(address::PALETTES, address::ATLAS2 + sizes::ATLAS);
        assert_eq!(
            address::SPRITE_TABLE,
            address::PALETTES + sizes::PALETTES_TOTAL
        );
        assert_eq!(address::LAYERS, address::SPRITE_TABLE + sizes::SPRITE_TABLE);
        assert_eq!(
            address::LAYERS,
            address::LAYER_HEADERS + sizes::LAYERS_HEADER
        );
        assert_eq!(address::CODE_BANK_ID, address::LAYERS + sizes::LAYER_TOTAL);
        assert_eq!(
            address::RAM_BANK_ID,
            address::CODE_BANK_ID + sizes::CODE_BANK_ID
        );
        assert_eq!(
            address::ATLAS1_BANK_ID,
            address::RAM_BANK_ID + sizes::RAM_BANK_ID
        );
        assert_eq!(
            address::ATLAS2_BANK_ID,
            address::ATLAS1_BANK_ID + sizes::ATLAS1_BANK_ID
        );
        assert_eq!(address::SP, address::ATLAS2_BANK_ID + sizes::ATLAS2_BANK_ID);
        assert_eq!(address::FP, address::SP + sizes::SP);
        assert_eq!(address::STACK, address::FP + sizes::FP);
        assert_eq!(Address(address::MAX), address::STACK + sizes::STACK);
    }

    #[test]
    #[allow(unused_comparisons)]
    fn check_all_commands_have_byte_counts() {
        for op in ops::ALL {
            assert!(ops::get_byte_count(op) >= 0, "{:02X}", op);
        }
    }
}
