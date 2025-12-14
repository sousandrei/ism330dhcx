#![allow(unused_parens)]
use modular_bitfield::{BitfieldSpecifier, bitfield, specifiers::B1};

#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum FsXl {
    G2 = 0b00,
    G16 = 0b01,
    G4 = 0b10,
    G8 = 0b11,
}

impl FsXl {
    pub fn sensitivity(&self) -> f32 {
        match self {
            FsXl::G2 => 0.061,
            FsXl::G4 => 0.122,
            FsXl::G8 => 0.244,
            FsXl::G16 => 0.488,
        }
    }
}

#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 4]
pub enum OdrXl {
    Off = 0b0000,
    Hz12_5 = 0b0001,
    Hz26 = 0b0010,
    Hz52 = 0b0011,
    Hz104 = 0b0100,
    Hz208 = 0b0101,
    Hz416 = 0b0110,
    Hz833 = 0b0111,
    Hz1667 = 0b1000,
    Hz3333 = 0b1001,
    Hz6667 = 0b1010,
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl1Xl {
    #[skip]
    pub __: B1,
    pub lpf2_xl_en: bool,
    pub fs_xl: FsXl,
    pub odr_xl: OdrXl,
}
