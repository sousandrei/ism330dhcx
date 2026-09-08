use bitfield::bitfield;
use core::convert::TryInto;
use embedded_hal::i2c::I2c;

use crate::Ism330Dhcx;
use crate::registers::{AccelValue, FsG, FsXl, GyroValue, Register};

/// Gyroscope batch data rate.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum BdrGy {
    /// Not batched
    Off = 0b0000,
    /// 6.5 Hz
    Hz6_5 = 0b1011,
    /// 12.5 Hz
    Hz12_5 = 0b0001,
    /// 26 Hz
    Hz26 = 0b0010,
    /// 52 Hz
    Hz52 = 0b0011,
    /// 104 Hz
    Hz104 = 0b0100,
    /// 208 Hz
    Hz208 = 0b0101,
    /// 417 Hz
    Hz417 = 0b0110,
    /// 833 Hz
    Hz833 = 0b0111,
    /// 1.667 kHz
    Hz1667 = 0b1000,
    /// 3.333 kHz
    Hz3333 = 0b1001,
    /// 6.667 kHz
    Hz6667 = 0b1010,
}

impl From<u8> for BdrGy {
    fn from(val: u8) -> Self {
        match val {
            0b0000 => BdrGy::Off,
            0b1011 => BdrGy::Hz6_5,
            0b0001 => BdrGy::Hz12_5,
            0b0010 => BdrGy::Hz26,
            0b0011 => BdrGy::Hz52,
            0b0100 => BdrGy::Hz104,
            0b0101 => BdrGy::Hz208,
            0b0110 => BdrGy::Hz417,
            0b0111 => BdrGy::Hz833,
            0b1000 => BdrGy::Hz1667,
            0b1001 => BdrGy::Hz3333,
            0b1010 => BdrGy::Hz6667,
            _ => BdrGy::Off,
        }
    }
}

impl From<BdrGy> for u8 {
    fn from(val: BdrGy) -> u8 {
        val as u8
    }
}

/// Accelerometer batch data rate.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum BdrXl {
    /// Not batched
    Off = 0b0000,
    /// 6.5 Hz
    Hz6_5 = 0b1011,
    /// 12.5 Hz
    Hz12_5 = 0b0001,
    /// 26 Hz
    Hz26 = 0b0010,
    /// 52 Hz
    Hz52 = 0b0011,
    /// 104 Hz
    Hz104 = 0b0100,
    /// 208 Hz
    Hz208 = 0b0101,
    /// 417 Hz
    Hz417 = 0b0110,
    /// 833 Hz
    Hz833 = 0b0111,
    /// 1.667 kHz
    Hz1667 = 0b1000,
    /// 3.333 kHz
    Hz3333 = 0b1001,
    /// 6.667 kHz
    Hz6667 = 0b1010,
}

impl From<u8> for BdrXl {
    fn from(val: u8) -> Self {
        match val {
            0b0000 => BdrXl::Off,
            0b1011 => BdrXl::Hz6_5,
            0b0001 => BdrXl::Hz12_5,
            0b0010 => BdrXl::Hz26,
            0b0011 => BdrXl::Hz52,
            0b0100 => BdrXl::Hz104,
            0b0101 => BdrXl::Hz208,
            0b0110 => BdrXl::Hz417,
            0b0111 => BdrXl::Hz833,
            0b1000 => BdrXl::Hz1667,
            0b1001 => BdrXl::Hz3333,
            0b1010 => BdrXl::Hz6667,
            _ => BdrXl::Off,
        }
    }
}

impl From<BdrXl> for u8 {
    fn from(val: BdrXl) -> u8 {
        val as u8
    }
}

/// FIFO mode selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum FifoMode {
    /// Bypass mode. FIFO disabled.
    Bypass = 0b000,
    /// FIFO mode. Stops collecting when full.
    FifoMode = 0b001,
    /// Continuous-to-FIFO mode.
    ContinuousToFifo = 0b011,
    /// Bypass-to-Continuous mode.
    BypassToContinuous = 0b100,
    /// Continuous mode. Overwrites oldest when full.
    Continuous = 0b110,
    /// Bypass-to-FIFO mode.
    BypassToFifo = 0b111,
}

