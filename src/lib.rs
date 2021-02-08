#![no_std]

use embedded_hal::blocking::i2c::{Write, WriteRead};

pub mod ctrl2g;
pub mod ctrl3c;
pub mod sensor;

use ctrl2g::{Ctrl2G, ODR};
use ctrl3c::Ctrl3C;
use sensor::Sensor;

/// Datasheed write address for the sensor. (D6h)
pub const I2C_ADDRESS: u8 = 0x6bu8;

pub struct ISM330DHCX<'a, Comm> {
    pub sensor: Sensor<'a, Comm>,
    pub ctrl2g: Ctrl2G,
    pub ctrl3c: Ctrl3C,
}

impl<'a, Comm, E> ISM330DHCX<'a, Comm>
where
    Comm: WriteRead<Error = E> + Write<Error = E>,
{
    pub fn new(address: u8, comm: &'a mut Comm) -> Result<ISM330DHCX<'a, Comm>, E> {
        let mut sensor = Sensor::new(address, comm);

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

    pub fn set_gyroscope_data_rate(&mut self, samples: ODR) -> Result<(), E> {
        self.ctrl2g.set_gyroscope_data_rate(samples);
        self.ctrl2g.modify(&mut self.sensor)?;
        Ok(())
    }
}
