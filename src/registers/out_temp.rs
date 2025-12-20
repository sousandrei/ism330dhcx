use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Temperature data output register (r). L and H registers together express a 16-bit word in two’s complement.
    pub struct OutTemp(u16);
    impl Debug;
    /// Temperature data output.
    pub temperature, _: 15, 0;
}

impl OutTemp {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }

    pub fn into_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }

    /// Returns the raw temperature value as a signed 16-bit integer.
    pub fn value(&self) -> i16 {
        self.0 as i16
    }

    /// Returns the temperature in degrees Celsius.
    /// The conversion formula is (raw_temp / 256.0) + 25.0.
    pub fn get_celsius(&self) -> f32 {
        (self.value() as f32 / 256.0) + 25.0
    }
}

impl Default for OutTemp {
    fn default() -> Self {
        Self::new()
    }
}

/// Temperature sensor methods.
pub trait Temperature {
    /// Get temperature in Celsius.
    fn get_temperature<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: I2c;
}

impl Temperature for Ism330Dhcx {
    fn get_temperature<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: I2c,
    {
        let mut measurements = [0u8; 2];
        i2c.write_read(
            self.address,
            &[Register::OutTempL.addr()],
            &mut measurements,
        )?;

        let temp = OutTemp::from_bytes(measurements);

        Ok(temp.get_celsius())
    }
}