impl From<u8> for FifoMode {
    fn from(val: u8) -> Self {
        match val {
            0b000 => FifoMode::Bypass,
            0b001 => FifoMode::FifoMode,
            0b011 => FifoMode::ContinuousToFifo,
            0b100 => FifoMode::BypassToContinuous,
            0b110 => FifoMode::Continuous,
            0b111 => FifoMode::BypassToFifo,
            _ => FifoMode::Bypass,
        }
    }
}

impl From<FifoMode> for u8 {
    fn from(val: FifoMode) -> u8 {
        val as u8
    }
}

/// Decimation for timestamp batching in FIFO.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum DecTsBatch {
    /// Not batched
    Off = 0b00,
    /// Decimation 1
    Dec1 = 0b01,
    /// Decimation 8
    Dec8 = 0b10,
    /// Decimation 32
    Dec32 = 0b11,
}

impl From<u8> for DecTsBatch {
    fn from(val: u8) -> Self {
        match val {
            0b01 => DecTsBatch::Dec1,
            0b10 => DecTsBatch::Dec8,
            0b11 => DecTsBatch::Dec32,
            _ => DecTsBatch::Off,
        }
    }
}

impl From<DecTsBatch> for u8 {
    fn from(val: DecTsBatch) -> u8 {
        val as u8
    }
}

/// Batch data rate for temperature data.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum OdrTBatch {
    /// Not batched
    Off = 0b00,
    /// 1.6 Hz
    Hz1_6 = 0b01,
    /// 12.5 Hz
    Hz12_5 = 0b10,
    /// 52 Hz
    Hz52 = 0b11,
}

impl From<u8> for OdrTBatch {
    fn from(val: u8) -> Self {
        match val {
            0b01 => OdrTBatch::Hz1_6,
            0b10 => OdrTBatch::Hz12_5,
            0b11 => OdrTBatch::Hz52,
            _ => OdrTBatch::Off,
        }
    }
}

impl From<OdrTBatch> for u8 {
    fn from(val: OdrTBatch) -> u8 {
        val as u8
    }
}

/// FIFO sensor identifiers.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum TagSensor {
    Empty = 0x00,
    GyroNC = 0x01,
    AccelNC = 0x02,
    Temperature = 0x03,
    Timestamp = 0x04,
    CfgChange = 0x05,
    AccelNcT2 = 0x06,
    AccelNcT1 = 0x07,
    Accel2xC = 0x08,
    Accel3xC = 0x09,
    GyroNcT2 = 0x0A,
    GyroNcT1 = 0x0B,
    Gyro2xC = 0x0C,
    Gyro3xC = 0x0D,
    SensorHubSlave0 = 0x0E,
    SensorHubSlave1 = 0x0F,
    SensorHubSlave2 = 0x10,
    SensorHubSlave3 = 0x11,
    StepCounter = 0x12,
    SensorHubNack = 0x19,
}

impl From<u8> for TagSensor {
    fn from(val: u8) -> Self {
        match val {
            0x00 => TagSensor::Empty,
            0x01 => TagSensor::GyroNC,
            0x02 => TagSensor::AccelNC,
            0x03 => TagSensor::Temperature,
            0x04 => TagSensor::Timestamp,
            0x05 => TagSensor::CfgChange,
            0x06 => TagSensor::AccelNcT2,
            0x07 => TagSensor::AccelNcT1,
            0x08 => TagSensor::Accel2xC,
            0x09 => TagSensor::Accel3xC,
            0x0A => TagSensor::GyroNcT2,
            0x0B => TagSensor::GyroNcT1,
            0x0C => TagSensor::Gyro2xC,
            0x0D => TagSensor::Gyro3xC,
            0x0E => TagSensor::SensorHubSlave0,
            0x0F => TagSensor::SensorHubSlave1,
            0x10 => TagSensor::SensorHubSlave2,
            0x11 => TagSensor::SensorHubSlave3,
            0x12 => TagSensor::StepCounter,
            0x19 => TagSensor::SensorHubNack,
            _ => TagSensor::Empty,
        }
    }
}

