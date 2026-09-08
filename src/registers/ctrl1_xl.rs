#![allow(unused_parens)]
use modular_bitfield::{Specifier, bitfield, specifiers::B1};

/// Accelerometer full-scale selection.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum FsXl {
    /// ±2 g
    G2 = 0b00,
    /// ±16 g
    G16 = 0b01,
    /// ±4 g
    G4 = 0b10,
    /// ±8 g
    G8 = 0b11,
}

impl FsXl {
    /// Returns sensitivity in mg/LSB.
    pub fn sensitivity(&self) -> f32 {
        match self {
            FsXl::G2 => 0.061,
            FsXl::G4 => 0.122,
            FsXl::G8 => 0.244,
            FsXl::G16 => 0.488,
        }
    }
}

/// Accelerometer output data rate selection.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 4]
pub enum OdrXl {
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

impl OdrXl {
    /// Calculate the actual output data rate using `INTERNAL_FREQ_FINE`.
    pub fn actual_hz(self, frequency_fine: i8) -> f32 {
        let coefficient = match self {
            Self::Off => return 0.0,
            Self::Hz12_5 => 512.0,
            Self::Hz26 => 256.0,
            Self::Hz52 => 128.0,
            Self::Hz104 => 64.0,
            Self::Hz208 => 32.0,
            Self::Hz416 => 16.0,
            Self::Hz833 => 8.0,
            Self::Hz1667 => 4.0,
            Self::Hz3333 => 2.0,
            Self::Hz6667 => 1.0,
        };
        (6667.0 + 0.0015 * frequency_fine as f32 * 6667.0) / coefficient
    }
}

/// Control register 1 (XL) - Accelerometer settings.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Xl {
    #[skip]
    pub __: B1,
    /// Low-pass filter 2 enable.
    pub lpf2_xl_en: bool,
    /// Full-scale selection.
    pub fs_xl: FsXl,
    /// Output data rate selection.
    pub odr_xl: OdrXl,
}

impl Default for Ctrl1Xl {
    fn default() -> Self {
        Self::new()
    }
}
