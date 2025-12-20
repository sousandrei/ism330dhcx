use bitfield::bitfield;

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

impl Copy for FifoCtrl2 {}
impl Clone for FifoCtrl2 {
    fn clone(&self) -> Self {
        *self
    }
}

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

impl Copy for FifoCtrl3 {}
impl Clone for FifoCtrl3 {
    fn clone(&self) -> Self {
        *self
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

impl Copy for FifoCtrl4 {}
impl Clone for FifoCtrl4 {
    fn clone(&self) -> Self {
        *self
    }
}