impl From<TagSensor> for u8 {
    fn from(val: TagSensor) -> u8 {
        val as u8
    }
}

bitfield! {
    /// FIFO control register 2.
    pub struct FifoCtrl2(u8);
    impl Debug;
    /// FIFO compression enable.
    pub fifo_compr_rt_en, set_fifo_compr_rt_en: 6;
    /// Stop on watermark enable.
    pub stop_on_wtm, set_stop_on_wtm: 7;
}

impl FifoCtrl2 {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for FifoCtrl2 {
    fn default() -> Self {
        Self::new()
    }
}

bitfield! {
    /// FIFO control register 3.
    pub struct FifoCtrl3(u8);
    impl Debug;
    /// Accelerometer batch data rate.
    pub from into BdrXl, bdr_xl, set_bdr_xl: 3, 0;
    /// Gyroscope batch data rate.
    pub from into BdrGy, bdr_gy, set_bdr_gy: 7, 4;
}

impl FifoCtrl3 {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for FifoCtrl3 {
    fn default() -> Self {
        Self::new()
    }
}

bitfield! {
    /// FIFO control register 4.
    pub struct FifoCtrl4(u8);
    impl Debug;
    /// FIFO mode selection.
    pub from into FifoMode, fifo_mode, set_fifo_mode: 2, 0;
    /// Selects batch data rate for temperature data.
    pub from into OdrTBatch, odr_t_batch, set_odr_t_batch: 5, 4;
    /// Selects decimation for timestamp batching in FIFO.
    pub from into DecTsBatch, dec_ts_batch, set_dec_ts_batch: 7, 6;
}

impl FifoCtrl4 {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for FifoCtrl4 {
    fn default() -> Self {
        Self::new()
    }
}

bitfield! {
    /// FIFO status registers.
    pub struct FifoStatus(u16);
    impl Debug;
    /// Unread FIFO samples (lower 10 bits).
    pub diff_fifo, _: 9, 0;
    /// FIFO Overrun Latched.
    pub fifo_ovr_latched, _: 11;
    /// Counter BDR reaches the CNT_BDR_TH_[10:0] threshold.
    pub counter_bdr_ia, _: 12;
    /// FIFO Full status.
    pub fifo_full_ia, _: 13;
    /// FIFO Overrun status.
    pub fifo_ovr_ia, _: 14;
    /// Set FIFO Watermark Status.
    pub fifo_wtm_ia, _: 15;
}

impl FifoStatus {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
    pub fn into_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

impl Default for FifoStatus {
    fn default() -> Self {
        Self::new()
    }
}

bitfield! {
    /// FIFO tag register (78h)
    pub struct FifoDataOutTag(u8);
    impl Debug;
    /// Parity check of TAG content.
    pub tag_parity, set_tag_parity: 0;
    /// 2-bit counter which identifies sensor time slot.
    pub tag_cnt, set_tag_cnt: 2, 1;
    /// Identifies the sensor.
    pub from into TagSensor, tag_sensor, set_tag_sensor: 7, 3;
}

impl FifoDataOutTag {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for FifoDataOutTag {
    fn default() -> Self {
        Self::new()
    }
}

bitfield! {
    /// FIFO data out registers
    pub struct FifoDataOut(u16);
    impl Debug;
    /// FIFO data output value
    pub fifo_data_out, set_fifo_data_out: 15, 0;
}

impl FifoDataOut {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u16; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u16; 1] {
        [self.0]
    }
}

impl Default for FifoDataOut {
    fn default() -> Self {
        Self::new()
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

        let (tag_raw, data) = out.split_at(1);
        let tag = FifoDataOutTag::from_bytes([tag_raw[0]]);
        let data: &[u8; 6] = data.try_into().expect("must be 6!");

        match tag.tag_sensor() {
            TagSensor::Empty => Ok(Value::Empty),
            TagSensor::GyroNC => Ok(Value::Gyro(GyroValue::from_msr(gyro_scale, data))),
            TagSensor::AccelNC => Ok(Value::Accel(AccelValue::from_msr(accel_scale, data))),
            TagSensor::Timestamp => {
                let _ts_raw = [data[0], data[1], data[2], data[3]];
                Ok(Value::Other(TagSensor::Timestamp as u8, data.clone()))
            }
            TagSensor::SensorHubNack => Ok(Value::Empty),
            _ => Ok(Value::Other(tag.tag_sensor() as u8, *data)),
        }
    }
}

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
        use crate::registers::Accelerometer;
        use crate::registers::Gyroscope;

