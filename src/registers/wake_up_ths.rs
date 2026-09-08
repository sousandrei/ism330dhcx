use modular_bitfield::{bitfield, specifiers::B6};
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WakeUpThs {
    pub wk_ths: B6,
    pub usr_off_on_wu: bool,
    pub single_double_tap: bool,
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
        r.set_single_double_tap(true);
        assert_eq!(r.into_bytes(), [0xbf]);
    }
}
