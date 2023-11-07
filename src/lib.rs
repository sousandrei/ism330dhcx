//! This is a simple driver for ST's `ism330dhcx` sensor.
//!
//! # Quick Start
//! To declare a sensor is pretty simple:
//!
//! ```rust
//! let sensor = Ism330Dhcx::new(&mut i2c).unwrap()
//! ```
//!
//! If you want to use another address for the chip, you can do:
//!
//! ```rust
//! let sensor = Ism330Dhcx::new_with_address(&mut i2c, 0x6au8).unwrap()
//! ```
//!
//! Or alter it after the fact
//!
//! ```rust
//! sensor.set_address(0x6au8);
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

#![cfg_attr(not(test), no_std)]

use core::convert::TryInto;
use embedded_hal::blocking::i2c::{Write, WriteRead};

pub mod ctrl1xl;
pub mod ctrl2g;
pub mod ctrl3c;
pub mod ctrl7g;
pub mod ctrl9xl;
pub mod fifo;
pub mod fifoctrl;
pub mod fifostatus;

use ctrl1xl::Ctrl1Xl;
use ctrl2g::Ctrl2G;
use ctrl3c::Ctrl3C;
use ctrl7g::Ctrl7G;
use ctrl9xl::Ctrl9Xl;
use fifoctrl::FifoCtrl;
use fifostatus::FifoStatus;

/// Datasheet write address for the device. (D6h)
pub const DEFAULT_I2C_ADDRESS: u8 = 0x6bu8;

const SENSORS_DPS_TO_RADS: f64 = 0.017453292;
const SENSORS_GRAVITY_STANDARD: f64 = 9.80665;

trait Register {
    fn read<I2C>(&self, i2c: &mut I2C, chip_addr: u8, reg_addr: u8) -> Result<u8, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut data: [u8; 1] = [0];
        i2c.write_read(chip_addr, &[reg_addr], &mut data)?;
        Ok(data[0])
    }

    fn write<I2C>(
        &self,
        i2c: &mut I2C,
        chip_addr: u8,
        reg_addr: u8,
        bits: u8,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        i2c.write(chip_addr, &[reg_addr, bits])
    }
}

pub struct Ism330Dhcx {
    pub address: u8,
    pub ctrl1xl: Ctrl1Xl,
    pub ctrl2g: Ctrl2G,
    pub ctrl7g: Ctrl7G,
    pub ctrl3c: Ctrl3C,
    pub ctrl9xl: Ctrl9Xl,
    pub fifoctrl: FifoCtrl,
    pub fifostatus: FifoStatus,
}

impl Ism330Dhcx {
    pub fn new<I2C, E>(i2c: &mut I2C) -> Result<Ism330Dhcx, E>
    where
        I2C: WriteRead<Error = E> + Write<Error = E>,
    {
        Ism330Dhcx::new_with_address(i2c, DEFAULT_I2C_ADDRESS)
    }

    pub fn new_with_address<I2C, E>(i2c: &mut I2C, address: u8) -> Result<Ism330Dhcx, E>
    where
        I2C: WriteRead<Error = E> + Write<Error = E>,
    {
        let mut registers = [0u8; 13];
        i2c.write_read(address, &[0x10], &mut registers)?;

        let ctrl1xl = Ctrl1Xl::new(registers[0], address);
        let ctrl2g = Ctrl2G::new(registers[1], address);
        let ctrl3c = Ctrl3C::new(registers[2], address);
        let ctrl7g = Ctrl7G::new(registers[6], address);
        let ctrl9xl = Ctrl9Xl::new(registers[8], address);
        let fifoctrl = FifoCtrl::new(registers[9..13].try_into().unwrap(), address);
        let fifostatus = FifoStatus::new(address);

        let ism330dhcx = Ism330Dhcx {
            address,
            ctrl1xl,
            ctrl2g,
            ctrl3c,
            ctrl7g,
            ctrl9xl,
            fifoctrl,
            fifostatus,
        };

        Ok(ism330dhcx)
    }

    pub fn set_address(&mut self, address: u8) {
        self.ctrl1xl.address = address;
        self.ctrl2g.address = address;
        self.ctrl3c.address = address;
        self.ctrl7g.address = address;
        self.ctrl9xl.address = address;
        self.fifoctrl.address = address;
        self.fifostatus.address = address;
    }

    /// Get temperature in Celsius.
    pub fn get_temperature<I2C>(&mut self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut measurements = [0u8; 2];
        i2c.write_read(self.address, &[0x20], &mut measurements)?;

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
        i2c.write_read(self.address, &[0x22], &mut measurements)?;

        Ok(parse_gyroscope(scale, &measurements))
    }

    pub fn get_accelerometer<I2C>(&mut self, i2c: &mut I2C) -> Result<[f64; 3], I2C::Error>
    where
        I2C: WriteRead,
    {
        let scale = self.ctrl1xl.chain_full_scale();

        let mut measurements = [0u8; 6];
        i2c.write_read(self.address, &[0x28], &mut measurements)?;

        Ok(parse_accelerometer(scale, &measurements))
    }

