use crate::RegisterBus;
use core::convert::{TryFrom, TryInto};

use crate::registers::{FsG, FsXl};
use crate::{AccelValue, GyroValue};

/// Sensor identifier encoded in a FIFO tag.
#[derive(Copy, Clone, Debug, Eq, PartialEq, defmt::Format)]
pub enum SensorTag {
    /// Empty tag.
    Empty,
    /// Gyroscope data (No Compression).
    GyroscopeNC,
    /// Accelerometer data (No Compression).
    AccelerometerNC,
    /// Temperature data.
    Temperature,
    /// Timestamp data.
    Timestamp,
    /// Configuration-change metadata.
    ConfigurationChange,
    /// Accelerometer compressed stream marker with two-sample history.
    AccelerometerNCT2,
    /// Accelerometer compressed stream marker with one-sample history.
    AccelerometerNCT1,
    /// Accelerometer compressed data with two samples.
    Accelerometer2xC,
    /// Accelerometer compressed data with three samples.
    Accelerometer3xC,
    /// Gyroscope compressed stream marker with two-sample history.
    GyroscopeNCT2,
    /// Gyroscope compressed stream marker with one-sample history.
    GyroscopeNCT1,
    /// Gyroscope compressed data with two samples.
    Gyroscope2xC,
    /// Gyroscope compressed data with three samples.
    Gyroscope3xC,
    /// Sensor-hub slave 0 data.
    SensorHubSlave0,
    /// Sensor-hub slave 1 data.
    SensorHubSlave1,
    /// Sensor-hub slave 2 data.
    SensorHubSlave2,
    /// Sensor-hub slave 3 data.
    SensorHubSlave3,
    /// Step-counter data.
    StepCounter,
    /// Sensor-hub NACK metadata.
    SensorHubNack,
    /// Reserved or unsupported tag value.
    Other(u8),
}

impl TryFrom<u8> for SensorTag {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0x00 => Ok(SensorTag::Empty),
            0x01 => Ok(SensorTag::GyroscopeNC),
            0x02 => Ok(SensorTag::AccelerometerNC),
            0x03 => Ok(SensorTag::Temperature),
            0x04 => Ok(SensorTag::Timestamp),
            0x05 => Ok(SensorTag::ConfigurationChange),
            0x06 => Ok(SensorTag::AccelerometerNCT2),
            0x07 => Ok(SensorTag::AccelerometerNCT1),
            0x08 => Ok(SensorTag::Accelerometer2xC),
            0x09 => Ok(SensorTag::Accelerometer3xC),
            0x0a => Ok(SensorTag::GyroscopeNCT2),
            0x0b => Ok(SensorTag::GyroscopeNCT1),
            0x0c => Ok(SensorTag::Gyroscope2xC),
            0x0d => Ok(SensorTag::Gyroscope3xC),
            0x0e => Ok(SensorTag::SensorHubSlave0),
            0x0f => Ok(SensorTag::SensorHubSlave1),
            0x10 => Ok(SensorTag::SensorHubSlave2),
            0x11 => Ok(SensorTag::SensorHubSlave3),
            0x12 => Ok(SensorTag::StepCounter),
            0x19 => Ok(SensorTag::SensorHubNack),
            x if x <= 0x1f => Ok(SensorTag::Other(x)),
            _ => Err(()),
        }
    }
}

/// FIFO tag metadata, including the time-slot counter and parity bit.
#[derive(Copy, Clone, Debug, Eq, PartialEq, defmt::Format)]
pub struct FifoTag {
    pub sensor: SensorTag,
    pub counter: u8,
    pub parity: bool,
}

impl FifoTag {
    fn from_byte(raw: u8) -> Self {
        Self {
            sensor: SensorTag::try_from((raw >> 3) & 0x1f).unwrap_or(SensorTag::Other(0x1f)),
            counter: (raw >> 1) & 0x03,
            parity: raw & 0x01 != 0,
        }
    }
}

