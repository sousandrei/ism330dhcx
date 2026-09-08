//! OIS and auxiliary SPI register definitions.

use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2, B3},
};

/// OIS interrupt and self-test configuration.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IntOis {
    pub st_xl_ois: B2,
    #[skip]
    pub __: B3,
    pub den_lh_ois: bool,
    pub lvl2_ois: bool,
    pub int2_drdy_ois: bool,
}

impl Default for IntOis {
    fn default() -> Self {
        Self::new()
    }
}

/// OIS gyroscope and interface configuration.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Ois {
    pub ois_en_spi2: bool,
    pub fs_125_ois: bool,
    pub fs_g_ois: B2,
    pub mode4_en: bool,
    pub sim_ois: bool,
    pub lvl1_ois: bool,
    #[skip]
    pub __: B1,
}

impl Default for Ctrl1Ois {
    fn default() -> Self {
        Self::new()
    }
}

/// OIS gyroscope filter configuration.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2Ois {
    pub hp_en_ois: bool,
    pub ftype_ois: B2,
    #[skip]
    pub __: B1,
    pub hpm_ois: B2,
    #[skip]
    pub ___: B2,
}

impl Default for Ctrl2Ois {
    fn default() -> Self {
        Self::new()
    }
}

/// OIS accelerometer configuration.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl3Ois {
    pub st_ois_clampdis: bool,
    pub st_ois: B2,
    pub filter_xl_conf_ois: B3,
    pub fs_xl_ois: B2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_reset_values() {
        assert_eq!(IntOis::default().into_bytes(), [0]);
        assert_eq!(Ctrl1Ois::default().into_bytes(), [0]);
        assert_eq!(Ctrl2Ois::default().into_bytes(), [0]);
        assert_eq!(Ctrl3Ois::default().into_bytes(), [0]);
    }
}

impl Default for Ctrl3Ois {
    fn default() -> Self {
        Self::new()
    }
}
