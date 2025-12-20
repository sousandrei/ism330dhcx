use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Internal frequency register (63h)
    pub struct InternalFreqFine(u8);
    impl Debug;
    /// Difference in percentage of the effective ODR. 8-bit format, 2's complement.
    pub freq_fine, set_freq_fine: 7, 0;
}

impl InternalFreqFine {
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

impl Default for InternalFreqFine {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for INTERNAL_FREQ_FINE register.
pub trait InternalFreqFineConfig {
    /// Difference in percentage of the effective ODR. 8-bit format, 2's complement.
    fn set_freq_fine<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl InternalFreqFineConfig for Ism330Dhcx {
    fn set_freq_fine<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::InternalFreqFine, |v| {
            let mut reg = InternalFreqFine::from_bytes([v]);
            reg.set_freq_fine(val);
            reg.into_bytes()[0]
        })
    }
}
