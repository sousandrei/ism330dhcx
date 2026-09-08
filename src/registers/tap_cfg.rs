#![allow(non_snake_case)]
use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2, B5},
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
    pub __: B2,
    pub interrupt_enable: bool,
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
    #[skip]
    pub __: B1,
    #[skip]
    pub ___: B1,
    #[skip]
    pub ____: B1,
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
    #[skip]
    pub __: B1,
    #[skip]
    pub ___: B1,
    #[skip]
    pub ____: B1,
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
        assert_eq!(r.into_bytes(), [0x1f]);
    }
}
