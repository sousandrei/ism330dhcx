use core::convert::{TryFrom, TryInto};
use embedded_hal::i2c::I2c;

use crate::registers::{FsG, FsXl};
use crate::{AccelValue, GyroValue};

/// Sensor tag identifying the data source in the FIFO.
#[repr(u8)]
pub enum SensorTag {
    /// Empty tag.
    Empty,
    /// Gyroscope data (No Compression).
    GyroscopeNC,
    /// Accelerometer data (No Compression).
    AccelerometerNC,
    /// Other tag values.
    Other(u8),
}

impl TryFrom<u8> for SensorTag {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(SensorTag::Empty),
            0x01 => Ok(SensorTag::GyroscopeNC),
            0x02 => Ok(SensorTag::AccelerometerNC),
            x if x <= 0x19 => Ok(SensorTag::Other(x)),
            _ => Err(()),
        }
    }
}

/// Parsed FIFO value.
#[derive(Copy, Clone, Debug, defmt::Format)]
pub enum Value {
    /// Empty value.
    Empty,
    /// Gyroscope reading.
    Gyro(GyroValue),
    /// Accelerometer reading.
    Accel(AccelValue),
    /// Other unparsed data.
    Other(u8, [u8; 6]),
}

const ADDR: u8 = 0x78;

/// FIFO output handler.
pub struct FifoOut {
    pub address: u8,
}

impl FifoOut {
    /// Create a new FIFO output handler.
    pub fn new(address: u8) -> Self {
        FifoOut { address }
    }

    /// Pop a value from the FIFO.
    ///
    /// Reads 7 bytes from the FIFO output register.
    /// The first 5 bits of the first byte are the tag.
    /// The rest is the data.
    pub fn pop<I2C, E>(
        &mut self,
        i2c: &mut I2C,
        gyro_scale: FsG,
        accel_scale: FsXl,
    ) -> Result<Value, E>
    where
        I2C: I2c<Error = E>,
    {
        let mut out = [0u8; 7];
        i2c.write_read(self.address, &[ADDR], &mut out)?;

        let (tag, out) = out.split_at(1);
        let tag = tag[0] >> 3;
        let out: &[u8; 6] = out.try_into().expect("must be 6!");

        match tag.try_into() {
            Ok(SensorTag::Empty) => Ok(Value::Empty),
            Ok(SensorTag::GyroscopeNC) => Ok(Value::Gyro(GyroValue::from_msr(gyro_scale, out))),
            Ok(SensorTag::AccelerometerNC) => {
                Ok(Value::Accel(AccelValue::from_msr(accel_scale, out)))
            }
            Ok(SensorTag::Other(u)) => Ok(Value::Other(u, *out)),
            _ => unreachable!(),
        }
    }
}

use crate::Ism330Dhcx;
use crate::registers::{
    BdrGy, BdrXl, FifoCtrl2, FifoCtrl3, FifoCtrl4, FifoMode, FifoStatus, Register,
};

/// FIFO methods.
pub trait Fifo<I2C, E>
where
    I2C: I2c<Error = E>,
{
    /// Set FIFO mode.
    fn set_fifo_mode(&mut self, mode: FifoMode) -> Result<(), E>;
    /// Set accelerometer batch data rate.
    fn set_fifo_accel_batch_rate(&mut self, rate: BdrXl) -> Result<(), E>;
    /// Set gyroscope batch data rate.
    fn set_fifo_gyro_batch_rate(&mut self, rate: BdrGy) -> Result<(), E>;
    /// Enable FIFO compression.
    fn set_fifo_compression(&mut self, enable: bool) -> Result<(), E>;
    /// Get FIFO status.
    fn get_fifo_status(&mut self) -> Result<FifoStatus, E>;
    /// Pop a value from the FIFO.
    fn fifo_pop(&mut self) -> Result<Value, E>;
}

impl<I2C, E> Fifo<I2C, E> for Ism330Dhcx<I2C>
where
    I2C: I2c<Error = E>,
{
    fn set_fifo_mode(&mut self, mode: FifoMode) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl4, |v| {
            let mut reg = FifoCtrl4::from_bytes([v]);
            reg.set_fifo_mode(mode);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_accel_batch_rate(&mut self, rate: BdrXl) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl3, |v| {
            let mut reg = FifoCtrl3::from_bytes([v]);
            reg.set_bdr_xl(rate);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_gyro_batch_rate(&mut self, rate: BdrGy) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl3, |v| {
            let mut reg = FifoCtrl3::from_bytes([v]);
            reg.set_bdr_gy(rate);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_compression(&mut self, enable: bool) -> Result<(), E> {
        self.modify_reg(Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_fifo_compr_rt_en(enable);
            reg.into_bytes()[0]
        })
    }

    fn get_fifo_status(&mut self) -> Result<FifoStatus, E> {
        let mut out = [0u8; 2];
        self.i2c
            .write_read(self.address, &[Register::FifoStatus1.addr()], &mut out)?;
        Ok(FifoStatus::from_bytes(out))
    }

    fn fifo_pop(&mut self) -> Result<Value, E> {
        use crate::accelerometer::Accelerometer;
        use crate::gyroscope::Gyroscope;

        let gyro_scale = self.get_gyro_scale()?;
        let accel_scale = self.get_accel_scale()?;

        FifoOut::new(self.address).pop(&mut self.i2c, gyro_scale, accel_scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn test_pop_gyro() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            0x6b,
            vec![0x78],
            vec![0x01 << 3, 0, 1, 0, 2, 0, 4],
        )]);

        let mut f = FifoOut::new(crate::DEFAULT_I2C_ADDRESS);
        let v = f.pop(&mut i2c, FsG::Dps250, FsXl::G2).unwrap();

        assert!(matches!(v, Value::Gyro(_)));
        println!("{:?}", v);
        i2c.done();
    }
}
