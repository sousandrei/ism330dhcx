use modular_bitfield::prelude::*;

/// Timestamp registers (40h - 43h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Timestamp {
    pub value: B8,
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new()
    }
}
