use bitfield::bitfield;

/// Threshold for 4D/6D function.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum SixdThs {
    /// 80 degrees
    Deg80 = 0b00,
    /// 70 degrees
    Deg70 = 0b01,
    /// 60 degrees
    Deg60 = 0b10,
    /// 50 degrees
    Deg50 = 0b11,
}

impl From<u8> for SixdThs {
    fn from(val: u8) -> Self {
        match val {
            0b00 => SixdThs::Deg80,
            0b01 => SixdThs::Deg70,
            0b10 => SixdThs::Deg60,
            0b11 => SixdThs::Deg50,
            _ => SixdThs::Deg80,
        }
    }
}

impl From<SixdThs> for u8 {
    fn from(val: SixdThs) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Portrait/landscape position and tap function threshold register (59h)
    pub struct TapThs6d(u8);
    impl Debug;
    /// Z-axis recognition threshold.
    pub tap_ths_z, set_tap_ths_z: 4, 0;
    /// Threshold for 4D/6D function.
    pub from into SixdThs, sixd_ths, set_sixd_ths: 6, 5;
    /// Enables detection of 4D orientation.
    pub d4d_en, set_d4d_en: 7;
}

impl TapThs6d {
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

impl Default for TapThs6d {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for TapThs6d {}
impl Clone for TapThs6d {
    fn clone(&self) -> Self {
        *self
    }
}
