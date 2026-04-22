use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Status register (1Eh)
    pub struct StatusReg(u8);
    impl Debug;
    /// Accelerometer new data available.
    pub xlda, _: 0;
    /// Gyroscope new data available.
    pub gda, _: 1;
    /// Temperature new data available.
    pub tda, _: 2;
}

impl StatusReg {
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

impl Default for StatusReg {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for STATUS_REG register.
pub trait StatusRegConfig {
    /// Read the status register to check for new data.
    fn get_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusReg, I2C::Error>
    where
        I2C: I2c;
}

impl StatusRegConfig for Ism330Dhcx {
    fn get_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusReg, I2C::Error>
    where
        I2C: I2c,
    {
        let v = self.read_reg(i2c, Register::StatusReg)?;
        Ok(StatusReg::from_bytes([v]))
    }
}
