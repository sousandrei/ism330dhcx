use bitfield::bitfield;

/// Free-fall threshold setting.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum FfThs {
    /// 156 mg
    Mg156 = 0b000,
    /// 219 mg
    Mg219 = 0b001,
    /// 250 mg
    Mg250 = 0b010,
    /// 312 mg
    Mg312 = 0b011,
    /// 344 mg
    Mg344 = 0b100,
    /// 406 mg
    Mg406 = 0b101,
    /// 469 mg
    Mg469 = 0b110,
    /// 500 mg
    Mg500 = 0b111,
}

impl From<u8> for FfThs {
    fn from(val: u8) -> Self {
        match val {
            0b000 => FfThs::Mg156,
            0b001 => FfThs::Mg219,
            0b010 => FfThs::Mg250,
            0b011 => FfThs::Mg312,
            0b100 => FfThs::Mg344,
            0b101 => FfThs::Mg406,
            0b110 => FfThs::Mg469,
            0b111 => FfThs::Mg500,
            _ => FfThs::Mg156,
        }
    }
}

impl From<FfThs> for u8 {
    fn from(val: FfThs) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Free-fall function duration setting register (5Dh)
    pub struct FreeFall(u8);
    impl Debug;
    /// Free-fall threshold setting.
    pub from into FfThs, ff_ths, set_ff_ths: 2, 0;
    /// Free-fall duration event (LSBs).
    pub ff_dur, set_ff_dur: 7, 3;
}

impl FreeFall {
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

impl Default for FreeFall {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for FreeFall {}
impl Clone for FreeFall {
    fn clone(&self) -> Self {
        *self
    }
}
