//! OIS and auxiliary SPI register definitions.

use modular_bitfield::{
    Specifier, bitfield,
    specifiers::{B1, B2, B3},
};

/// OIS gyroscope full-scale selection.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum OisGyroScale {
    /// +/-250 dps.
    Dps250 = 0,
    /// +/-500 dps.
    Dps500 = 1,
    /// +/-1000 dps.
    Dps1000 = 2,
    /// +/-2000 dps.
    Dps2000 = 3,
}

/// OIS accelerometer full-scale selection.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum OisAccelScale {
    /// +/-2 g.
    G2 = 0,
    /// +/-16 g.
    G16 = 1,
    /// +/-4 g.
    G4 = 2,
    /// +/-8 g.
    G8 = 3,
}

/// OIS gyroscope low-pass filter selection.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum OisFilterType {
    /// 297 Hz bandwidth.
    Bw297Hz = 0,
    /// 222 Hz bandwidth.
    Bw222Hz = 1,
    /// 154 Hz bandwidth.
    Bw154Hz = 2,
    /// 470 Hz bandwidth.
    Bw470Hz = 3,
}

/// OIS gyroscope high-pass filter cutoff.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum OisFilterCutoff {
    /// 16 mHz.
    Mhz16 = 0,
    /// 65 mHz.
    Mhz65 = 1,
    /// 260 mHz.
    Mhz260 = 2,
    /// 1.04 Hz.
    Hz1_04 = 3,
}

/// OIS gyroscope self-test selection.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum OisSelfTest {
    /// Normal mode.
    Normal = 0,
    /// Positive self-test.
    Positive = 1,
    /// Negative self-test.
    Negative = 3,
}

/// OIS accelerometer self-test selection.
#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum OisAccelSelfTest {
    /// Normal mode.
    Normal = 0,
    /// Positive self-test.
    Positive = 1,
    /// Negative self-test.
    Negative = 2,
}

/// OIS interrupt and self-test configuration.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IntOis {
    pub st_xl_ois: OisAccelSelfTest,
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
    pub fs_g_ois: OisGyroScale,
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
    pub ftype_ois: OisFilterType,
    #[skip]
    pub __: B1,
    pub hpm_ois: OisFilterCutoff,
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
    pub st_ois: OisSelfTest,
    pub filter_xl_conf_ois: B3,
    pub fs_xl_ois: OisAccelScale,
}

impl Default for Ctrl3Ois {
    fn default() -> Self {
        Self::new()
    }
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
