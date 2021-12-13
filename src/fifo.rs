use embedded_hal::blocking::i2c::WriteRead;
use core::convert::{TryFrom, TryInto};

use crate::{parse_accelerometer, parse_gyroscope, Register};

#[repr(u8)]
pub enum SensorTag {
    GyroscopeNC = 0x01,
    AccelerometerNC = 0x02,
}

impl TryFrom<u8> for SensorTag {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == SensorTag::GyroscopeNC as u8 => Ok(SensorTag::GyroscopeNC),
            x if x == SensorTag::AccelerometerNC as u8 => Ok(SensorTag::AccelerometerNC),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Gyroscope([f64; 3]),
    Accelerometer([f64; 3]),
}

const ADDR: u8 = 0x78;

pub struct FifoOut;

impl Register for FifoOut {}

impl FifoOut {
    pub fn new() -> Self {
        FifoOut
    }

    /// Pop a value from the FIFO.
    pub fn pop<I2C>(
        &mut self,
        i2c: &mut I2C,
        gyro_scale: f64,
        accel_scale: f64,
    ) -> Result<Value, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut out = [0u8; 7];
        i2c.write_read(crate::I2C_ADDRESS, &[ADDR], &mut out)?;

        let tag = out[0] >> 3;

        match tag.try_into() {
            Ok(SensorTag::GyroscopeNC) => Ok(Value::Gyroscope(parse_gyroscope(
                gyro_scale,
                out[1..].try_into().unwrap(),
            ))),
            Ok(SensorTag::AccelerometerNC) => Ok(Value::Accelerometer(parse_accelerometer(
                accel_scale,
                out[1..].try_into().unwrap(),
            ))),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::i2c::{Mock, Transaction};

    #[test]
    fn test_pop_gyro() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            0x6a,
            vec![0x78],
            vec![0x01 << 3, 0, 1, 0, 2, 0, 4],
        )]);

        let mut f = FifoOut;
        let v = f.pop(&mut i2c, 1., 1.).unwrap();

        assert!(matches!(v, Value::Gyroscope(_)));
        println!("{:?}", v);
    }
}
