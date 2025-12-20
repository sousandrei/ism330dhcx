use bitfield::bitfield;

bitfield! {
    /// Free-fall, wakeup and sleep mode functions duration setting register (5Ch)
    pub struct WakeUpDur(u8);
    impl Debug;
    /// Duration to go in sleep mode.
    pub sleep_dur, set_sleep_dur: 3, 0;
    /// Weight of 1 LSB of wakeup threshold.
    pub wake_ths_w, set_wake_ths_w: 4;
    /// Wake up duration event.
    pub wake_dur, set_wake_dur: 6, 5;
    /// Free fall duration event (MSB).
    pub ff_dur5, set_ff_dur5: 7;
}

impl WakeUpDur {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for WakeUpDur {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for WakeUpDur {}
impl Clone for WakeUpDur {
    fn clone(&self) -> Self {
        *self
    }
}
