use embedded_hal::blocking::i2c::{Write, WriteRead};

pub struct Sensor<'a, Comm> {
    address: u8,
    comm: &'a mut Comm,
}

impl<'a, Comm> Sensor<'a, Comm> {
    pub fn new(address: u8, comm: &'a mut Comm) -> Sensor<'a, Comm> {
        Sensor { address, comm }
    }

    fn address(&self) -> u8 {
        self.address
    }

    fn comm(&mut self) -> &mut Comm {
        self.comm
    }
}

pub fn write_read<Comm>(sensor: &mut Sensor<Comm>, reg_addr: u8) -> Result<u8, Comm::Error>
where
    Comm: WriteRead,
{
    let mut data: [u8; 1] = [0];
    let sensor_addr = sensor.address();
    sensor
        .comm()
        .write_read(sensor_addr, &[reg_addr], &mut data)?;
    Ok(data[0])
}

pub fn write<Comm>(sensor: &mut Sensor<Comm>, reg_addr: u8, bits: u8) -> Result<(), Comm::Error>
where
    Comm: Write,
{
    let sensor_addr = sensor.address();
    sensor.comm().write(sensor_addr, &[reg_addr, bits])
}
