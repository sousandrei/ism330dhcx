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

pub mod fifo;
pub mod registers;

use embedded_hal::i2c::I2c;
use registers::*;

/// Datasheet write address for the device. (D6h)
pub const DEFAULT_I2C_ADDRESS: u8 = 0x6bu8;

const SENSORS_DPS_TO_RADS: f64 = 0.017453292;
const SENSORS_GRAVITY_STANDARD: f64 = 9.80665;

#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct GyroValue {
    range: FsG,
    count: [i16; 3],
}

impl GyroValue {
    pub fn new(range: FsG, count: [i16; 3]) -> GyroValue {
        GyroValue { range, count }
    }

    pub fn from_msr(range: FsG, measurements: &[u8; 6]) -> GyroValue {
        let raw_gyro_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_gyro_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_gyro_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);
        GyroValue {
            range,
            count: [raw_gyro_x, raw_gyro_y, raw_gyro_z],
        }
    }

    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// As radians [rad]
    pub fn as_rad(&self) -> [f64; 3] {
        self.as_mdps().map(|v| v * SENSORS_DPS_TO_RADS / 1000.)
    }

    /// As milli degrees per second [mdps]
    pub fn as_mdps(&self) -> [f64; 3] {
        let sensitivity = self.range.sensitivity() as f64;
        self.count.map(|r| r as f64 * sensitivity)
    }

    /// As degrees per second [dps]
    pub fn as_dps(&self) -> [f64; 3] {
        self.as_mdps().map(|v| v / 1000.)
    }
}

#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct AccelValue {
    range: FsXl,
    count: [i16; 3],
}

impl AccelValue {
    pub fn new(range: FsXl, count: [i16; 3]) -> AccelValue {
        AccelValue { range, count }
    }

