use modular_bitfield::prelude::*;

/// Status register (1Eh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct StatusReg {
    /// Accelerometer new data available.
    pub xlda: bool,
    /// Gyroscope new data available.
    pub gda: bool,
    /// Temperature new data available.
    pub tda: bool,
    #[skip]
    pub __: B5,
}

impl Default for StatusReg {
    fn default() -> Self {
        Self::new()
    }
}
