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

pub use accelerometer::{AccelValue, SENSORS_GRAVITY_STANDARD};
pub use gyroscope::{GyroValue, SENSORS_DPS_TO_RADS};

use embedded_hal::i2c::I2c;
use registers::*;

/// Datasheet write address for the device. (D6h)
pub const DEFAULT_I2C_ADDRESS: u8 = 0x6bu8;

// SENSORS_DPS_TO_RADS moved to gyroscope.rs
// SENSORS_GRAVITY_STANDARD moved to accelerometer.rs

pub struct Ism330Dhcx<I2C> {
    pub address: u8,
    i2c: I2C,
}

impl<I2C, E> Ism330Dhcx<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Result<Self, E> {
        Self::new_with_address(i2c, DEFAULT_I2C_ADDRESS)
    }

    pub fn new_with_address(i2c: I2C, address: u8) -> Result<Self, E> {
        Ok(Self { address, i2c })
    }

    /// Return the underlying I2C interface
    pub fn destroy(self) -> I2C {
        self.i2c
    }

    fn read_reg(&mut self, reg: Register) -> Result<u8, E> {
        let mut buffer = [0u8];
        self.i2c
            .write_read(self.address, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }

    fn write_reg(&mut self, reg: Register, value: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[reg.addr(), value])
    }

    fn modify_reg<F>(&mut self, reg: Register, f: F) -> Result<(), E>
    where
        F: FnOnce(u8) -> u8,
    {
        let value = self.read_reg(reg)?;
        let new_value = f(value);
        self.write_reg(reg, new_value)
    }

    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }

    // ===========================================
    // Configuration
    // ===========================================

    pub fn set_boot(&mut self, boot: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_boot(boot);
            reg.into_bytes()[0]
        })
    }

    pub fn set_bdu(&mut self, bdu: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_bdu(bdu);
            reg.into_bytes()[0]
        })
    }

    pub fn set_if_inc(&mut self, if_inc: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_if_inc(if_inc);
            reg.into_bytes()[0]
        })
    }

    // ===========================================
    // Accelerometer
    // ===========================================

    pub fn set_accel_odr(&mut self, odr: OdrXl) -> Result<(), E> {
        self.modify_reg(Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_odr_xl(odr);
            reg.into_bytes()[0]
        })
    }

    pub fn set_accel_scale(&mut self, scale: FsXl) -> Result<(), E> {
        self.modify_reg(Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_fs_xl(scale);
            reg.into_bytes()[0]
        })
    }

    pub fn get_accel_scale(&mut self) -> Result<FsXl, E> {
        let v = self.read_reg(Register::Ctrl1Xl)?;
        let reg = Ctrl1Xl::from_bytes([v]);
        Ok(reg.fs_xl())
    }

    pub fn set_lpf2_xl_en(&mut self, enable: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_lpf2_xl_en(enable);
            reg.into_bytes()[0]
        })
    }

    // ===========================================
    // Gyroscope
    // ===========================================

    pub fn set_gyro_odr(&mut self, odr: OdrG) -> Result<(), E> {
        self.modify_reg(Register::Ctrl2G, |v| {
            let mut reg = Ctrl2G::from_bytes([v]);
            reg.set_odr_g(odr);
            reg.into_bytes()[0]
        })
    }

    pub fn set_gyro_scale(&mut self, scale: FsG) -> Result<(), E> {
        self.modify_reg(Register::Ctrl2G, |v| {
            let mut reg = Ctrl2G::from_bytes([v]);

            // Reset fields
            reg.set_fs_125(false);
            reg.set_fs_4000(false);

            match scale {
                FsG::Dps125 => reg.set_fs_125(true),
                FsG::Dps4000 => reg.set_fs_4000(true),
                FsG::Dps250 => reg.set_fs_g(FsGScale::Dps250),
                FsG::Dps500 => reg.set_fs_g(FsGScale::Dps500),
                FsG::Dps1000 => reg.set_fs_g(FsGScale::Dps1000),
                FsG::Dps2000 => reg.set_fs_g(FsGScale::Dps2000),
            }
            reg.into_bytes()[0]
        })
    }

    pub fn get_gyro_scale(&mut self) -> Result<FsG, E> {
        let v = self.read_reg(Register::Ctrl2G)?;
        let reg = Ctrl2G::from_bytes([v]);

        if reg.fs_4000() {
            return Ok(FsG::Dps4000);
        }
        if reg.fs_125() {
            return Ok(FsG::Dps125);
        }
        Ok(match reg.fs_g() {
            FsGScale::Dps250 => FsG::Dps250,
            FsGScale::Dps500 => FsG::Dps500,
            FsGScale::Dps1000 => FsG::Dps1000,
            FsGScale::Dps2000 => FsG::Dps2000,
        })
    }

    // ===========================================
    // CTRL7_G
    // ===========================================

    pub fn set_g_hm_mode(&mut self, enable: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl7G, |v| {
            let mut reg = Ctrl7G::from_bytes([v]);
            reg.set_g_hm_mode(enable);
            reg.into_bytes()[0]
        })
    }

    // ===========================================
    // CTRL9_XL
    // ===========================================

    pub fn set_den_x(&mut self, val: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_den_x(val);
            reg.into_bytes()[0]
        })
    }
    pub fn set_den_y(&mut self, val: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_den_y(val);
            reg.into_bytes()[0]
        })
    }
    pub fn set_den_z(&mut self, val: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_den_z(val);
            reg.into_bytes()[0]
        })
    }
    pub fn set_den_device_conf(&mut self, val: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_device_conf(val);
            reg.into_bytes()[0]
        })
    }

    // ===========================================
    // FIFO
    // ===========================================

    pub fn set_fifo_mode(&mut self, mode: FifoMode) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl4, |v| {
            let mut reg = FifoCtrl4::from_bytes([v]);
            reg.set_fifo_mode(mode);
            reg.into_bytes()[0]
        })
    }

    pub fn set_fifo_accel_batch_rate(&mut self, rate: BdrXl) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl3, |v| {
            let mut reg = FifoCtrl3::from_bytes([v]);
            reg.set_bdr_xl(rate);
            reg.into_bytes()[0]
        })
    }

    pub fn set_fifo_gyro_batch_rate(&mut self, rate: BdrGy) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl3, |v| {
            let mut reg = FifoCtrl3::from_bytes([v]);
            reg.set_bdr_gy(rate);
            reg.into_bytes()[0]
        })
    }

    pub fn set_fifo_compression(&mut self, enable: bool) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_fifo_compr_rt_en(enable);
            reg.into_bytes()[0]
        })
    }

    pub fn get_fifo_status(&mut self) -> Result<FifoStatus, E> {
        let mut out = [0u8; 2];
        self.i2c
            .write_read(self.address, &[Register::FifoStatus1.addr()], &mut out)?;
        Ok(FifoStatus::from_bytes(out))
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

    pub fn get_gyroscope(&mut self) -> Result<GyroValue, E> {
        let scale = self.get_gyro_scale()?;

        let mut measurements = [0u8; 6];
        self.i2c
            .write_read(self.address, &[0x22], &mut measurements)?;

        Ok(GyroValue::from_msr(scale, &measurements))
    }

    pub fn get_accelerometer(&mut self) -> Result<AccelValue, E> {
        let scale = self.get_accel_scale()?;

        let mut measurements = [0u8; 6];
        self.i2c
            .write_read(self.address, &[0x28], &mut measurements)?;

        Ok(AccelValue::from_msr(scale, &measurements))
    }

    pub fn fifo_pop(&mut self) -> Result<fifo::Value, E> {
        let gyro_scale = self.get_gyro_scale()?;
        let accel_scale = self.get_accel_scale()?;

        fifo::FifoOut::new(self.address).pop(&mut self.i2c, gyro_scale, accel_scale)
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
