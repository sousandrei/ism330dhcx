use modular_bitfield::prelude::*;

/// Control register 10 (19h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl10C {
    #[skip]
    pub __1: B2,
    /// Enables timestamp counter.
    pub timestamp_en: bool,
    #[skip]
    pub __2: B5,
}

impl Default for Ctrl10C {
    fn default() -> Self {
        Self::new()
    }
}