    pub fn from_msr(range: FsXl, measurements: &[u8; 6]) -> AccelValue {
        let raw_acc_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_acc_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_acc_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);
        AccelValue {
            range,
            count: [raw_acc_x, raw_acc_y, raw_acc_z],
        }
    }

    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// As [m/s^2]
    pub fn as_m_ss(&self) -> [f64; 3] {
        self.as_mg().map(|v| v * SENSORS_GRAVITY_STANDARD / 1000.)
    }

    /// As [milli-g]
    pub fn as_mg(&self) -> [f64; 3] {
        let sensitivity = self.range.sensitivity() as f64;
        self.count.map(|r| r as f64 * sensitivity)
    }

    /// As [g]
    pub fn as_g(&self) -> [f64; 3] {
        self.as_mg().map(|v| v / 1000.)
    }
}

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
        self.i2c.write_read(self.address, &[reg.addr()], &mut buffer)?;
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
            if boot {
                v | CTRL3_C_BOOT
            } else {
                v & !CTRL3_C_BOOT
            }
        })
    }

    pub fn set_bdu(&mut self, bdu: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl3C, |v| {
            if bdu {
                v | CTRL3_C_BDU
            } else {
                v & !CTRL3_C_BDU
            }
        })
    }

    pub fn set_if_inc(&mut self, if_inc: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl3C, |v| {
            if if_inc {
                v | CTRL3_C_IF_INC
            } else {
                v & !CTRL3_C_IF_INC
            }
        })
    }

    // ===========================================
    // Accelerometer
    // ===========================================

    pub fn set_accel_odr(&mut self, odr: OdrXl) -> Result<(), E> {
        self.modify_reg(Register::Ctrl1Xl, |v| {
            (v & !CTRL1_XL_ODR_MASK) | ((odr as u8) << CTRL1_XL_ODR_SHIFT)
        })
    }

    pub fn set_accel_scale(&mut self, scale: FsXl) -> Result<(), E> {
        self.modify_reg(Register::Ctrl1Xl, |v| {
            (v & !CTRL1_XL_FS_MASK) | ((scale as u8) << CTRL1_XL_FS_SHIFT)
        })
    }

    pub fn get_accel_scale(&mut self) -> Result<FsXl, E> {
        let v = self.read_reg(Register::Ctrl1Xl)?;
        let raw = (v & CTRL1_XL_FS_MASK) >> CTRL1_XL_FS_SHIFT;
        Ok(match raw {
            0b00 => FsXl::G2,
            0b01 => FsXl::G16,
            0b10 => FsXl::G4,
            0b11 => FsXl::G8,
            _ => unreachable!(),
        })
    }

    pub fn set_lpf2_xl_en(&mut self, enable: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl1Xl, |v| {
            if enable {
                v | CTRL1_XL_LPF2_XL_EN
            } else {
                v & !CTRL1_XL_LPF2_XL_EN
            }
        })
    }

    // ===========================================
    // Gyroscope
    // ===========================================

    pub fn set_gyro_odr(&mut self, odr: OdrG) -> Result<(), E> {
        self.modify_reg(Register::Ctrl2G, |v| {
            (v & !CTRL2_G_ODR_MASK) | ((odr as u8) << CTRL2_G_ODR_SHIFT)
        })
    }

    pub fn set_gyro_scale(&mut self, scale: FsG) -> Result<(), E> {
        // Special cases for 125 and 4000
        match scale {
            FsG::Dps125 => self.modify_reg(Register::Ctrl2G, |v| (v & !CTRL2_G_FS_MASK) | CTRL2_G_FS_125),
            FsG::Dps4000 => self.modify_reg(Register::Ctrl2G, |v| (v & !CTRL2_G_FS_MASK) | CTRL2_G_FS_4000),
            _ => self.modify_reg(Register::Ctrl2G, |v| {
                let mask = match scale {
                    FsG::Dps250 => 0b00,
                    FsG::Dps500 => 0b01,
                    FsG::Dps1000 => 0b10,
                    FsG::Dps2000 => 0b11,
                    _ => unreachable!(),
                };
                (v & !CTRL2_G_FS_MASK) | (mask << CTRL2_G_FS_SHIFT)
            }),
        }
    }

    pub fn get_gyro_scale(&mut self) -> Result<FsG, E> {
        let v = self.read_reg(Register::Ctrl2G)?;
        // Check special bits first
        if (v & CTRL2_G_FS_4000) != 0 {
            return Ok(FsG::Dps4000);
        }
        if (v & CTRL2_G_FS_125) != 0 {
            return Ok(FsG::Dps125);
        }
        let raw = (v & CTRL2_G_FS_MASK) >> CTRL2_G_FS_SHIFT;
        Ok(match raw {
            0b00 => FsG::Dps250,
            0b01 => FsG::Dps500,
            0b10 => FsG::Dps1000,
            0b11 => FsG::Dps2000,
            _ => unreachable!(),
        })
    }
    
    // ===========================================
    // CTRL7_G
    // ===========================================

    pub fn set_g_hm_mode(&mut self, enable: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl7G, |v| {
            if enable {
                v & !CTRL7_G_G_HM_MODE // Inverted logic: 0 is enable
            } else {
                v | CTRL7_G_G_HM_MODE
            }
        })
    }

    // ===========================================
    // CTRL9_XL
    // ===========================================

    pub fn set_den_x(&mut self, val: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl9Xl, |v| if val { v | CTRL9_XL_DEN_X } else { v & !CTRL9_XL_DEN_X })
    }
    pub fn set_den_y(&mut self, val: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl9Xl, |v| if val { v | CTRL9_XL_DEN_Y } else { v & !CTRL9_XL_DEN_Y })
    }
    pub fn set_den_z(&mut self, val: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl9Xl, |v| if val { v | CTRL9_XL_DEN_Z } else { v & !CTRL9_XL_DEN_Z })
    }
    pub fn set_den_device_conf(&mut self, val: bool) -> Result<(), E> {
        self.modify_reg(Register::Ctrl9Xl, |v| if val { v | CTRL9_XL_DEVICE_CONF } else { v & !CTRL9_XL_DEVICE_CONF })
    }

    // ===========================================
    // FIFO
    // ===========================================

    pub fn set_fifo_mode(&mut self, mode: FifoMode) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl4, |v| {
            (v & !FIFO_CTRL4_FIFO_MODE_MASK) | (mode as u8)
        })
    }

    pub fn set_fifo_accel_batch_rate(&mut self, rate: BdrXl) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl3, |v| {
            (v & !FIFO_CTRL3_BDR_XL_MASK) | (rate as u8)
        })
    }

    pub fn set_fifo_gyro_batch_rate(&mut self, rate: BdrGy) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl3, |v| {
            (v & !FIFO_CTRL3_BDR_GY_MASK) | ((rate as u8) << FIFO_CTRL3_BDR_GY_SHIFT)
        })
    }

    pub fn set_fifo_compression(&mut self, enable: bool) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl2, |v| {
            if enable {
                v | FIFO_CTRL2_FIFO_COMPR_RT_EN
            } else {
                v & !FIFO_CTRL2_FIFO_COMPR_RT_EN
            }
        })
    }

    // ===========================================
    // Sensors
    // ===========================================

    /// Get temperature in Celsius.
    pub fn get_temperature(&mut self) -> Result<f32, E> {
        let mut measurements = [0u8; 2];
        self.i2c.write_read(self.address, &[0x20], &mut measurements)?;

        let raw_temp = (measurements[1] as i16) << 8 | measurements[0] as i16;
        let temp: f32 = (raw_temp as f32 / 256.0) + 25.0;

        Ok(temp)
    }

    pub fn get_gyroscope(&mut self) -> Result<GyroValue, E> {
        let scale = self.get_gyro_scale()?;

        let mut measurements = [0u8; 6];
        self.i2c.write_read(self.address, &[0x22], &mut measurements)?;

        Ok(GyroValue::from_msr(scale, &measurements))
    }

    pub fn get_accelerometer(&mut self) -> Result<AccelValue, E> {
        let scale = self.get_accel_scale()?;

        let mut measurements = [0u8; 6];
        self.i2c.write_read(self.address, &[0x28], &mut measurements)?;

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
