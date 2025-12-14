//! This is a simple driver for ST's `ism330dhcx` sensor.
//!
//! # Quick Start
//! To declare a sensor is pretty simple:
//!
//! ```rust,ignore
//! let sensor = Ism330Dhcx::new(i2c).unwrap();
//! ```
//!
//! The driver now owns the I2C bus.
//!
//! To configure the sensor, use the high-level methods:
//!
//! ```rust,ignore
//! sensor.set_accel_odr(OdrXl::Hz52).unwrap();
//! sensor.set_boot(true).unwrap();
//! ```
//!
//! # Reference
//!
//! - [Sensor page](https://www.st.com/en/mems-and-sensors/ism330dhcx.html)
//! - [Datasheet](https://www.st.com/resource/en/datasheet/ism330dhcx.pdf)

#![cfg_attr(not(test), no_std)]

pub mod accelerometer;
pub mod fifo;
pub mod gyroscope;
pub mod registers;

pub use accelerometer::{AccelValue, Accelerometer, SENSORS_GRAVITY_STANDARD};
pub use fifo::Fifo;
pub use gyroscope::{GyroValue, Gyroscope, SENSORS_DPS_TO_RADS};

use embedded_hal::i2c::I2c;
use registers::*;

/// Datasheet write address for the device. (D6h)
pub const DEFAULT_I2C_ADDRESS: u8 = 0x6bu8;

// SENSORS_DPS_TO_RADS moved to gyroscope.rs
// SENSORS_GRAVITY_STANDARD moved to accelerometer.rs

/// Driver for the ISM330DHCX sensor.
pub struct Ism330Dhcx<I2C> {
    /// I2C address.
    pub address: u8,
    i2c: I2C,
}

impl<I2C, E> Ism330Dhcx<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Create a new driver instance with the default I2C address (0x6B).
    pub fn new(i2c: I2C) -> Result<Self, E> {
        Self::new_with_address(i2c, DEFAULT_I2C_ADDRESS)
    }

    /// Create a new driver instance with a specific I2C address.
    pub fn new_with_address(i2c: I2C, address: u8) -> Result<Self, E> {
        Ok(Self { address, i2c })
    }

    /// Destroy the driver and return the underlying I2C interface.
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    pub(crate) fn read_reg(&mut self, reg: Register) -> Result<u8, E> {
        let mut buffer = [0u8];
        self.i2c
            .write_read(self.address, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }

    pub(crate) fn write_reg(&mut self, reg: Register, value: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[reg.addr(), value])
    }

    pub(crate) fn modify_reg<F>(&mut self, reg: Register, f: F) -> Result<(), E>
    where
        F: FnOnce(u8) -> u8,
    {
        let value = self.read_reg(reg)?;
        let new_value = f(value);
        self.write_reg(reg, new_value)
    }

    /// Set the I2C address.
    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }

    // ===========================================
    // Configuration
    // ===========================================

    /// Reboot memory content.
    pub fn set_boot(&mut self, boot: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_boot(boot);
            reg.into_bytes()[0]
        })
    }

    /// Block Data Update.
    ///
    /// If true, output registers are not updated until MSB and LSB have been read.
    pub fn set_bdu(&mut self, bdu: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_bdu(bdu);
            reg.into_bytes()[0]
        })
    }

    /// Register address automatically incremented during a multiple byte access with a serial interface.
    pub fn set_if_inc(&mut self, if_inc: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_if_inc(if_inc);
            reg.into_bytes()[0]
        })
    }

    // ===========================================
    // Sensors
    // ===========================================

    /// Get temperature in Celsius.
    pub fn get_temperature(&mut self) -> Result<f32, E> {
        let mut measurements = [0u8; 2];
        self.i2c
            .write_read(self.address, &[0x20], &mut measurements)?;

        let raw_temp = (measurements[1] as i16) << 8 | measurements[0] as i16;
        let temp: f32 = (raw_temp as f32 / 256.0) + 25.0;

        Ok(temp)
    }

    /// Set chain full scale.
    pub fn set_chain_full_scale(&mut self) -> Result<&mut Self, E> {
        self.set_gyro_scale(FsG::Dps500)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn parse_acceleromtere_2g() {
        use registers::FsXl;

        // Table 19 in AN5398
        assert_eq!(
            AccelValue::from_msr(FsXl::G2, &[0x0, 0x0, 0x0, 0x0, 0x0, 0x0]).as_m_ss(),
            [0., 0., 0.]
        );

        let a = AccelValue::from_msr(FsXl::G2, &[0x69, 0x16, 0x0, 0x0, 0x0, 0x0]).as_m_ss();
        assert_abs_diff_eq!(a[0], 0.350 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);

        let a = AccelValue::from_msr(FsXl::G2, &[0x09, 0x40, 0x0, 0x0, 0x0, 0x0]).as_m_ss();
        assert_abs_diff_eq!(a[0], 1.0 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);

        let a = AccelValue::from_msr(FsXl::G2, &[0x97, 0xe9, 0x0, 0x0, 0x0, 0x0]).as_m_ss();
        assert_abs_diff_eq!(a[0], -0.350 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);

        let a = AccelValue::from_msr(FsXl::G2, &[0xf7, 0xbf, 0x0, 0x0, 0x0, 0x0]).as_m_ss();
        assert_abs_diff_eq!(a[0], -1.0 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);
    }

    #[test]
    fn parse_gyro_250dps() {
        use registers::FsG;

        // Table 19 in AN5398
        assert_eq!(
            GyroValue::from_msr(FsG::Dps250, &[0x0, 0x0, 0x0, 0x0, 0x0, 0x0]).as_rad(),
            [0., 0., 0.]
        );

        let a = GyroValue::from_msr(FsG::Dps250, &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]).as_rad();
        assert_abs_diff_eq!(a[0], 100. * SENSORS_DPS_TO_RADS, epsilon = 0.01);

        let a = GyroValue::from_msr(FsG::Dps250, &[0x49, 0x59, 0x0, 0x0, 0x0, 0x0]).as_rad();
        assert_abs_diff_eq!(a[0], 200. * SENSORS_DPS_TO_RADS, epsilon = 0.01);

        let a = GyroValue::from_msr(FsG::Dps250, &[0x5c, 0xd3, 0x0, 0x0, 0x0, 0x0]).as_rad();
        assert_abs_diff_eq!(a[0], -100. * SENSORS_DPS_TO_RADS, epsilon = 0.01);

        let a = GyroValue::from_msr(FsG::Dps250, &[0xb7, 0xa6, 0x0, 0x0, 0x0, 0x0]).as_rad();
        assert_abs_diff_eq!(a[0], -200. * SENSORS_DPS_TO_RADS, epsilon = 0.01);
    }

    #[test]
    fn test_set_fifo_mode() {
        // Read register 0x0A (FifoCtrl4), modify it, write back.
        let i2c = Mock::new(&[
            Transaction::write_read(DEFAULT_I2C_ADDRESS, vec![0x0A], vec![0b00000000]),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![0x0A, 0b00000001]),
        ]);

        // Pass the mock by value to the sensor
        let mut sensor = Ism330Dhcx::new(i2c).unwrap();
        sensor.set_fifo_mode(FifoMode::FifoMode).unwrap();

        // Destroy sensor to get i2c back and verify expectations
        let mut i2c = sensor.destroy();
        i2c.done();
    }
}
