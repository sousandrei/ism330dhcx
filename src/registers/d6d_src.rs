use modular_bitfield::bitfield;
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct D6dSrc {
    pub xl: bool,
    pub xh: bool,
    pub yl: bool,
    pub yh: bool,
    pub zl: bool,
    pub zh: bool,
    pub d6d_ia: bool,
    pub den_drdy: bool,
}
impl Default for D6dSrc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(D6dSrc::default().into_bytes(), [0]);
        let mut r = D6dSrc::new();
        r.set_d6d_ia(true);
        assert_eq!(r.into_bytes(), [0x40]);
        r.set_den_drdy(true);
        r.set_zh(true);
        r.set_zl(true);
        r.set_yh(true);
        r.set_yl(true);
        r.set_xh(true);
        r.set_xl(true);
        assert_eq!(r.into_bytes(), [0xff]);
    }
}
