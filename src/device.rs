use embedded_hal::blocking::i2c::{Write, WriteRead};

pub struct Device<'a, I2C> {
    address: u8,
    i2c: &'a mut I2C,
}

impl<'a, I2C> Device<'a, I2C> {
    pub fn new(address: u8, i2c: &'a mut I2C) -> Device<'a, I2C> {
        Device { address, i2c: i2c }
    }

    fn address(&self) -> u8 {
        self.address
    }

    fn i2c(&mut self) -> &mut I2C {
        self.i2c
    }
}

pub fn write_read<I2C>(device: &mut Device<I2C>, reg_addr: u8) -> Result<u8, I2C::Error>
where
    I2C: WriteRead,
{
    let mut data: [u8; 1] = [0];
    let device_addr = device.address();
    device
        .i2c()
        .write_read(device_addr, &[reg_addr], &mut data)?;
    Ok(data[0])
}

pub fn write<I2C>(device: &mut Device<I2C>, reg_addr: u8, bits: u8) -> Result<(), I2C::Error>
where
    I2C: Write,
{
    let device_addr = device.address();
    device.i2c().write(device_addr, &[reg_addr, bits])
}
