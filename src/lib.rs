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

// SENSORS_GRAVITY_STANDARD moved to accelerometer.rs

/// Errors for the ISM330DHCX driver.
#[derive(Debug, Copy, Clone, defmt::Format)]
pub enum Error<E> {
    /// I2C bus error.
    I2c(E),
    /// Invalid device found (WHO_AM_I mismatch).
    InvalidDevice(u8),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Self::I2c(error)
    }
}

/// Driver for the ISM330DHCX sensor.
pub struct Ism330Dhcx {
    /// I2C address.
    pub address: u8,
}

impl Ism330Dhcx {
    /// Create a new driver instance with the default I2C address (0x6B).
    pub fn new<I2C>(i2c: &mut I2C) -> Result<Self, Error<I2C::Error>>
    where
        I2C: I2c,
    {
        Self::new_with_address(i2c, DEFAULT_I2C_ADDRESS)
    }

    /// Create a new driver instance with a specific I2C address.
    pub fn new_with_address<I2C>(i2c: &mut I2C, address: u8) -> Result<Self, Error<I2C::Error>>
    where
        I2C: I2c,
    {
        let mut buffer = [0u8];
        i2c.write_read(address, &[Register::WhoAmI.addr()], &mut buffer)?;

        if buffer[0] != 0x6b {
            return Err(Error::InvalidDevice(buffer[0]));
        }

        let sensor = Self { address };

        // Set sane defaults: BDU and IF_INC
        sensor.set_bdu(i2c, true)?;
        sensor.set_if_inc(i2c, true)?;

        Ok(sensor)
    }

    pub(crate) fn read_reg<I2C>(&self, i2c: &mut I2C, reg: Register) -> Result<u8, I2C::Error>
    where
        I2C: I2c,
    {
        let mut buffer = [0u8];
        i2c.write_read(self.address, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }

    pub(crate) fn write_reg<I2C>(
        &self,
        i2c: &mut I2C,
        reg: Register,
        value: u8,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        i2c.write(self.address, &[reg.addr(), value])
    }

    pub(crate) fn modify_reg<I2C, F>(
        &self,
        i2c: &mut I2C,
        reg: Register,
        f: F,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
        F: FnOnce(u8) -> u8,
    {
        let value = self.read_reg(i2c, reg)?;
        let new_value = f(value);
        self.write_reg(i2c, reg, new_value)
    }

    /// Set the I2C address.
    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }

    // ===========================================
    // Configuration
    // ===========================================

    /// Reboot memory content.
    pub fn set_boot<I2C>(&mut self, i2c: &mut I2C, boot: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_boot(boot);
            reg.into_bytes()[0]
        })
    }

    /// Block Data Update.
    ///
    /// If true, output registers are not updated until MSB and LSB have been read.
    pub fn set_bdu<I2C>(&self, i2c: &mut I2C, bdu: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_bdu(bdu);
            reg.into_bytes()[0]
        })
    }

    /// Register address automatically incremented during a multiple byte access with a serial interface.
    pub fn set_if_inc<I2C>(&self, i2c: &mut I2C, if_inc: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_if_inc(if_inc);
            reg.into_bytes()[0]
        })
    }

    // ===========================================
    // Sensors
    // ===========================================

    /// Get temperature in Celsius.
    pub fn get_temperature<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: I2c,
    {
        let mut measurements = [0u8; 2];
        i2c.write_read(self.address, &[0x20], &mut measurements)?;

        let raw_temp = (measurements[1] as i16) << 8 | measurements[0] as i16;
        let temp: f32 = (raw_temp as f32 / 256.0) + 25.0;

        Ok(temp)
    }

    /// Set chain full scale.
    pub fn set_chain_full_scale<I2C>(
        &mut self,
        i2c: &mut I2C,
        scale: FsG,
    ) -> Result<&mut Self, I2C::Error>
    where
        I2C: I2c,
    {
        self.set_gyro_scale(i2c, scale)?;
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
        // Read WhoAmI, then BDU and IF_INC modification (read Ctrl3C, write Ctrl3C)
        // Note: set_bdu(true) and set_if_inc(true) will each read and write Ctrl3C.
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::WhoAmI.addr()],
                vec![0x6b],
            ),
            // set_bdu(true)
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr()],
                vec![0b00000000],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr(), 0b01000000],
            ),
            // set_if_inc(true)
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr()],
                vec![0b01000000],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr(), 0b01000100],
            ),
            // set_fifo_mode
            Transaction::write_read(DEFAULT_I2C_ADDRESS, vec![0x0A], vec![0b00000000]),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![0x0A, 0b00000001]),
        ]);

        // Pass the mock by reference to the sensor
        let mut sensor = Ism330Dhcx::new(&mut i2c).unwrap();
        sensor.set_fifo_mode(&mut i2c, FifoMode::FifoMode).unwrap();

        // Verify expectations
        i2c.done();
    }

    #[test]
    fn test_new_invalid_device() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::WhoAmI.addr()],
            vec![0xFF],
        )]);

        let sensor = Ism330Dhcx::new(&mut i2c);
        let err = sensor.err().unwrap();
        assert!(matches!(err, Error::InvalidDevice(0xFF)));
        i2c.done();
    }
}
