use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

/// Threshold for 4D/6D function.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum SixdThs {
    /// 80 degrees
    Deg80 = 0b00,
    /// 70 degrees
    Deg70 = 0b01,
    /// 60 degrees
    Deg60 = 0b10,
    /// 50 degrees
    Deg50 = 0b11,
}

impl From<u8> for SixdThs {
    fn from(val: u8) -> Self {
        match val {
            0b00 => SixdThs::Deg80,
            0b01 => SixdThs::Deg70,
            0b10 => SixdThs::Deg60,
            0b11 => SixdThs::Deg50,
            _ => SixdThs::Deg80,
        }
    }
}

impl From<SixdThs> for u8 {
    fn from(val: SixdThs) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Portrait/landscape position and tap function threshold register (59h)
    pub struct TapThs6d(u8);
    impl Debug;
    /// Z-axis recognition threshold.
    pub tap_ths_z, set_tap_ths_z: 4, 0;
    /// Threshold for 4D/6D function.
    pub from into SixdThs, sixd_ths, set_sixd_ths: 6, 5;
    /// Enables detection of 4D orientation.
    pub d4d_en, set_d4d_en: 7;
}

impl TapThs6d {
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

impl Default for TapThs6d {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for TAP_THS_6D register.
pub trait TapThs6dConfig {
    /// Z-axis recognition threshold.
    fn set_tap_ths_z<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Threshold for 4D/6D function.
    fn set_sixd_ths<I2C>(&self, i2c: &mut I2C, val: SixdThs) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enables detection of 4D orientation.
    fn set_d4d_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl TapThs6dConfig for Ism330Dhcx {
    fn set_tap_ths_z<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::TapThs6d, |v| {
            let mut reg = TapThs6d::from_bytes([v]);
            reg.set_tap_ths_z(val);
            reg.into_bytes()[0]
        })
    }

    fn set_sixd_ths<I2C>(&self, i2c: &mut I2C, val: SixdThs) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::TapThs6d, |v| {
            let mut reg = TapThs6d::from_bytes([v]);
            reg.set_sixd_ths(val);
            reg.into_bytes()[0]
        })
    }

    fn set_d4d_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::TapThs6d, |v| {
            let mut reg = TapThs6d::from_bytes([v]);
            reg.set_d4d_en(val);
            reg.into_bytes()[0]
        })
    }
}
