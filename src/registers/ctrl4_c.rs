use modular_bitfield::{bitfield, specifiers::B1};

/// Control register 4.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl4C {
    pub den_xl_en: bool,
    pub sleep_g: bool,
    pub i2c_disable: bool,
    pub drdy_mask: bool,
    #[skip]
    pub __: B1,
    pub int2_on_int1: bool,
    #[skip]
    pub ___: B1,
    pub den_lh: bool,
}

impl Default for Ctrl4C {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Ctrl4C::default().into_bytes(), [0]);
        let mut r = Ctrl4C::new();
        r.set_den_lh(true);
        assert_eq!(r.into_bytes(), [0x80]);
    }
}