/// Parsed FIFO value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, defmt::Format)]
pub enum Value {
    /// Empty value.
    Empty,
    /// Gyroscope reading.
    Gyro(GyroValue),
    /// Accelerometer reading.
    Accel(AccelValue),
    /// Temperature sample in raw sensor counts.
    Temperature(i16),
    /// Timestamp sample in raw timestamp ticks.
    Timestamp(u32),
    /// FIFO configuration-change metadata.
    ConfigurationChange([u8; 6]),
    /// Sensor-hub sample, identified by sensor-hub slave number.
    SensorHub(u8, [u8; 6]),
    /// Step-counter sample.
    StepCounter(u16),
    /// Sensor-hub NACK metadata.
    SensorHubNack([u8; 6]),
    /// Compressed accelerometer or gyroscope payload.
    Compressed(SensorTag, [u8; 6]),
    /// Reserved or unsupported payload.
    Other(u8, [u8; 6]),
}

/// A FIFO word with its tag metadata and decoded payload.
#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct FifoEntry {
    pub tag: FifoTag,
    pub value: Value,
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
    /// The first byte contains the sensor tag, time-slot counter, and parity.
    pub fn pop<I2C>(
        &mut self,
        i2c: &mut I2C,
        gyro_scale: FsG,
        accel_scale: FsXl,
    ) -> Result<Value, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(self.pop_entry(i2c, gyro_scale, accel_scale)?.value)
    }

    /// Pop a FIFO word while preserving tag metadata.
    pub fn pop_entry<I2C>(
        &mut self,
        i2c: &mut I2C,
        gyro_scale: FsG,
        accel_scale: FsXl,
    ) -> Result<FifoEntry, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut out = [0u8; 7];
        i2c.read_register(self.address, Register::FifoDataOutTag.addr(), &mut out)?;

        let tag = FifoTag::from_byte(out[0]);
        let data: &[u8; 6] = out[1..]
            .try_into()
            .expect("FIFO words contain six data bytes");
        let value = match tag.sensor {
            SensorTag::Empty => Value::Empty,
            SensorTag::GyroscopeNC => Value::Gyro(GyroValue::from_msr(gyro_scale, data)),
            SensorTag::AccelerometerNC => Value::Accel(AccelValue::from_msr(accel_scale, data)),
            SensorTag::Temperature => Value::Temperature(i16::from_le_bytes([data[0], data[1]])),
            SensorTag::Timestamp => {
                Value::Timestamp(u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
            }
            SensorTag::ConfigurationChange => Value::ConfigurationChange(*data),
            SensorTag::SensorHubSlave0 => Value::SensorHub(0, *data),
            SensorTag::SensorHubSlave1 => Value::SensorHub(1, *data),
            SensorTag::SensorHubSlave2 => Value::SensorHub(2, *data),
            SensorTag::SensorHubSlave3 => Value::SensorHub(3, *data),
            SensorTag::StepCounter => Value::StepCounter(u16::from_le_bytes([data[0], data[1]])),
            SensorTag::SensorHubNack => Value::SensorHubNack(*data),
            SensorTag::Other(raw) => Value::Other(raw, *data),
            sensor => Value::Compressed(sensor, *data),
        };

        Ok(FifoEntry { tag, value })
    }
}

use crate::Ism330Dhcx;
use crate::registers::{
    BdrGy, BdrXl, CounterBdrReg1, FifoCtrl1, FifoCtrl2, FifoCtrl3, FifoCtrl4, FifoMode,
    FifoStatus1, FifoStatus2, Register, TemperatureBatchRate, TimestampBatchDecimation,
    UncompressedDataRate,
};

