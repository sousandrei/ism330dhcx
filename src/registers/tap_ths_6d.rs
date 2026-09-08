use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2, B5},
};
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct TapThs6d {
    pub tap_ths_z: B5,
    pub sixd_ths: B2,
    #[skip]
    pub __: B1,
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
        assert_eq!(r.into_bytes(), [0x60]);
    }
}
