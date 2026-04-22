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

pub mod registers;

pub use registers::{
    AccelValue, Accelerometer, FifoOut, GyroValue, Gyroscope, SENSORS_DPS_TO_RADS,
    SENSORS_GRAVITY_STANDARD, TagSensor, Temperature, Value,
    all_int_src::AllIntSrcConfig,
    counter_bdr::{CounterBdrReg1Config, CounterBdrReg2Config},
    ctrl1_xl::Ctrl1XlConfig,
    ctrl2_g::Ctrl2GConfig,
    ctrl3_c::Ctrl3CConfig,
    ctrl4_c::Ctrl4CConfig,
    ctrl5_c::Ctrl5CConfig,
    ctrl6_c::Ctrl6CConfig,
    ctrl7_g::Ctrl7GConfig,
    ctrl8_xl::Ctrl8XlConfig,
    ctrl9_xl::Ctrl9XlConfig,
    ctrl10_c::Ctrl10CConfig,
    fifo::{Fifo, FifoCtrl2Config, FifoCtrl3Config, FifoCtrl4Config},
    free_fall::FreeFallConfig,
    func_cfg_access::FuncCfgAccessConfig,
    int_ctrl::{Int1CtrlConfig, Int2CtrlConfig},
    int_dur2::IntDur2Config,
    internal_freq_fine::InternalFreqFineConfig,
    md_cfg::{Md1CfgConfig, Md2CfgConfig},
    ofs_usr::OfsUsrConfig,
    pin_ctrl::PinCtrlConfig,
    status_reg::StatusRegConfig,
    tap_cfg::{TapCfg0Config, TapCfg1Config, TapCfg2Config},
    tap_ths_6d::TapThs6dConfig,
    timestamp::TimestampConfig,
    wake_up_dur::WakeUpDurConfig,
    wake_up_src::WakeUpSrcConfig,
    wake_up_ths::WakeUpThsConfig,
};

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::Register;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

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
