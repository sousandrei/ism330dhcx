use modular_bitfield::prelude::*;

/// Free-fall, wakeup and sleep mode functions duration setting register (5Ch)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct WakeUpDur {
    /// Duration to go in sleep mode.
    pub sleep_dur: B4,
    /// Weight of 1 LSB of wakeup threshold.
    pub wake_ths_w: bool,
    /// Wake up duration event.
    pub wake_dur: B2,
    /// Free fall duration event (MSB).
    pub ff_dur5: bool,
}

impl Default for WakeUpDur {
    fn default() -> Self {
        Self::new()
    }
}
