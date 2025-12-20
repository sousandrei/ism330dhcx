use bitfield::bitfield;

bitfield! {
    /// Tap recognition function setting register (5Ah)
    pub struct IntDur2(u8);
    impl Debug;
    /// Maximum duration of overthreshold event.
    pub shock, set_shock: 1, 0;
    /// Expected quiet time after a tap detection.
    pub quiet, set_quiet: 3, 2;
    /// Duration of maximum time gap for double-tap recognition.
    pub dur, set_dur: 7, 4;
}

impl IntDur2 {
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

impl Default for IntDur2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for IntDur2 {}
impl Clone for IntDur2 {
    fn clone(&self) -> Self {
        *self
    }
}
