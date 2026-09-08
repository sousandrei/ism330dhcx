#![allow(unused_parens)]
use modular_bitfield::{bitfield, specifiers::B1, BitfieldSpecifier};

/// Gyroscope high-pass filter cutoff selection.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum HpmG {
    /// 16 mHz
    Hpmg16 = 0b00,
    /// 65 mHz
    Hpmg65 = 0b01,
    /// 260 mHz
    Hpmg260 = 0b10,
    /// 1.04 Hz
    Hpmg104 = 0b11,
}

/// Control register 7 (Gyroscope).
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl7G {
    /// OIS enable.
    pub ois_on: bool,
    /// User off on out enable.
    pub usr_off_on_out: bool,
    /// OIS enable.
    pub ois_on_en: bool,
    /// High-pass filter cutoff freq.
    pub hpm_g: HpmG,
    #[skip]
    pub __: B1,
    /// High-pass filter enable.
    pub hp_en_g: bool,
    /// High-performance operating mode disable (0 = enabled).
    pub g_hm_mode: bool,
}

impl Default for Ctrl7G {
    fn default() -> Self {
        Self::new()
    }
}
