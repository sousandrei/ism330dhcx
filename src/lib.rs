#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};

pub mod device;
use device::Device;

pub mod ctrl2g;
pub mod ctrl3c;
pub mod ctrl9xl;

use ctrl2g::Ctrl2G;
use ctrl3c::Ctrl3C;
use device::Device;

/// Datasheed write address for the device. (D6h)
pub const I2C_ADDRESS: u8 = 0x6bu8;

pub struct ISM330DHCX<'a, I2c> {
    pub device: Device<'a, I2c>,
    pub ctrl2g: Ctrl2G,
    pub ctrl3c: Ctrl3C,
    pub ctrl9xl: Ctrl9Xl,
}

impl<'a, I2c, E> ISM330DHCX<'a, I2c>
where
    I2c: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(address: u8, i2c: &'a mut I2c) -> Result<ISM330DHCX<'a, I2c>, E> {
        let mut device = Device::new(address, i2c);

        let ctrl3c = Ctrl3C::new(&mut device)?;
        let ctrl2g = Ctrl2G::new(&mut device)?;
        let ctrl3c = Ctrl3C::new(&mut sensor)?;
        let ctrl2g = Ctrl2G::new(&mut sensor)?;

        let ism330dhcx = ISM330DHCX {
            device,
            ctrl2g,
            ctrl3c,
            ctrl2g,
        };

        Ok(ism330dhcx)
    }

    pub fn power_down(&mut self) -> Result<(), E> {
        self.ctrl3c.power_down();
        self.ctrl3c.modify(&mut self.device)?;
        Ok(())
    }

    pub fn set_gyroscope_data_rate(&mut self, samples: ctrl2g::ODR) -> Result<(), E> {
        self.ctrl2g.set_gyroscope_data_rate(samples);
        self.ctrl2g.modify(&mut self.device)?;
        Ok(())
    }
}
