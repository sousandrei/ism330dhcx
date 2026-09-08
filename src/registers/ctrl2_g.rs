#![allow(unused_parens)]
use modular_bitfield::{Specifier, bitfield};

/// Gyroscope full-scale selection.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum FsGScale {
    /// ±250 dps
    Dps250 = 0b00,
    /// ±500 dps
    Dps500 = 0b01,
    /// ±1000 dps
    Dps1000 = 0b10,
    /// ±2000 dps
    Dps2000 = 0b11,
}

/// Helper enum for all available gyroscope full-scale ranges.
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
    /// Returns sensitivity in mdps/LSB.
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

/// Gyroscope output data rate.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 4]
pub enum OdrG {
    /// Power-down
    Off = 0b0000,
    /// 12.5 Hz (high performance)
    Hz12_5 = 0b0001,
    /// 26 Hz (high performance)
    Hz26 = 0b0010,
    /// 52 Hz (high performance)
    Hz52 = 0b0011,
    /// 104 Hz (high performance)
    Hz104 = 0b0100,
    /// 208 Hz (high performance)
    Hz208 = 0b0101,
    /// 416 Hz (high performance)
    Hz416 = 0b0110,
    /// 833 Hz (high performance)
    Hz833 = 0b0111,
    /// 1.667 kHz (high performance)
    Hz1667 = 0b1000,
    /// 3.333 kHz (high performance)
    Hz3333 = 0b1001,
    /// 6.667 kHz (high performance)
    Hz6667 = 0b1010,
}

/// Control register 2 (Gyroscope).
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl2G {
    /// Full-scale 4000 dps enable.
    pub fs_4000: bool,
    /// Full-scale 125 dps enable.
    pub fs_125: bool,
    /// Full-scale selection.
    pub fs_g: FsGScale,
    /// Output data rate selection.
    pub odr_g: OdrG,
}

impl Default for Ctrl2G {
    fn default() -> Self {
        Self::new()
    }
}
