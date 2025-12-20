use modular_bitfield::prelude::*;

/// OIS interrupt configuration register (6Fh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct IntOis {
    /// Selects accelerometer self-test.
    pub st_xl_ois: B2,
    #[skip]
    pub __1: B1,
    #[skip]
    pub __2: B2,
    /// Indicates polarity of DEN signal on OIS chain.
    pub den_lh_ois: bool,
    /// Enables level-sensitive latched mode on the OIS chain.
    pub lvl2_ois: bool,
    /// Enables OIS chain DRDY on INT2 pin.
    pub int2_drdy_ois: bool,
}

impl Default for IntOis {
    fn default() -> Self {
        Self::new()
    }
}
