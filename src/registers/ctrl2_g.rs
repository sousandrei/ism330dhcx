use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

/// Gyroscope full-scale selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
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

impl From<u8> for FsGScale {
    fn from(val: u8) -> Self {
        match val {
            0b00 => FsGScale::Dps250,
            0b01 => FsGScale::Dps500,
            0b10 => FsGScale::Dps1000,
            0b11 => FsGScale::Dps2000,
            _ => FsGScale::Dps250,
        }
    }
}

impl From<FsGScale> for u8 {
    fn from(val: FsGScale) -> u8 {
        val as u8
    }
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
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
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

impl From<u8> for OdrG {
    fn from(val: u8) -> Self {
        match val {
            0b0000 => OdrG::Off,
            0b0001 => OdrG::Hz12_5,
            0b0010 => OdrG::Hz26,
            0b0011 => OdrG::Hz52,
            0b0100 => OdrG::Hz104,
            0b0101 => OdrG::Hz208,
            0b0110 => OdrG::Hz416,
            0b0111 => OdrG::Hz833,
            0b1000 => OdrG::Hz1667,
            0b1001 => OdrG::Hz3333,
            0b1010 => OdrG::Hz6667,
            _ => OdrG::Off,
        }
    }
}

impl From<OdrG> for u8 {
    fn from(val: OdrG) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Control register 2 (Gyroscope).
    pub struct Ctrl2G(u8);
    impl Debug;
    /// Full-scale 4000 dps enable.
    pub fs_4000, set_fs_4000: 0;
    /// Full-scale 125 dps enable.
    pub fs_125, set_fs_125: 1;
    /// Full-scale selection.
    pub from into FsGScale, fs_g, set_fs_g: 3, 2;
    /// Output data rate selection.
    pub from into OdrG, odr_g, set_odr_g: 7, 4;
}

impl Ctrl2G {
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

impl Default for Ctrl2G {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CTRL2_G register.
pub trait Ctrl2GConfig {
    /// Full-scale 4000 dps enable.
    fn set_fs_4000<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Full-scale 125 dps enable.
    fn set_fs_125<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Full-scale selection.
    fn set_fs_g<I2C>(&self, i2c: &mut I2C, val: FsGScale) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Output data rate selection.
    fn set_odr_g<I2C>(&self, i2c: &mut I2C, val: OdrG) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ctrl2GConfig for Ism330Dhcx {
    fn set_fs_4000<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl2G, |v| {
            let mut reg = Ctrl2G::from_bytes([v]);
            reg.set_fs_4000(val);
            reg.into_bytes()[0]
        })
    }

    fn set_fs_125<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl2G, |v| {
            let mut reg = Ctrl2G::from_bytes([v]);
            reg.set_fs_125(val);
            reg.into_bytes()[0]
        })
    }

    fn set_fs_g<I2C>(&self, i2c: &mut I2C, val: FsGScale) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl2G, |v| {
            let mut reg = Ctrl2G::from_bytes([v]);
            reg.set_fs_g(val);
            reg.into_bytes()[0]
        })
    }

    fn set_odr_g<I2C>(&self, i2c: &mut I2C, val: OdrG) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl2G, |v| {
            let mut reg = Ctrl2G::from_bytes([v]);
            reg.set_odr_g(val);
            reg.into_bytes()[0]
        })
    }
}
