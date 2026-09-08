use modular_bitfield::{bitfield, specifiers::B1};
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TapSrc {
    pub z_tap: bool,
    pub y_tap: bool,
    pub x_tap: bool,
    pub tap_sign: bool,
    pub double_tap: bool,
    pub single_tap: bool,
    pub tap_ia: bool,
    #[skip]
    pub __: B1,
}
impl Default for TapSrc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(TapSrc::default().into_bytes(), [0]);
        let mut r = TapSrc::new();
        r.set_tap_ia(true);
        assert_eq!(r.into_bytes(), [0x40]);
        r.set_single_tap(true);
        r.set_double_tap(true);
        r.set_tap_sign(true);
        r.set_x_tap(true);
        r.set_y_tap(true);
        r.set_z_tap(true);
        assert_eq!(r.into_bytes(), [0x7f]);
    }
}
