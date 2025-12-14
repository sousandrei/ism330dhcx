#![allow(unused_parens)]
use modular_bitfield::{BitfieldSpecifier, bitfield};

#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum FsGScale {
    Dps250 = 0b00,
    Dps500 = 0b01,
    Dps1000 = 0b10,
    Dps2000 = 0b11,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, defmt::Format)]
pub enum FsG {
    Dps125,
    Dps250,
    Dps500,
    Dps1000,
    Dps2000,
    Dps4000,
}

impl FsG {
    pub fn sensitivity(&self) -> f32 {
        match self {
            FsG::Dps125 => 4.375,
            FsG::Dps250 => 8.750,
            FsG::Dps500 => 17.50,
            FsG::Dps1000 => 35.,
            FsG::Dps2000 => 70.,
            FsG::Dps4000 => 140.,
        }
    }
}

#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 4]
pub enum OdrG {
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
pub struct Ctrl2G {
    pub fs_4000: bool,
    pub fs_125: bool,
    pub fs_g: FsGScale,
    pub odr_g: OdrG,
}

impl Default for Ctrl2G {
    fn default() -> Self {
        Self::new()
    }
}
