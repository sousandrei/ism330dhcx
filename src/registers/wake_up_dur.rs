use modular_bitfield::{
    bitfield,
    specifiers::{B2, B4},
};
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct WakeUpDur {
    pub sleep_dur: B4,
    pub wake_ths_w: B2,
    pub ff_dur: B2,
}
impl Default for WakeUpDur {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(WakeUpDur::default().into_bytes(), [0]);
        let mut r = WakeUpDur::new();
        r.set_sleep_dur(0xf);
        assert_eq!(r.into_bytes(), [0x0f]);
    }
}
