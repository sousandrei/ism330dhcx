use bitfield::bitfield;

bitfield! {
    /// OIS interrupt configuration register (6Fh)
    pub struct IntOis(u8);
    impl Debug;
    /// Selects accelerometer self-test.
    pub st_xl_ois, set_st_xl_ois: 1, 0;
    /// Indicates polarity of DEN signal on OIS chain.
    pub den_lh_ois, set_den_lh_ois: 5;
    /// Enables level-sensitive latched mode on the OIS chain.
    pub lvl2_ois, set_lvl2_ois: 6;
    /// Enables OIS chain DRDY on INT2 pin.
    pub int2_drdy_ois, set_int2_drdy_ois: 7;
}

impl IntOis {
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

impl Default for IntOis {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for IntOis {}
impl Clone for IntOis {
    fn clone(&self) -> Self {
        *self
    }
}
