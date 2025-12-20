use bitfield::bitfield;

bitfield! {
    /// Single/double-tap selection and wake-up configuration (5Bh)
    pub struct WakeUpThs(u8);
    impl Debug;
    /// Threshold for wakeup.
    pub wk_ths, set_wk_ths: 5, 0;
    /// Sends the low-pass filtered data with user offset correction to the wakeup function.
    pub usr_off_on_wu, set_usr_off_on_wu: 6;
    /// Single/double-tap event enable.
    pub single_double_tap, set_single_double_tap: 7;
}

impl WakeUpThs {
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

impl Default for WakeUpThs {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for WakeUpThs {}
impl Clone for WakeUpThs {
    fn clone(&self) -> Self {
        *self
    }
}
