use modular_bitfield::prelude::*;

/// Free-fall threshold setting.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 3]
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

/// Free-fall function duration setting register (5Dh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FreeFall {
    /// Free-fall threshold setting.
    pub ff_ths: FfThs,
    /// Free-fall duration event (LSBs).
    pub ff_dur: B5,
}

impl Default for FreeFall {
    fn default() -> Self {
        Self::new()
    }
}
