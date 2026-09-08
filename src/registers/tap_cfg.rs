#![allow(non_snake_case)]
use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2, B3, B5},
};

/// Tap and interrupt configuration register 0.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct TapCfg0 {
    pub lir: bool,
    pub tap_x_en: bool,
    pub tap_y_en: bool,
    pub tap_z_en: bool,
    pub slope_fds: bool,
    pub sleep_status_on_int: bool,
    pub int_clr_on_read: bool,
    #[skip]
    pub __: B1,
}
impl Default for TapCfg0 {
    fn default() -> Self {
        Self::new()
    }
}

/// Tap threshold and axis priority configuration register 1.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct TapCfg1 {
    pub tap_ths_x: B5,
    pub tap_priority: B3,
}
impl Default for TapCfg1 {
    fn default() -> Self {
        Self::new()
    }
}

/// Tap threshold and axis priority configuration register 2.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct TapCfg2 {
    pub tap_ths_y: B5,
    pub inact_en: B2,
    pub interrupts_enable: bool,
}
impl Default for TapCfg2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(TapCfg0::default().into_bytes(), [0]);
        assert_eq!(TapCfg1::default().into_bytes(), [0]);
        assert_eq!(TapCfg2::default().into_bytes(), [0]);
        let mut r = TapCfg1::new();
        r.set_tap_ths_x(0x1f);
        r.set_tap_priority(0b101);
        assert_eq!(r.into_bytes(), [0xbf]);
        let mut r = TapCfg2::new();
        r.set_tap_ths_y(0x1f);
        r.set_inact_en(3);
        r.set_interrupts_enable(true);
        assert_eq!(r.into_bytes(), [0xff]);
        let mut r = TapCfg0::new();
        r.set_sleep_status_on_int(true);
        r.set_int_clr_on_read(true);
        assert_eq!(r.into_bytes(), [0x60]);
    }
}
