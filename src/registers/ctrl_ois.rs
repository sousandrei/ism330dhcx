use modular_bitfield::prelude::*;

/// OIS configuration register 1 (70h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl1Ois {
    /// Enables OIS chain data processing.
    pub ois_en_spi2: bool,
    /// Selects gyroscope OIS chain full-scale +-125 dps.
    pub fs_125_ois: bool,
    /// Selects gyroscope OIS chain full-scale.
    pub fs_g_ois: B2,
    /// Enables accelerometer OIS chain.
    pub mode4_en: bool,
    /// SPI2 3- or 4-wire interface.
    pub sim_ois: bool,
    /// Enables level-sensitive trigger mode on OIS chain.
    pub lvl1_ois: bool,
    #[skip]
    pub __: B1,
}

impl Default for Ctrl1Ois {
    fn default() -> Self {
        Self::new()
    }
}

/// OIS configuration register 2 (71h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl2Ois {
    /// Enables gyroscope OIS chain digital high-pass filter.
    pub hp_en_ois: bool,
    /// Selects gyroscope digital LPF1 filter bandwidth.
    pub ftype_ois: B2,
    #[skip]
    pub __1: B1,
    /// Selects gyroscope OIS chain digital high-pass filter cutoff.
    pub hpm_ois: B2,
    #[skip]
    pub __2: B2,
}

impl Default for Ctrl2Ois {
    fn default() -> Self {
        Self::new()
    }
}

/// OIS configuration register 3 (72h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl3Ois {
    /// Disables OIS chain clamp.
    pub st_ois_clampdis: bool,
    /// Selects gyroscope OIS chain self-test.
    pub st_ois: B2,
    /// Selects accelerometer OIS channel bandwidth.
    pub filter_xl_conf_ois: B3,
    /// Selects accelerometer OIS channel full-scale.
    pub fs_xl_ois: B2,
}

impl Default for Ctrl3Ois {
    fn default() -> Self {
        Self::new()
    }
}
