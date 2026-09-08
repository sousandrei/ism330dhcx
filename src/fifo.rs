use core::convert::{TryFrom, TryInto};
use embedded_hal::i2c::I2c;

use crate::registers::{FsG, FsXl};
use crate::{AccelValue, GyroValue};

/// Sensor tag identifying the data source in the FIFO.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
        i2c.write_read(self.address, &[Register::FifoDataOutTag.addr()], &mut out)?;

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
            Err(_) => Ok(Value::Other(tag, *out)),
        }
    }
}

use crate::Ism330Dhcx;
use crate::registers::{
    BdrGy, BdrXl, FifoCtrl1, FifoCtrl2, FifoCtrl3, FifoCtrl4, FifoMode, FifoStatus1, FifoStatus2,
    Register, TemperatureBatchRate, TimestampBatchDecimation, UncompressedDataRate,
};

/// FIFO methods.
pub trait Fifo {
    /// Set the FIFO watermark threshold in FIFO entries (0..=511).
    fn set_fifo_watermark<I2C>(&self, i2c: &mut I2C, watermark: u16) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Stop collecting FIFO samples when the watermark is reached.
    fn set_fifo_stop_on_watermark<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set the rate used for uncompressed FIFO data.
    fn set_fifo_uncompressed_rate<I2C>(
        &self,
        i2c: &mut I2C,
        rate: UncompressedDataRate,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enable batching when the sensor output data rate changes.
    fn set_fifo_odr_change<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set FIFO timestamp batching decimation.
    fn set_fifo_timestamp_batching<I2C>(
        &self,
        i2c: &mut I2C,
        decimation: TimestampBatchDecimation,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set FIFO temperature batching rate.
    fn set_fifo_temperature_batch_rate<I2C>(
        &self,
        i2c: &mut I2C,
        rate: TemperatureBatchRate,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
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
    fn get_fifo_status<I2C>(&self, i2c: &mut I2C) -> Result<(FifoStatus1, FifoStatus2), I2C::Error>
    where
        I2C: I2c;
    /// Read the number of unread FIFO entries (0..=1023).
    fn get_fifo_sample_count<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c;
    /// Pop a value from the FIFO.
    fn fifo_pop<I2C>(&self, i2c: &mut I2C) -> Result<Value, I2C::Error>
    where
        I2C: I2c;
}

impl Fifo for Ism330Dhcx {
    fn set_fifo_watermark<I2C>(&self, i2c: &mut I2C, watermark: u16) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        assert!(watermark <= 0x01ff, "FIFO watermark must fit in 9 bits");
        self.write_reg(
            i2c,
            Register::FifoCtrl1,
            FifoCtrl1::new().with_wtm(watermark as u8).into_bytes()[0],
        )?;
        self.modify_reg(i2c, Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_wtm8(watermark & 0x0100 != 0);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_stop_on_watermark<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_stop_on_wtm(enable);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_uncompressed_rate<I2C>(
        &self,
        i2c: &mut I2C,
        rate: UncompressedDataRate,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_uncompressed_rate(rate);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_odr_change<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_odrchg_en(enable);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_timestamp_batching<I2C>(
        &self,
        i2c: &mut I2C,
        decimation: TimestampBatchDecimation,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl4, |v| {
            let mut reg = FifoCtrl4::from_bytes([v]);
            reg.set_dec_ts_batch(decimation);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_temperature_batch_rate<I2C>(
        &self,
        i2c: &mut I2C,
        rate: TemperatureBatchRate,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl4, |v| {
            let mut reg = FifoCtrl4::from_bytes([v]);
            reg.set_odr_t_batch(rate);
            reg.into_bytes()[0]
        })
    }

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

    fn get_fifo_status<I2C>(&self, i2c: &mut I2C) -> Result<(FifoStatus1, FifoStatus2), I2C::Error>
    where
        I2C: I2c,
    {
        let mut out = [0u8; 2];
        i2c.write_read(self.address, &[Register::FifoStatus1.addr()], &mut out)?;
        Ok((
            FifoStatus1::from_bytes([out[0]]),
            FifoStatus2::from_bytes([out[1]]),
        ))
    }

    fn get_fifo_sample_count<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c,
    {
        let (status1, status2) = self.get_fifo_status(i2c)?;
        Ok(u16::from(status1.diff_fifo()) | (u16::from(status2.diff_fifo()) << 8))
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
            vec![Register::FifoDataOutTag.addr()],
            vec![0x01 << 3, 0, 1, 0, 2, 0, 4],
        )]);

        let mut f = FifoOut::new(crate::DEFAULT_I2C_ADDRESS);
        let v = f.pop(&mut i2c, FsG::Dps250, FsXl::G2).unwrap();

        assert!(matches!(v, Value::Gyro(_)));
        println!("{:?}", v);
        i2c.done();
    }

    #[test]
    fn test_set_fifo_watermark() {
        let sensor = crate::Ism330Dhcx {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write(
                crate::DEFAULT_I2C_ADDRESS,
                vec![Register::FifoCtrl1.addr(), 0x23],
            ),
            Transaction::write_read(
                crate::DEFAULT_I2C_ADDRESS,
                vec![Register::FifoCtrl2.addr()],
                vec![0x40],
            ),
            Transaction::write(
                crate::DEFAULT_I2C_ADDRESS,
                vec![Register::FifoCtrl2.addr(), 0x41],
            ),
        ]);

        sensor.set_fifo_watermark(&mut i2c, 0x123).unwrap();
        i2c.done();
    }

    #[test]
    fn test_get_fifo_status_decodes_both_registers() {
        let sensor = crate::Ism330Dhcx {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write_read(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::FifoStatus1.addr()],
            vec![0xaa, 0xfb],
        )]);

        let (status1, status2) = sensor.get_fifo_status(&mut i2c).unwrap();
        assert_eq!(status1.diff_fifo(), 0xaa);
        assert_eq!(status2.diff_fifo(), 0b11);
        assert!(status2.fifo_wtm_ia());
        assert!(status2.fifo_ovr_latched());
        i2c.done();
    }

    #[test]
    fn test_get_fifo_sample_count_combines_status_registers() {
        let sensor = crate::Ism330Dhcx {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write_read(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::FifoStatus1.addr()],
            vec![0xaa, 0x03],
        )]);

        assert_eq!(sensor.get_fifo_sample_count(&mut i2c).unwrap(), 0x3aa);
        i2c.done();
    }

    #[test]
    fn test_reserved_fifo_tag_is_returned_as_other() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::FifoDataOutTag.addr()],
            vec![0x1f << 3, 0, 1, 2, 3, 4, 5],
        )]);
        let mut fifo = FifoOut::new(crate::DEFAULT_I2C_ADDRESS);

        let value = fifo.pop(&mut i2c, FsG::Dps250, FsXl::G2).unwrap();
        assert!(matches!(value, Value::Other(0x1f, _)));
        i2c.done();
    }

    #[test]
    fn test_set_fifo_timestamp_batching_preserves_mode() {
        let sensor = crate::Ism330Dhcx {
            address: crate::DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                crate::DEFAULT_I2C_ADDRESS,
                vec![Register::FifoCtrl4.addr()],
                vec![FifoMode::Continuous as u8],
            ),
            Transaction::write(
                crate::DEFAULT_I2C_ADDRESS,
                vec![Register::FifoCtrl4.addr(), 0x86],
            ),
        ]);

        sensor
            .set_fifo_timestamp_batching(&mut i2c, TimestampBatchDecimation::Every8)
            .unwrap();
        i2c.done();
    }
}
