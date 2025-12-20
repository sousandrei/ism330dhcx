use modular_bitfield::prelude::*;

/// Threshold for 4D/6D function.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 2]
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

/// Portrait/landscape position and tap function threshold register (59h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct TapThs6d {
    /// Z-axis recognition threshold.
    pub tap_ths_z: B5,
    /// Threshold for 4D/6D function.
    pub sixd_ths: SixdThs,
    /// Enables detection of 4D orientation.
    pub d4d_en: bool,
}

impl Default for TapThs6d {
    fn default() -> Self {
        Self::new()
    }
}
