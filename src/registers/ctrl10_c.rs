use modular_bitfield::{bitfield, specifiers::B3};

/// Control register 10.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl10C {
    #[skip]
    pub reserved: B3,
    pub den_lh: bool,
    pub pedo_rst_step: bool,
    pub timestamp_en: bool,
    pub sign_motion_en: bool,
    pub func_en: bool,
}
impl Default for Ctrl10C {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Ctrl10C::default().into_bytes(), [0]);
        let mut r = Ctrl10C::new();
        r.set_timestamp_en(true);
        assert_eq!(r.into_bytes(), [0x20]);
    }
}
