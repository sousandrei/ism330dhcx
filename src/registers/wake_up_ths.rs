use modular_bitfield::prelude::*;

/// Single/double-tap selection and wake-up configuration (5Bh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct WakeUpThs {
    /// Threshold for wakeup.
    pub wk_ths: B6,
    /// Sends the low-pass filtered data with user offset correction to the wakeup function.
    pub usr_off_on_wu: bool,
    /// Single/double-tap event enable.
    pub single_double_tap: bool,
}

impl Default for WakeUpThs {
    fn default() -> Self {
        Self::new()
    }
}
