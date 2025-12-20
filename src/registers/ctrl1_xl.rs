use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

/// Accelerometer full-scale selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
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

impl From<u8> for FsXl {
    fn from(val: u8) -> Self {
        match val {
            0b00 => FsXl::G2,
            0b01 => FsXl::G16,
            0b10 => FsXl::G4,
            0b11 => FsXl::G8,
            _ => FsXl::G2,
        }
    }
}

impl From<FsXl> for u8 {
    fn from(val: FsXl) -> u8 {
        val as u8
    }
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
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

impl From<u8> for OdrXl {
    fn from(val: u8) -> Self {
        match val {
            0b0000 => OdrXl::Off,
            0b0001 => OdrXl::Hz12_5,
            0b0010 => OdrXl::Hz26,
            0b0011 => OdrXl::Hz52,
            0b0100 => OdrXl::Hz104,
            0b0101 => OdrXl::Hz208,
            0b0110 => OdrXl::Hz416,
            0b0111 => OdrXl::Hz833,
            0b1000 => OdrXl::Hz1667,
            0b1001 => OdrXl::Hz3333,
            0b1010 => OdrXl::Hz6667,
            _ => OdrXl::Off,
        }
    }
}

impl From<OdrXl> for u8 {
    fn from(val: OdrXl) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Control register 1 (XL) - Accelerometer settings.
    pub struct Ctrl1Xl(u8);
    impl Debug;
    /// Low-pass filter 2 enable.
    pub lpf2_xl_en, set_lpf2_xl_en: 1;
    /// Full-scale selection.
    pub from into FsXl, fs_xl, set_fs_xl: 3, 2;
    /// Output data rate selection.
    pub from into OdrXl, odr_xl, set_odr_xl: 7, 4;
}

impl Ctrl1Xl {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for Ctrl1Xl {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CTRL1_XL register.
pub trait Ctrl1XlConfig {
    /// Low-pass filter 2 enable.
    fn set_lpf2_xl_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Full-scale selection.
    fn set_fs_xl<I2C>(&self, i2c: &mut I2C, val: FsXl) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Output data rate selection.
    fn set_odr_xl<I2C>(&self, i2c: &mut I2C, val: OdrXl) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ctrl1XlConfig for Ism330Dhcx {
    fn set_lpf2_xl_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_lpf2_xl_en(val);
            reg.into_bytes()[0]
        })
    }

    fn set_fs_xl<I2C>(&self, i2c: &mut I2C, val: FsXl) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_fs_xl(val);
            reg.into_bytes()[0]
        })
    }

    fn set_odr_xl<I2C>(&self, i2c: &mut I2C, val: OdrXl) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_odr_xl(val);
            reg.into_bytes()[0]
        })
    }
}
