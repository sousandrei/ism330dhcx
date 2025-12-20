#![allow(unused_parens)]
use modular_bitfield::{
    BitfieldSpecifier, bitfield,
    specifiers::{B5, B6},
};

/// FIFO control register 2.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FifoCtrl2 {
    #[skip]
    pub __: B6,
    /// FIFO compression enable.
    pub fifo_compr_rt_en: bool,
    /// Stop on watermark enable.
    pub stop_on_wtm: bool,
}

impl Default for FifoCtrl2 {
    fn default() -> Self {
        Self::new()
    }
}

/// Gyroscope batch data rate.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 4]
pub enum BdrGy {
    /// Not batched
    Off = 0b0000,
    /// 6.5 Hz
    Hz6_5 = 0b1011,
    /// 12.5 Hz
    Hz12_5 = 0b0001,
    /// 26 Hz
    Hz26 = 0b0010,
    /// 52 Hz
    Hz52 = 0b0011,
    /// 104 Hz
    Hz104 = 0b0100,
    /// 208 Hz
    Hz208 = 0b0101,
    /// 417 Hz
    Hz417 = 0b0110,
    /// 833 Hz
    Hz833 = 0b0111,
    /// 1.667 kHz
    Hz1667 = 0b1000,
    /// 3.333 kHz
    Hz3333 = 0b1001,
    /// 6.667 kHz
    Hz6667 = 0b1010,
}

/// Accelerometer batch data rate.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 4]
pub enum BdrXl {
    /// Not batched
    Off = 0b0000,
    /// 6.5 Hz
    Hz6_5 = 0b1011,
    /// 12.5 Hz
    Hz12_5 = 0b0001,
    /// 26 Hz
    Hz26 = 0b0010,
    /// 52 Hz
    Hz52 = 0b0011,
    /// 104 Hz
    Hz104 = 0b0100,
    /// 208 Hz
    Hz208 = 0b0101,
    /// 417 Hz
    Hz417 = 0b0110,
    /// 833 Hz
    Hz833 = 0b0111,
    /// 1.667 kHz
    Hz1667 = 0b1000,
    /// 3.333 kHz
    Hz3333 = 0b1001,
    /// 6.667 kHz
    Hz6667 = 0b1010,
}

/// FIFO control register 3.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FifoCtrl3 {
    /// Accelerometer batch data rate.
    pub bdr_xl: BdrXl,
    /// Gyroscope batch data rate.
    pub bdr_gy: BdrGy,
}

impl Default for FifoCtrl3 {
    fn default() -> Self {
        Self::new()
    }
}

/// FIFO mode selection.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 3]
pub enum FifoMode {
    /// Bypass mode. FIFO disabled.
    Bypass = 0b000,
    /// FIFO mode. Stops collecting when full.
    FifoMode = 0b001,
    /// Continuous-to-FIFO mode.
    ContinuousToFifo = 0b011,
    /// Bypass-to-Continuous mode.
    BypassToContinuous = 0b100,
    /// Continuous mode. Overwrites oldest when full.
    Continuous = 0b110,
    /// Bypass-to-FIFO mode.
    BypassToFifo = 0b111,
}

/// FIFO control register 4.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FifoCtrl4 {
    /// FIFO mode selection.
    pub fifo_mode: FifoMode,
    #[skip]
    pub __: B5,
}

impl Default for FifoCtrl4 {
    fn default() -> Self {
        Self::new()
    }
}
