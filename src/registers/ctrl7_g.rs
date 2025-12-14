#![allow(unused_parens)]
use modular_bitfield::{BitfieldSpecifier, bitfield, specifiers::B1};

#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum HpmG {
    Hpmg16 = 0b00,  // ±250 mHz
    Hpmg65 = 0b01,  // ±500 mHz
    Hpmg260 = 0b10, // ±1000 mHz
    Hpmg104 = 0b11, // ±4000 Hz
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl7G {
    pub ois_on: bool,
    pub usr_off_on_out: bool,
    pub ois_on_en: bool,
    pub hpm_g: HpmG,
    #[skip]
    pub __: B1,
    pub hp_en_g: bool,
    pub g_hm_mode: bool,
}