    pub fn fifo_pop<I2C>(&mut self, i2c: &mut I2C) -> Result<fifo::Value, I2C::Error>
    where
        I2C: WriteRead,
    {
        let gyro_scale = self.ctrl2g.chain_full_scale();
        let accel_scale = self.ctrl1xl.chain_full_scale();

        fifo::FifoOut::new(self.address).pop(i2c, gyro_scale, accel_scale)
    }
}

pub(crate) fn parse_gyroscope(scale: f64, measurements: &[u8; 6]) -> [f64; 3] {
    let raw_gyro_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
    let raw_gyro_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
    let raw_gyro_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);

    // Sensitivity mdps / LSB: G_So in Table 2.
    let sens = match scale {
        125. => 4.375,
        250. => 8.750,
        500. => 17.50,
        1000. => 35.,
        2000. => 70.,
        4000. => 140.,
        _ => unreachable!(),
    };

    let gyro_x = raw_gyro_x as f64 * sens * SENSORS_DPS_TO_RADS / 1000.;
    let gyro_y = raw_gyro_y as f64 * sens * SENSORS_DPS_TO_RADS / 1000.0;
    let gyro_z = raw_gyro_z as f64 * sens * SENSORS_DPS_TO_RADS / 1000.0;

    [gyro_x, gyro_y, gyro_z]
}

pub(crate) fn parse_accelerometer(scale: f64, measurements: &[u8; 6]) -> [f64; 3] {
    let raw_acc_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
    let raw_acc_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
    let raw_acc_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);

    // Sensitivity milli-g / LSB: LA_So in Table 2.
    let sens = match scale {
        2. => 0.061,
        4. => 0.122,
        8. => 0.244,
        16. => 0.488,
        _ => unreachable!(),
    };

    let acc_x = raw_acc_x as f64 * sens * SENSORS_GRAVITY_STANDARD / 1000.0;
    let acc_y = raw_acc_y as f64 * sens * SENSORS_GRAVITY_STANDARD / 1000.0;
    let acc_z = raw_acc_z as f64 * sens * SENSORS_GRAVITY_STANDARD / 1000.0;

    [acc_x, acc_y, acc_z]
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn parse_acceleromtere_2g() {
        // Table 19 in AN5398
        assert_eq!(
            parse_accelerometer(2., &[0x0, 0x0, 0x0, 0x0, 0x0, 0x0]),
            [0., 0., 0.]
        );

        let a = parse_accelerometer(2., &[0x69, 0x16, 0x0, 0x0, 0x0, 0x0]);
        assert_abs_diff_eq!(a[0], 0.350 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);

        let a = parse_accelerometer(2., &[0x09, 0x40, 0x0, 0x0, 0x0, 0x0]);
        assert_abs_diff_eq!(a[0], 1.0 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);

        let a = parse_accelerometer(2., &[0x97, 0xe9, 0x0, 0x0, 0x0, 0x0]);
        assert_abs_diff_eq!(a[0], -0.350 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);

        let a = parse_accelerometer(2., &[0xf7, 0xbf, 0x0, 0x0, 0x0, 0x0]);
        assert_abs_diff_eq!(a[0], -1.0 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);
    }

    #[test]
    fn parse_gyro_250dps() {
        // Table 19 in AN5398
        assert_eq!(
            parse_gyroscope(250., &[0x0, 0x0, 0x0, 0x0, 0x0, 0x0]),
            [0., 0., 0.]
        );

        let a = parse_gyroscope(250., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
        assert_abs_diff_eq!(a[0], 100. * SENSORS_DPS_TO_RADS, epsilon = 0.01);

        let a = parse_gyroscope(250., &[0x49, 0x59, 0x0, 0x0, 0x0, 0x0]);
        assert_abs_diff_eq!(a[0], 200. * SENSORS_DPS_TO_RADS, epsilon = 0.01);

        let a = parse_gyroscope(250., &[0x5c, 0xd3, 0x0, 0x0, 0x0, 0x0]);
        assert_abs_diff_eq!(a[0], -100. * SENSORS_DPS_TO_RADS, epsilon = 0.01);

        let a = parse_gyroscope(250., &[0xb7, 0xa6, 0x0, 0x0, 0x0, 0x0]);
        assert_abs_diff_eq!(a[0], -200. * SENSORS_DPS_TO_RADS, epsilon = 0.01);
    }

    #[test]
    fn parse_ranges_not_unreachable() {
        parse_gyroscope(125., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
        parse_gyroscope(250., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
        parse_gyroscope(500., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
        parse_gyroscope(1000., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
        parse_gyroscope(2000., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
        parse_gyroscope(4000., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);

        parse_accelerometer(2., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
        parse_accelerometer(4., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
        parse_accelerometer(8., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
        parse_accelerometer(16., &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]);
    }
}
