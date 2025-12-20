use bitfield::bitfield;

bitfield! {
    /// Source register for all interrupts (1Ah)
    pub struct AllIntSrc(u8);
    impl Debug;
    /// Free-fall event status.
    pub ff_ia, set_ff_ia: 0;
    /// Wake-up event status.
    pub wu_ia, set_wu_ia: 1;
    /// Single-tap event status.
    pub single_tap, set_single_tap: 2;
    /// Double-tap event status.
    pub double_tap, set_double_tap: 3;
    /// 6D orientation change event status.
    pub d6d_ia, set_d6d_ia: 4;
    /// Detects change event in activity/inactivity status.
    pub sleep_change_ia, set_sleep_change_ia: 5;
    /// Alerts timestamp overflow.
    pub timestamp_endcount, set_timestamp_endcount: 7;
}

impl AllIntSrc {
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

impl Default for AllIntSrc {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for AllIntSrc {}
impl Clone for AllIntSrc {
    fn clone(&self) -> Self {
        *self
    }
}
