use core::fmt;

use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::{read, write};

/// The CTRL9_XL (control 9) register
// #[derive(Debug)]
pub struct Ctrl9Xl(u8);

impl fmt::Display for Ctrl9Xl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Binary for Ctrl9Xl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::LowerHex for Ctrl9Xl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

pub const ADDR: u8 = 0x18u8;

/// Enables the proper device configuration.
///
/// Default value: 0
///
/// (0: default; 1: enabled)
///
/// It is recommended to always set this bit to 1 during device configuration.
pub const DEVICE_CONF: u8 = 1;

/// DEN active level configuration.
///
/// Default value: 0
///
/// (0: active low; 1: active high)
pub const DEN_LH: u8 = 2;

/// Extends DEN functionality to accelerometer sensor.
///
/// Default value: 0
///
/// (0: disabled; 1: enabled)
pub const DEN_EN: u8 = 3;

/// DEN stamping sensor selection.
///
/// Default value: 0
///
/// (0: DEN pin info stamped in the gyroscope axis selected by bits \[7:5\]; 1: DEN pin info stamped in the accelerometer axis selected by bits \[7:5\])
pub const DEN_XL_G: u8 = 4;

/// DEN value stored in LSB of X-axis.
///
/// Default value: 1
///
///(0: DEN not stored in X-axis LSB; 1: DEN stored in X-axis LSB)
pub const DEN_X: u8 = 5;

/// DEN value stored in LSB of Y-axis.
///
/// Default value: 1
///
///(0: DEN not stored in Y-axis LSB; 1: DEN stored in Y-axis LSB)
pub const DEN_Y: u8 = 6;

/// DEN value stored in LSB of Z-axis.
///
/// Default value: 1
///
///(0: DEN not stored in Z-axis LSB; 1: DEN stored in Z-axis LSB)
pub const DEN_Z: u8 = 7;

impl Ctrl9Xl {
    pub fn new<I2C>(i2c: &mut I2C) -> Result<Self, I2C::Error>
    where
        I2C: WriteRead,
    {
        let bits = read(i2c, ADDR)?;
        let register = Ctrl9Xl(bits);

        Ok(register)
    }

    pub fn set_device_conf<I2C>(&mut self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= value << DEVICE_CONF;
        write(i2c, ADDR, self.0)
    }

    pub fn set_den_x<I2C>(&mut self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= value << DEN_X;
        write(i2c, ADDR, self.0)
    }

    pub fn set_den_y<I2C>(&mut self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= value << DEN_Y;
        write(i2c, ADDR, self.0)
    }

    pub fn set_den_z<I2C>(&mut self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= value << DEN_Z;
        write(i2c, ADDR, self.0)
    }
}
