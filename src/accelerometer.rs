use crate::registers::FsXl;

/// Standard gravity constant [m/s²]
pub const SENSORS_GRAVITY_STANDARD: f64 = 9.80665;

/// High-level accelerometer reading.
#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct AccelValue {
    range: FsXl,
    count: [i16; 3],
}

impl AccelValue {
    /// Create a new `AccelValue` from raw counts and range.
    pub fn new(range: FsXl, count: [i16; 3]) -> AccelValue {
        AccelValue { range, count }
    }

    /// Create a new `AccelValue` from raw byte measurements (little-endian).
    pub fn from_msr(range: FsXl, measurements: &[u8; 6]) -> AccelValue {
        let raw_acc_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_acc_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_acc_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);
        AccelValue {
            range,
            count: [raw_acc_x, raw_acc_y, raw_acc_z],
        }
    }

    /// Return the raw signed integer counts for X, Y, Z axes.
    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// Return acceleration in meters per second squared [m/s²].
    pub fn as_m_ss(&self) -> [f64; 3] {
        self.as_mg().map(|v| v * SENSORS_GRAVITY_STANDARD / 1000.)
    }

    /// Return acceleration in milli-g \[mg\].
    pub fn as_mg(&self) -> [f64; 3] {
        let sensitivity = self.range.sensitivity() as f64;
        self.count.map(|r| r as f64 * sensitivity)
    }

    /// Return acceleration in g \[g\].
    pub fn as_g(&self) -> [f64; 3] {
        self.as_mg().map(|v| v / 1000.)
    }
}

use crate::Ism330Dhcx;
use crate::registers::{Ctrl1Xl, OdrXl, Register};
use embedded_hal::i2c::I2c;

/// Accelerometer sensor methods.
pub trait Accelerometer {
    /// Set accelerometer output data rate.
    fn set_accel_odr<I2C>(&mut self, i2c: &mut I2C, odr: OdrXl) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set accelerometer full-scale range.
    fn set_accel_scale<I2C>(&mut self, i2c: &mut I2C, scale: FsXl) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Get current accelerometer full-scale range.
    fn get_accel_scale<I2C>(&self, i2c: &mut I2C) -> Result<FsXl, I2C::Error>
    where
        I2C: I2c;
    /// Enable low-pass filter 2 for accelerometer.
    fn set_lpf2_xl_en<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set DEN value on X axis.
    fn set_den_x<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set DEN value on Y axis.
    fn set_den_y<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set DEN value on Z axis.
    fn set_den_z<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set DEN device configuration.
    fn set_den_device_conf<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Get accelerometer reading.
    fn get_accelerometer<I2C>(&self, i2c: &mut I2C) -> Result<AccelValue, I2C::Error>
    where
        I2C: I2c;
}

impl Accelerometer for Ism330Dhcx {
    fn set_accel_odr<I2C>(&mut self, i2c: &mut I2C, odr: OdrXl) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_odr_xl(odr);
            reg.into_bytes()[0]
        })
    }

    fn set_accel_scale<I2C>(&mut self, i2c: &mut I2C, scale: FsXl) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_fs_xl(scale);
            reg.into_bytes()[0]
        })
    }

    fn get_accel_scale<I2C>(&self, i2c: &mut I2C) -> Result<FsXl, I2C::Error>
    where
        I2C: I2c,
    {
        let v = self.read_reg(i2c, Register::Ctrl1Xl)?;
        let reg = Ctrl1Xl::from_bytes([v]);
        Ok(reg.fs_xl())
    }

    fn set_lpf2_xl_en<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_lpf2_xl_en(enable);
            reg.into_bytes()[0]
        })
    }

    fn set_den_x<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = crate::registers::Ctrl9Xl::from_bytes([v]);
            reg.set_den_x(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_y<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = crate::registers::Ctrl9Xl::from_bytes([v]);
            reg.set_den_y(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_z<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = crate::registers::Ctrl9Xl::from_bytes([v]);
            reg.set_den_z(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_device_conf<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = crate::registers::Ctrl9Xl::from_bytes([v]);
            reg.set_device_conf(val);
            reg.into_bytes()[0]
        })
    }

    fn get_accelerometer<I2C>(&self, i2c: &mut I2C) -> Result<AccelValue, I2C::Error>
    where
        I2C: I2c,
    {
        let scale = self.get_accel_scale(i2c)?;

        let mut measurements = [0u8; 6];
        i2c.write_read(self.address, &[0x28], &mut measurements)?;

        Ok(AccelValue::from_msr(scale, &measurements))
    }
}
