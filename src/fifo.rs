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
    pub fn pop<I2C>(
        &mut self,
        i2c: &mut I2C,
        gyro_scale: FsG,
        accel_scale: FsXl,
    ) -> Result<Value, I2C::Error>
    where
        I2C: I2c,
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
pub trait Fifo {
    /// Set FIFO mode.
    fn set_fifo_mode<I2C>(&mut self, i2c: &mut I2C, mode: FifoMode) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set accelerometer batch data rate.
    fn set_fifo_accel_batch_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        rate: BdrXl,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set gyroscope batch data rate.
    fn set_fifo_gyro_batch_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        rate: BdrGy,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enable FIFO compression.
    fn set_fifo_compression<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Get FIFO status.
    fn get_fifo_status<I2C>(&self, i2c: &mut I2C) -> Result<FifoStatus, I2C::Error>
    where
        I2C: I2c;
    /// Pop a value from the FIFO.
    fn fifo_pop<I2C>(&self, i2c: &mut I2C) -> Result<Value, I2C::Error>
    where
        I2C: I2c;
}

impl Fifo for Ism330Dhcx {
    fn set_fifo_mode<I2C>(&mut self, i2c: &mut I2C, mode: FifoMode) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl4, |v| {
            let mut reg = FifoCtrl4::from_bytes([v]);
            reg.set_fifo_mode(mode);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_accel_batch_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        rate: BdrXl,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl3, |v| {
            let mut reg = FifoCtrl3::from_bytes([v]);
            reg.set_bdr_xl(rate);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_gyro_batch_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        rate: BdrGy,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl3, |v| {
            let mut reg = FifoCtrl3::from_bytes([v]);
            reg.set_bdr_gy(rate);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_compression<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_fifo_compr_rt_en(enable);
            reg.into_bytes()[0]
        })
    }

    fn get_fifo_status<I2C>(&self, i2c: &mut I2C) -> Result<FifoStatus, I2C::Error>
    where
        I2C: I2c,
    {
        let mut out = [0u8; 2];
        i2c.write_read(self.address, &[Register::FifoStatus1.addr()], &mut out)?;
        Ok(FifoStatus::from_bytes(out))
    }

    fn fifo_pop<I2C>(&self, i2c: &mut I2C) -> Result<Value, I2C::Error>
    where
        I2C: I2c,
    {
        use crate::accelerometer::Accelerometer;
        use crate::gyroscope::Gyroscope;

        let gyro_scale = self.get_gyro_scale(i2c)?;
        let accel_scale = self.get_accel_scale(i2c)?;

        FifoOut::new(self.address).pop(i2c, gyro_scale, accel_scale)
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
