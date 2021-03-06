//! This is a simple driver for ST's `ism330dhcx` sensor.
//!
//! # Quick Start
//! To declare a sensor is pretty simple:
//!
//! ```rust
//! let sensor = Ism330Dhcx::new(&mut i2c).unwrap()
//! ```
//!
//! All registers have the bits addressed by their function, for example here se set the `BOOT` register in the `CTRL_3C` register to `1`
//!
//! ```rust
//! sensor.ctrl3c.set_boot(i2c, true).unwrap();
//! ```
//!
//! For bits that operate together, they have their custom type abstracted. For example, to set the accelerometer data rate you have to operate 4 bits. But here you just have to specify your desired data rate and the driver takes care of it.
//!
//! ```rust
//! // Sets the following bits
//! // ODR_XL3 to 0
//! // ODR_XL2 to 0
//! // ODR_XL1 to 1
//! // ODR_XL0 to 1
//!
//! sensor
//!     .ctrl1xl
//!     .set_accelerometer_data_rate(i2c, ctrl1xl::Odr_Xl::Hz52)
//!     .unwrap();
//! ```

//!
//! # Reference
//!
//!- [Sensor page](https://www.st.com/en/mems-and-sensors/ism330dhcx.html)
//!- [Datasheet](https://www.st.com/resource/en/datasheet/ism330dhcx.pdf)

#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};

pub mod ctrl1xl;
pub mod ctrl2g;
pub mod ctrl3c;
pub mod ctrl7g;
pub mod ctrl9xl;

use ctrl1xl::Ctrl1Xl;
use ctrl2g::Ctrl2G;
use ctrl3c::Ctrl3C;
use ctrl7g::Ctrl7G;
use ctrl9xl::Ctrl9Xl;

/// Datasheed write address for the device. (D6h)
pub const I2C_ADDRESS: u8 = 0x6bu8;

const SENSORS_DPS_TO_RADS: f64 = 0.017453292;
const SENSORS_GRAVITY_STANDARD: f64 = 9.80665;

trait Register {
    fn read<I2C>(&self, i2c: &mut I2C, reg_addr: u8) -> Result<u8, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut data: [u8; 1] = [0];
        i2c.write_read(I2C_ADDRESS, &[reg_addr], &mut data)?;
        Ok(data[0])
    }

    fn write<I2C>(&self, i2c: &mut I2C, reg_addr: u8, bits: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        i2c.write(I2C_ADDRESS, &[reg_addr, bits])
    }
}
pub struct Ism330Dhcx {
    pub ctrl1xl: Ctrl1Xl,
    pub ctrl2g: Ctrl2G,
    pub ctrl7g: Ctrl7G,
    pub ctrl3c: Ctrl3C,
    pub ctrl9xl: Ctrl9Xl,
}

impl Ism330Dhcx {
    pub fn new<I2C, E>(i2c: &mut I2C) -> Result<Ism330Dhcx, E>
    where
        I2C: WriteRead<Error = E> + Write<Error = E>,
    {
        let mut registers = [0u8; 9];
        i2c.write_read(I2C_ADDRESS, &[0x10], &mut registers)?;

        let ctrl1xl = Ctrl1Xl::new(registers[0]);
        let ctrl2g = Ctrl2G::new(registers[1]);
        let ctrl3c = Ctrl3C::new(registers[2]);
        let ctrl7g = Ctrl7G::new(registers[6]);
        let ctrl9xl = Ctrl9Xl::new(registers[8]);

        let ism330dhcx = Ism330Dhcx {
            ctrl1xl,
            ctrl2g,
            ctrl3c,
            ctrl7g,
            ctrl9xl,
        };

        Ok(ism330dhcx)
    }

    pub fn get_temperature<I2C>(&mut self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut measurements = [0u8; 2];
        i2c.write_read(I2C_ADDRESS, &[0x20], &mut measurements)?;

        let raw_temp = (measurements[1] as i16) << 8 | measurements[0] as i16;
        let temp: f32 = (raw_temp as f32 / 256.0) + 25.0;

        Ok(temp)
    }

    pub fn get_gyroscope<I2C>(&mut self, i2c: &mut I2C) -> Result<[f64; 3], I2C::Error>
    where
        I2C: WriteRead,
    {
        let scale = self.ctrl2g.chain_full_scale();

        let mut measurements = [0u8; 6];
        i2c.write_read(I2C_ADDRESS, &[0x22], &mut measurements)?;

        let raw_gyro_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_gyro_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_gyro_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);

        let gyro_x = raw_gyro_x as f64 * scale * SENSORS_DPS_TO_RADS / 1000.0;
        let gyro_y = raw_gyro_y as f64 * scale * SENSORS_DPS_TO_RADS / 1000.0;
        let gyro_z = raw_gyro_z as f64 * scale * SENSORS_DPS_TO_RADS / 1000.0;

        Ok([gyro_x, gyro_y, gyro_z])
    }

    pub fn get_accelerometer<I2C>(&mut self, i2c: &mut I2C) -> Result<[f64; 3], I2C::Error>
    where
        I2C: WriteRead,
    {
        let scale = self.ctrl1xl.chain_full_scale();

        let mut measurements = [0u8; 6];
        i2c.write_read(I2C_ADDRESS, &[0x28], &mut measurements)?;

        let raw_acc_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_acc_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_acc_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);

        let acc_x = raw_acc_x as f64 * scale * SENSORS_GRAVITY_STANDARD / 1000.0;
        let acc_y = raw_acc_y as f64 * scale * SENSORS_GRAVITY_STANDARD / 1000.0;
        let acc_z = raw_acc_z as f64 * scale * SENSORS_GRAVITY_STANDARD / 1000.0;

        Ok([acc_x, acc_y, acc_z])
    }
}
