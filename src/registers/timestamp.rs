use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Timestamp registers (40h - 43h)
    pub struct Timestamp(u32);
    impl Debug;
    pub value, set_value: 32, 0;
}

impl Timestamp {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u32; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u32; 1] {
        [self.0]
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for TIMESTAMP registers.
pub trait TimestampConfig {
    /// Read the internal 24-bit timestamp (returned as u32).
    fn get_timestamp<I2C>(&self, i2c: &mut I2C) -> Result<u32, I2C::Error>
    where
        I2C: I2c;
}

impl TimestampConfig for Ism330Dhcx {
    fn get_timestamp<I2C>(&self, i2c: &mut I2C) -> Result<u32, I2C::Error>
    where
        I2C: I2c,
    {
        let mut buffer = [0u8; 4];
        i2c.write_read(self.address, &[Register::Timestamp0.addr()], &mut buffer)?;
        // The timestamp is 24-bit, but stored in 4 bytes (last byte is ignored/zero)
        Ok(u32::from_le_bytes(buffer) & 0x00FFFFFF)
    }
}
