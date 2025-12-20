use modular_bitfield::prelude::*;

/// Accelerometer user offset correction registers (73h - 75h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct OfsUsr {
    /// Accelerometer user offset correction.
    pub ofs_usr: B8,
}

impl Default for OfsUsr {
    fn default() -> Self {
        Self::new()
    }
}
