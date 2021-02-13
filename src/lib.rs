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

pub struct ISM330DHCX {
    pub ctrl1xl: Ctrl1Xl,
    pub ctrl2g: Ctrl2G,
    pub ctrl7g: Ctrl7G,
    pub ctrl3c: Ctrl3C,
    pub ctrl9xl: Ctrl9Xl,
}

impl ISM330DHCX {
    pub fn new<I2C, E>(i2c: &mut I2C) -> Result<ISM330DHCX, E>
    where
        I2C: WriteRead<Error = E> + Write<Error = E>,
    {
        let ctrl1xl = Ctrl1Xl::new(i2c)?;
        let ctrl2g = Ctrl2G::new(i2c)?;
        let ctrl3c = Ctrl3C::new(i2c)?;
        let ctrl7g = Ctrl7G::new(i2c)?;
        let ctrl9xl = Ctrl9Xl::new(i2c)?;

        let ism330dhcx = ISM330DHCX {
            ctrl1xl,
            ctrl2g,
            ctrl3c,
            ctrl7g,
            ctrl9xl,
        };

        Ok(ism330dhcx)
    }
}

pub fn write_read<I2C>(i2c: &mut I2C, reg_addr: u8) -> Result<u8, I2C::Error>
where
    I2C: WriteRead,
{
    let mut data: [u8; 1] = [0];
    i2c.write_read(I2C_ADDRESS, &[reg_addr], &mut data)?;
    Ok(data[0])
}

pub fn write<I2C>(i2c: &mut I2C, reg_addr: u8, bits: u8) -> Result<(), I2C::Error>
where
    I2C: Write,
{
    i2c.write(I2C_ADDRESS, &[reg_addr, bits])
}
