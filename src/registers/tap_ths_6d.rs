use modular_bitfield::{
    bitfield,
    specifiers::{B2, B5},
};
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TapThs6d {
    pub tap_ths_z: B5,
    pub sixd_ths: B2,
    pub d4d_en: bool,
}
impl Default for TapThs6d {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(TapThs6d::default().into_bytes(), [0]);
        let mut r = TapThs6d::new();
        r.set_sixd_ths(3);
        r.set_d4d_en(true);
        assert_eq!(r.into_bytes(), [0xe0]);
    }
}
