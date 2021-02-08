#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};

pub mod ctrl2g;
pub mod ctrl3c;
pub mod device;

use ctrl2g::Ctrl2G;
use ctrl3c::Ctrl3C;
use device::Device;

/// Datasheed write address for the sensor. (D6h)
pub const I2C_ADDRESS: u8 = 0x6bu8;

pub struct ISM330DHCX<'a, I2c> {
    pub sensor: Device<'a, I2c>,
    pub ctrl2g: Ctrl2G,
    pub ctrl3c: Ctrl3C,
}

impl<'a, I2c, E> ISM330DHCX<'a, I2c>
where
    I2c: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(address: u8, i2c: &'a mut I2c) -> Result<ISM330DHCX<'a, I2c>, E> {
        let mut sensor = Device::new(address, i2c);

        let ctrl3c = Ctrl3C::new(&mut sensor)?;
        let ctrl2g = Ctrl2G::new(&mut sensor)?;

        let ism330dhcx = ISM330DHCX {
            sensor,
            ctrl3c,
            ctrl2g,
        };

        Ok(ism330dhcx)
    }

    pub fn power_down(&mut self) -> Result<(), E> {
        self.ctrl3c.power_down();
        self.ctrl3c.modify(&mut self.sensor)?;
        Ok(())
    }

    pub fn set_gyroscope_data_rate(&mut self, samples: ctrl2g::ODR) -> Result<(), E> {
        self.ctrl2g.set_gyroscope_data_rate(samples);
        self.ctrl2g.modify(&mut self.sensor)?;
        Ok(())
    }
}
