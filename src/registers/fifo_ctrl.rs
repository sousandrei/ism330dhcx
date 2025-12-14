#![allow(unused_parens)]
use modular_bitfield::{
    BitfieldSpecifier, bitfield,
    specifiers::{B5, B6},
};

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FifoCtrl2 {
    #[skip]
    pub __: B6,
    pub fifo_compr_rt_en: bool,
    pub stop_on_wtm: bool,
}

#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 4]
pub enum BdrGy {
    Off = 0b0000,
    Hz6_5 = 0b1011,
    Hz12_5 = 0b0001,
    Hz26 = 0b0010,
    Hz52 = 0b0011,
    Hz104 = 0b0100,
    Hz208 = 0b0101,
    Hz417 = 0b0110,
    Hz833 = 0b0111,
    Hz1667 = 0b1000,
    Hz3333 = 0b1001,
    Hz6667 = 0b1010,
}

#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 4]
pub enum BdrXl {
    Off = 0b0000,
    Hz6_5 = 0b1011,
    Hz12_5 = 0b0001,
    Hz26 = 0b0010,
    Hz52 = 0b0011,
    Hz104 = 0b0100,
    Hz208 = 0b0101,
    Hz417 = 0b0110,
    Hz833 = 0b0111,
    Hz1667 = 0b1000,
    Hz3333 = 0b1001,
    Hz6667 = 0b1010,
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FifoCtrl3 {
    pub bdr_xl: BdrXl,
    pub bdr_gy: BdrGy,
}

#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 3]
pub enum FifoMode {
    Bypass = 0b000,
    FifoMode = 0b001,
    ContinuousToFifo = 0b011,
    BypassToContinuous = 0b100,
    Continuous = 0b110,
    BypassToFifo = 0b111,
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FifoCtrl4 {
    pub fifo_mode: FifoMode,
    #[skip]
    pub __: B5,
}
