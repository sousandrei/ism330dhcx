use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2, B8},
};

/// FIFO status register 1.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FifoStatus1 {
    /// Low eight bits of the number of unread FIFO samples.
    pub diff_fifo: B8,
}

impl Default for FifoStatus1 {
    fn default() -> Self {
        Self::new()
    }
}

/// FIFO status register 2.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FifoStatus2 {
    /// High two bits of the number of unread FIFO samples.
    pub diff_fifo: B2,
    #[skip]
    pub __: B1,
    /// Latched FIFO overrun status.
    pub fifo_ovr_latched: bool,
    /// Counter BDR threshold status.
    pub counter_bdr_ia: bool,
    /// FIFO Full interrupt status.
    pub fifo_full_ia: bool,
    /// FIFO Overrun interrupt status.
    pub fifo_ovr_ia: bool,
    /// FIFO watermark status.
    pub fifo_wtm_ia: bool,
}

impl Default for FifoStatus2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_and_default() {
        assert_eq!(FifoStatus1::default().into_bytes(), [0]);
        assert_eq!(FifoStatus2::default().into_bytes(), [0]);

        let mut status = FifoStatus2::new();
        status.set_fifo_wtm_ia(true);
        status.set_fifo_ovr_ia(true);
        status.set_fifo_full_ia(true);
        status.set_counter_bdr_ia(true);
        status.set_fifo_ovr_latched(true);
        status.set_diff_fifo(0b11);
        assert_eq!(status.into_bytes(), [0xfb]);
    }
}
