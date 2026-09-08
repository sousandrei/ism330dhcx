use modular_bitfield::{Specifier, bitfield};

#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 3]
pub enum HpcfXl {
    OdrDiv4 = 0b00,
    OdrDiv10 = 0b01,
    OdrDiv20 = 0b10,
    OdrDiv45 = 0b11,
    OdrDiv100 = 0b100,
    OdrDiv200 = 0b101,
    OdrDiv400 = 0b110,
    OdrDiv800 = 0b111,
}

/// Control register 8 (accelerometer).
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl8Xl {
    pub low_pass_on_6d: bool,
    pub slope_fds: bool,
    pub hp_slope_xl_en: bool,
    pub fast_settling_mode_xl: bool,
    pub hp_ref_mode: bool,
    pub hpcf_xl: HpcfXl,
}
impl Default for Ctrl8Xl {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Ctrl8Xl::default().into_bytes(), [0]);
        let mut r = Ctrl8Xl::new();
        r.set_low_pass_on_6d(true);
        assert_eq!(r.into_bytes(), [0x01]);
    }

    #[test]
    fn filter_fields_preserve_their_positions() {
        let mut reg = Ctrl8Xl::new();
        reg.set_low_pass_on_6d(true);
        reg.set_hp_slope_xl_en(true);
        reg.set_fast_settling_mode_xl(true);
        reg.set_slope_fds(true);
        reg.set_hpcf_xl(HpcfXl::OdrDiv800);
        reg.set_hp_ref_mode(true);
        assert_eq!(reg.into_bytes(), [0xff]);
    }
}
