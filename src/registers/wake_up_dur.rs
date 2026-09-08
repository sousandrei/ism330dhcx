use modular_bitfield::{
    bitfield,
    specifiers::{B2, B4},
};
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct WakeUpDur {
    pub sleep_dur: B4,
    pub wake_ths_w: bool,
    pub wake_dur: B2,
    pub ff_dur: bool,
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
        r.set_wake_ths_w(true);
        r.set_wake_dur(3);
        r.set_ff_dur(true);
        assert_eq!(r.into_bytes(), [0xff]);
    }
}
