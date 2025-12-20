use modular_bitfield::prelude::*;

/// Accelerometer LPF2 and HP filter configuration and cutoff setting.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 3]
pub enum HpcfXl {
    /// ODR/4
    OdrDiv4 = 0b000,
    /// ODR/10
    OdrDiv10 = 0b001,
    /// ODR/20
    OdrDiv20 = 0b010,
    /// ODR/45
    OdrDiv45 = 0b011,
    /// ODR/100
    OdrDiv100 = 0b100,
    /// ODR/200
    OdrDiv200 = 0b101,
    /// ODR/400
    OdrDiv400 = 0b110,
    /// ODR/800
    OdrDiv800 = 0b111,
}

/// Control register 8 (17h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl8Xl {
    /// LPF2 on 6D function selection.
    pub low_pass_on_6d: bool,
    #[skip]
    pub __: B1,
    /// Accelerometer slope filter / high-pass filter selection.
    pub hp_slope_xl_en: bool,
    /// Enables accelerometer LPF2 and HPF fast-settling mode.
    pub fastsettl_mode_xl: bool,
    /// Enables accelerometer high-pass filter reference mode.
    pub hp_ref_mode_xl: bool,
    /// Accelerometer LPF2 and HP filter configuration and cutoff setting.
    pub hpcf_xl: HpcfXl,
}

impl Default for Ctrl8Xl {
    fn default() -> Self {
        Self::new()
    }
}