/// FIFO methods.
pub trait Fifo {
    /// Set the FIFO watermark threshold in FIFO entries (0..=511).
    fn set_fifo_watermark<I2C>(&self, i2c: &mut I2C, watermark: u16) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Stop collecting FIFO samples when the watermark is reached.
    fn set_fifo_stop_on_watermark<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set the rate used for uncompressed FIFO data.
    fn set_fifo_uncompressed_rate<I2C>(
        &self,
        i2c: &mut I2C,
        rate: UncompressedDataRate,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Enable batching when the sensor output data rate changes.
    fn set_fifo_odr_change<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set FIFO timestamp batching decimation.
    fn set_fifo_timestamp_batching<I2C>(
        &self,
        i2c: &mut I2C,
        decimation: TimestampBatchDecimation,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set FIFO temperature batching rate.
    fn set_fifo_temperature_batch_rate<I2C>(
        &self,
        i2c: &mut I2C,
        rate: TemperatureBatchRate,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set FIFO mode.
    fn set_fifo_mode<I2C>(&mut self, i2c: &mut I2C, mode: FifoMode) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set accelerometer batch data rate.
    fn set_fifo_accel_batch_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        rate: BdrXl,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set gyroscope batch data rate.
    fn set_fifo_gyro_batch_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        rate: BdrGy,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Enable FIFO compression.
    fn set_fifo_compression<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Get FIFO status.
    fn get_fifo_status<I2C>(&self, i2c: &mut I2C) -> Result<(FifoStatus1, FifoStatus2), I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the number of unread FIFO entries (0..=1023).
    fn get_fifo_sample_count<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: RegisterBus;
    /// Pop a value from the FIFO.
    fn fifo_pop<I2C>(&self, i2c: &mut I2C) -> Result<Value, I2C::Error>
    where
        I2C: RegisterBus;
    /// Pop a FIFO word with tag metadata.
    fn fifo_pop_entry<I2C>(&self, i2c: &mut I2C) -> Result<FifoEntry, I2C::Error>
    where
        I2C: RegisterBus;
    /// Enable pulsed data-ready mode.
    fn set_dataready_pulsed<I2C>(&self, i2c: &mut I2C, pulsed: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Reset the internal counter of batch events.
    fn reset_counter_bdr<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Select the accelerometer or gyroscope batch-event counter trigger.
    fn set_trig_counter_bdr<I2C>(&self, i2c: &mut I2C, gyro: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set the 11-bit batch-event counter threshold.
    fn set_cnt_bdr_threshold<I2C>(&self, i2c: &mut I2C, threshold: u16) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
}

impl Fifo for Ism330Dhcx {
    fn set_fifo_watermark<I2C>(&self, i2c: &mut I2C, watermark: u16) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
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
        I2C: RegisterBus,
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
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_uncompressed_rate(rate);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_odr_change<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
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
        I2C: RegisterBus,
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
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::FifoCtrl4, |v| {
            let mut reg = FifoCtrl4::from_bytes([v]);
            reg.set_odr_t_batch(rate);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_mode<I2C>(&mut self, i2c: &mut I2C, mode: FifoMode) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
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
        I2C: RegisterBus,
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
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::FifoCtrl3, |v| {
            let mut reg = FifoCtrl3::from_bytes([v]);
            reg.set_bdr_gy(rate);
            reg.into_bytes()[0]
        })
    }

    fn set_fifo_compression<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_fifo_compr_rt_en(enable);
            reg.into_bytes()[0]
        })
    }

    fn get_fifo_status<I2C>(&self, i2c: &mut I2C) -> Result<(FifoStatus1, FifoStatus2), I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut out = [0u8; 2];
        self.read_register(i2c, Register::FifoStatus1.addr(), &mut out)?;
        Ok((
            FifoStatus1::from_bytes([out[0]]),
            FifoStatus2::from_bytes([out[1]]),
        ))
    }

