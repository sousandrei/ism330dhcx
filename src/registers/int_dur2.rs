use modular_bitfield::prelude::*;

/// Tap recognition function setting register (5Ah)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct IntDur2 {
    /// Maximum duration of overthreshold event.
    pub shock: B2,
    /// Expected quiet time after a tap detection.
    pub quiet: B2,
    /// Duration of maximum time gap for double-tap recognition.
    pub dur: B4,
}

impl Default for IntDur2 {
    fn default() -> Self {
        Self::new()
    }
}
