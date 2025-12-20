use bitfield::bitfield;

bitfield! {
    /// FIFO status registers.
    pub struct FifoStatus(u16);
    impl Debug;
    /// Unread FIFO samples (lower 10 bits).
    pub diff_fifo, _: 9, 0;
    /// FIFO Overrun Latched.
    pub fifo_ovr_latched, _: 11;
    /// Counter BDR reaches the CNT_BDR_TH_[10:0] threshold.
    pub counter_bdr_ia, _: 12;
    /// FIFO Full status.
    pub fifo_full_ia, _: 13;
    /// FIFO Overrun status.
    pub fifo_ovr_ia, _: 14;
    /// Set FIFO Watermark Status.
    pub fifo_wtm_ia, _: 15;
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
