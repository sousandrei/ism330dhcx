use bitfield::bitfield;

/// Accelerometer LPF2 and HP filter configuration and cutoff setting.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
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

impl From<u8> for HpcfXl {
    fn from(val: u8) -> Self {
        match val {
            0b000 => HpcfXl::OdrDiv4,
            0b001 => HpcfXl::OdrDiv10,
            0b010 => HpcfXl::OdrDiv20,
            0b011 => HpcfXl::OdrDiv45,
            0b100 => HpcfXl::OdrDiv100,
            0b101 => HpcfXl::OdrDiv200,
            0b110 => HpcfXl::OdrDiv400,
            0b111 => HpcfXl::OdrDiv800,
            _ => HpcfXl::OdrDiv4,
        }
    }
}

impl From<HpcfXl> for u8 {
    fn from(val: HpcfXl) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Control register 8 (17h)
    pub struct Ctrl8Xl(u8);
    impl Debug;
    /// LPF2 on 6D function selection.
    pub low_pass_on_6d, set_low_pass_on_6d: 0;
    /// Accelerometer slope filter / high-pass filter selection.
    pub hp_slope_xl_en, set_hp_slope_xl_en: 2;
    /// Enables accelerometer LPF2 and HPF fast-settling mode.
    pub fastsettl_mode_xl, set_fastsettl_mode_xl: 3;
    /// Enables accelerometer high-pass filter reference mode.
    pub hp_ref_mode_xl, set_hp_ref_mode_xl: 4;
    /// Accelerometer LPF2 and HP filter configuration and cutoff setting.
    pub from into HpcfXl, hpcf_xl, set_hpcf_xl: 7, 5;
}

impl Ctrl8Xl {
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

impl Default for Ctrl8Xl {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Ctrl8Xl {}
impl Clone for Ctrl8Xl {
    fn clone(&self) -> Self {
        *self
    }
}
