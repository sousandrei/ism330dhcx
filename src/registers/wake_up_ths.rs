use modular_bitfield::{
    bitfield,
    specifiers::{B1, B6},
};
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct WakeUpThs {
    pub wk_ths: B6,
    pub usr_off_on_wu: bool,
    #[skip]
    pub __: B1,
}
impl Default for WakeUpThs {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(WakeUpThs::default().into_bytes(), [0]);
        let mut r = WakeUpThs::new();
        r.set_wk_ths(0x3f);
        assert_eq!(r.into_bytes(), [0x3f]);
    }
}
