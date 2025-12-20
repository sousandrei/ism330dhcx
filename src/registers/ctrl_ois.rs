use bitfield::bitfield;

bitfield! {
    /// OIS configuration register 1 (70h)
    pub struct Ctrl1Ois(u8);
    impl Debug;
    /// Enables OIS chain data processing.
    pub ois_en_spi2, set_ois_en_spi2: 0;
    /// Selects gyroscope OIS chain full-scale +-125 dps.
    pub fs_125_ois, set_fs_125_ois: 1;
    /// Selects gyroscope OIS chain full-scale.
    pub fs_g_ois, set_fs_g_ois: 3, 2;
    /// Enables accelerometer OIS chain.
    pub mode4_en, set_mode4_en: 4;
    /// SPI2 3- or 4-wire interface.
    pub sim_ois, set_sim_ois: 5;
    /// Enables level-sensitive trigger mode on OIS chain.
    pub lvl1_ois, set_lvl1_ois: 6;
}

impl Ctrl1Ois {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for Ctrl1Ois {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Ctrl1Ois {}
impl Clone for Ctrl1Ois {
    fn clone(&self) -> Self {
        *self
    }
}

bitfield! {
    /// OIS configuration register 2 (71h)
    pub struct Ctrl2Ois(u8);
    impl Debug;
    /// Enables gyroscope OIS chain digital high-pass filter.
    pub hp_en_ois, set_hp_en_ois: 0;
    /// Selects gyroscope digital LPF1 filter bandwidth.
    pub ftype_ois, set_ftype_ois: 2, 1;
    /// Selects gyroscope OIS chain digital high-pass filter cutoff.
    pub hpm_ois, set_hpm_ois: 5, 4;
}

impl Ctrl2Ois {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for Ctrl2Ois {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Ctrl2Ois {}
impl Clone for Ctrl2Ois {
    fn clone(&self) -> Self {
        *self
    }
}

bitfield! {
    /// OIS configuration register 3 (72h)
    pub struct Ctrl3Ois(u8);
    impl Debug;
    /// Disables OIS chain clamp.
    pub st_ois_clampdis, set_st_ois_clampdis: 0;
    /// Selects gyroscope OIS chain self-test.
    pub st_ois, set_st_ois: 2, 1;
    /// Selects accelerometer OIS channel bandwidth.
    pub filter_xl_conf_ois, set_filter_xl_conf_ois: 5, 3;
    /// Selects accelerometer OIS channel full-scale.
    pub fs_xl_ois, set_fs_xl_ois: 7, 6;
}

impl Ctrl3Ois {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for Ctrl3Ois {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Ctrl3Ois {}
impl Clone for Ctrl3Ois {
    fn clone(&self) -> Self {
        *self
    }
}
