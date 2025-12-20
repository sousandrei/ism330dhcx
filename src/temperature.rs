use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;

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

        let raw_temp = (measurements[1] as i16) << 8 | measurements[0] as i16;
        let temp: f32 = (raw_temp as f32 / 256.0) + 25.0;

        Ok(temp)
    }
}
