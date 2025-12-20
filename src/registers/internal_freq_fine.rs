use modular_bitfield::prelude::*;

/// Internal frequency register (63h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct InternalFreqFine {
    /// Difference in percentage of the effective ODR. 8-bit format, 2's complement.
    pub freq_fine: B8,
}

impl Default for InternalFreqFine {
    fn default() -> Self {
        Self::new()
    }
}