        let gyro_scale = self.get_gyro_scale(i2c)?;
        let accel_scale = self.get_accel_scale(i2c)?;

        FifoOut::new(self.address).pop(i2c, gyro_scale, accel_scale)
    }
}

/// Configuration methods for FIFO_CTRL2 register.
pub trait FifoCtrl2Config {
    /// FIFO compression enable.
    fn set_fifo_compr_rt_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Stop on watermark enable.
    fn set_stop_on_wtm<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl FifoCtrl2Config for Ism330Dhcx {
    fn set_fifo_compr_rt_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_fifo_compr_rt_en(val);
            reg.into_bytes()[0]
        })
    }

    fn set_stop_on_wtm<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl2, |v| {
            let mut reg = FifoCtrl2::from_bytes([v]);
            reg.set_stop_on_wtm(val);
            reg.into_bytes()[0]
        })
    }
}

/// Configuration methods for FIFO_CTRL3 register.
pub trait FifoCtrl3Config {
    /// Accelerometer batch data rate.
    fn set_bdr_xl<I2C>(&self, i2c: &mut I2C, val: BdrXl) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Gyroscope batch data rate.
    fn set_bdr_gy<I2C>(&self, i2c: &mut I2C, val: BdrGy) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl FifoCtrl3Config for Ism330Dhcx {
    fn set_bdr_xl<I2C>(&self, i2c: &mut I2C, val: BdrXl) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl3, |v| {
            let mut reg = FifoCtrl3::from_bytes([v]);
            reg.set_bdr_xl(val);
            reg.into_bytes()[0]
        })
    }

    fn set_bdr_gy<I2C>(&self, i2c: &mut I2C, val: BdrGy) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl3, |v| {
            let mut reg = FifoCtrl3::from_bytes([v]);
            reg.set_bdr_gy(val);
            reg.into_bytes()[0]
        })
    }
}

/// Configuration methods for FIFO_CTRL4 register.
pub trait FifoCtrl4Config {
    /// FIFO mode selection.
    fn set_fifo_mode<I2C>(&self, i2c: &mut I2C, val: FifoMode) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Selects batch data rate for temperature data.
    fn set_odr_t_batch<I2C>(&self, i2c: &mut I2C, val: OdrTBatch) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Selects decimation for timestamp batching in FIFO.
    fn set_dec_ts_batch<I2C>(&self, i2c: &mut I2C, val: DecTsBatch) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl FifoCtrl4Config for Ism330Dhcx {
    fn set_fifo_mode<I2C>(&self, i2c: &mut I2C, val: FifoMode) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl4, |v| {
            let mut reg = FifoCtrl4::from_bytes([v]);
            reg.set_fifo_mode(val);
            reg.into_bytes()[0]
        })
    }

    fn set_odr_t_batch<I2C>(&self, i2c: &mut I2C, val: OdrTBatch) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl4, |v| {
            let mut reg = FifoCtrl4::from_bytes([v]);
            reg.set_odr_t_batch(val);
            reg.into_bytes()[0]
        })
    }

    fn set_dec_ts_batch<I2C>(&self, i2c: &mut I2C, val: DecTsBatch) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FifoCtrl4, |v| {
            let mut reg = FifoCtrl4::from_bytes([v]);
            reg.set_dec_ts_batch(val);
            reg.into_bytes()[0]
        })
    }
}
