use modular_bitfield::prelude::*;

/// Gyroscope low-pass filter (LPF1) bandwidth selection.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 3]
pub enum Ftype {
    /// Bandwidth 0
    Bw0 = 0b000,
    /// Bandwidth 1
    Bw1 = 0b001,
    /// Bandwidth 2
    Bw2 = 0b010,
    /// Bandwidth 3
    Bw3 = 0b011,
    /// Bandwidth 4
    Bw4 = 0b100,
    /// Bandwidth 5
    Bw5 = 0b101,
    /// Bandwidth 6
    Bw6 = 0b110,
    /// Bandwidth 7
    Bw7 = 0b111,
}

/// Control register 6 (15h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl6C {
    /// Gyroscope low-pass filter (LPF1) bandwidth selection.
    pub ftype: Ftype,
    /// Weight of XL user offset bits.
    pub usr_off_w: bool,
    /// Disables high-performance operating mode for accelerometer.
    pub xl_hm_mode: bool,
    /// Enables DEN level-sensitive latched mode.
    pub lvl2_en: bool,
    /// Enables DEN data level-sensitive trigger mode.
    pub lvl1_en: bool,
    /// Enables DEN data edge-sensitive trigger mode.
    pub trig_en: bool,
}

impl Default for Ctrl6C {
    fn default() -> Self {
        Self::new()
    }
}