    fn get_fifo_sample_count<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let (status1, status2) = self.get_fifo_status(i2c)?;
        Ok(u16::from(status1.diff_fifo()) | (u16::from(status2.diff_fifo()) << 8))
    }

    fn fifo_pop<I2C>(&self, i2c: &mut I2C) -> Result<Value, I2C::Error>
    where
        I2C: RegisterBus,
    {
        use crate::accelerometer::Accelerometer;
        use crate::gyroscope::Gyroscope;

        let gyro_scale = self.get_gyro_scale(i2c)?;
        let accel_scale = self.get_accel_scale(i2c)?;

        FifoOut::new(self.address).pop(i2c, gyro_scale, accel_scale)
    }

    fn fifo_pop_entry<I2C>(&self, i2c: &mut I2C) -> Result<FifoEntry, I2C::Error>
    where
        I2C: RegisterBus,
    {
        use crate::accelerometer::Accelerometer;
        use crate::gyroscope::Gyroscope;

        let gyro_scale = self.get_gyro_scale(i2c)?;
        let accel_scale = self.get_accel_scale(i2c)?;

        FifoOut::new(self.address).pop_entry(i2c, gyro_scale, accel_scale)
    }

    fn set_dataready_pulsed<I2C>(&self, i2c: &mut I2C, pulsed: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |value| {
            let mut reg = CounterBdrReg1::from_bytes([value]);
            reg.set_dataready_pulsed(pulsed);
            reg.into_bytes()[0]
        })
    }

    fn reset_counter_bdr<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |value| {
            let mut reg = CounterBdrReg1::from_bytes([value]);
            reg.set_rst_counter_bdr(true);
            reg.into_bytes()[0]
        })
    }

    fn set_trig_counter_bdr<I2C>(&self, i2c: &mut I2C, gyro: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |value| {
            let mut reg = CounterBdrReg1::from_bytes([value]);
            reg.set_trig_counter_bdr(gyro);
            reg.into_bytes()[0]
        })
    }

    fn set_cnt_bdr_threshold<I2C>(&self, i2c: &mut I2C, threshold: u16) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        let threshold = threshold & 0x07ff;
        self.modify_reg(i2c, Register::CounterBdrReg1, |value| {
            let mut reg = CounterBdrReg1::from_bytes([value]);
            reg.set_cnt_bdr_th_msb((threshold >> 8) as u8);
            reg.into_bytes()[0]
        })?;
        self.write_reg(i2c, Register::CounterBdrReg2, threshold as u8)
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
    fn recognizes_all_documented_fifo_sensor_tags() {
        let tags = [
            (0x00, SensorTag::Empty),
            (0x01, SensorTag::GyroscopeNC),
            (0x02, SensorTag::AccelerometerNC),
            (0x03, SensorTag::Temperature),
            (0x04, SensorTag::Timestamp),
            (0x05, SensorTag::ConfigurationChange),
            (0x06, SensorTag::AccelerometerNCT2),
            (0x07, SensorTag::AccelerometerNCT1),
            (0x08, SensorTag::Accelerometer2xC),
            (0x09, SensorTag::Accelerometer3xC),
            (0x0a, SensorTag::GyroscopeNCT2),
            (0x0b, SensorTag::GyroscopeNCT1),
            (0x0c, SensorTag::Gyroscope2xC),
            (0x0d, SensorTag::Gyroscope3xC),
            (0x0e, SensorTag::SensorHubSlave0),
            (0x0f, SensorTag::SensorHubSlave1),
            (0x10, SensorTag::SensorHubSlave2),
            (0x11, SensorTag::SensorHubSlave3),
            (0x12, SensorTag::StepCounter),
            (0x19, SensorTag::SensorHubNack),
        ];

        for (raw, expected) in tags {
            assert_eq!(SensorTag::try_from(raw), Ok(expected));
        }
        assert_eq!(SensorTag::try_from(0x20), Err(()));
    }

    #[test]
    fn decodes_metadata_and_special_fifo_payloads() {
        let words = [
            (0x03 << 3 | 0b01, Value::Temperature(0x1234)),
            (0x04 << 3 | 0b10, Value::Timestamp(0x78561234)),
            (
                0x0e << 3,
                Value::SensorHub(0, [0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc]),
            ),
            (0x12 << 3, Value::StepCounter(0x1234)),
            (
                0x19 << 3,
                Value::SensorHubNack([0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc]),
            ),
        ];

        for (tag, expected) in words {
            let mut i2c = Mock::new(&[Transaction::write_read(
                crate::DEFAULT_I2C_ADDRESS,
                vec![Register::FifoDataOutTag.addr()],
                vec![tag, 0x34, 0x12, 0x56, 0x78, 0x9a, 0xbc],
            )]);
            let mut fifo = FifoOut::new(crate::DEFAULT_I2C_ADDRESS);
            let entry = fifo.pop_entry(&mut i2c, FsG::Dps250, FsXl::G2).unwrap();

            assert_eq!(entry.value, expected);
            i2c.done();
        }
    }

    #[test]
    fn preserves_fifo_tag_counter_and_parity() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            crate::DEFAULT_I2C_ADDRESS,
            vec![Register::FifoDataOutTag.addr()],
            vec![0x02 << 3 | 0b111, 0, 0, 0, 0, 0, 0],
        )]);
        let mut fifo = FifoOut::new(crate::DEFAULT_I2C_ADDRESS);

        let entry = fifo.pop_entry(&mut i2c, FsG::Dps250, FsXl::G2).unwrap();

        assert_eq!(entry.tag.sensor, SensorTag::AccelerometerNC);
        assert_eq!(entry.tag.counter, 3);
        assert!(entry.tag.parity);
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
