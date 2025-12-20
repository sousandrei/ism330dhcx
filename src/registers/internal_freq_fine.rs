use bitfield::bitfield;

bitfield! {
    /// Internal frequency register (63h)
    pub struct InternalFreqFine(u8);
    impl Debug;
    /// Difference in percentage of the effective ODR. 8-bit format, 2's complement.
    pub freq_fine, set_freq_fine: 7, 0;
}

impl InternalFreqFine {
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

impl Default for InternalFreqFine {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for InternalFreqFine {}
impl Clone for InternalFreqFine {
    fn clone(&self) -> Self {
        *self
    }
}
