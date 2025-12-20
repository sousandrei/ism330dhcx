#![allow(unused_parens)]
use modular_bitfield::{
    bitfield,
    specifiers::{B1, B3, B10},
};

/// FIFO status registers.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FifoStatus {
    /// Unread FIFO samples (lower 10 bits).
    pub diff_fifo: B10,
    #[skip]
    pub __: B1,
    /// FIFO Overrun interrupt status.
    pub fifo_ovr_ia: bool,
    /// FIFO Full interrupt status.
    pub fifo_full_ia: bool,
    #[skip]
    pub __: B3,
}

impl Default for FifoStatus {
    fn default() -> Self {
        Self::new()
    }
}
