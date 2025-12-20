use modular_bitfield::prelude::*;

/// Control register 4 (13h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl4C {
    #[skip]
    pub __1: B1,
    /// Enables gyroscope digital LPF1.
    pub lpf1_sel_g: bool,
    /// Disables I2C interface.
    pub i2c_disable: bool,
    /// Enables data available mask until filter settling ends.
    pub drdy_mask: bool,
    #[skip]
    pub __2: B1,
    /// All interrupt signals available on INT1 pin enable.
    pub int2_on_int1: bool,
    /// Enables gyroscope Sleep mode.
    pub sleep_g: bool,
    #[skip]
    pub __3: B1,
}

impl Default for Ctrl4C {
    fn default() -> Self {
        Self::new()
    }
}
