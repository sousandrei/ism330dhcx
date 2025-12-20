use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Control register 10 (19h)
    pub struct Ctrl10C(u8);
    impl Debug;
    /// Enables timestamp counter.
    pub timestamp_en, set_timestamp_en: 5;
}

impl Ctrl10C {
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

impl Default for Ctrl10C {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CTRL10_C register.
pub trait Ctrl10CConfig {
    /// Enables timestamp counter.
    fn set_timestamp_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ctrl10CConfig for Ism330Dhcx {
    fn set_timestamp_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl10C, |v| {
            let mut reg = Ctrl10C::from_bytes([v]);
            reg.set_timestamp_en(val);
            reg.into_bytes()[0]
        })
    }
}
