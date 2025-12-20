use bitfield::bitfield;

bitfield! {
    /// FIFO status registers.
    pub struct FifoStatus(u16);
    impl Debug;
    /// Unread FIFO samples (lower 10 bits).
    pub diff_fifo, _: 9, 0;
    /// FIFO Overrun interrupt status.
    pub fifo_ovr_ia, _: 11;
    /// FIFO Full interrupt status.
    pub fifo_full_ia, _: 12;
}

impl FifoStatus {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
    pub fn into_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

impl Default for FifoStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for FifoStatus {}
impl Clone for FifoStatus {
    fn clone(&self) -> Self {
        *self
    }
}
