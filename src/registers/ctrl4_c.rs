use bitfield::bitfield;

bitfield! {
    /// Control register 4 (13h)
    pub struct Ctrl4C(u8);
    impl Debug;
    /// Enables gyroscope digital LPF1.
    pub lpf1_sel_g, set_lpf1_sel_g: 1;
    /// Disables I2C interface.
    pub i2c_disable, set_i2c_disable: 2;
    /// Enables data available mask until filter settling ends.
    pub drdy_mask, set_drdy_mask: 3;
    /// All interrupt signals available on INT1 pin enable.
    pub int2_on_int1, set_int2_on_int1: 5;
    /// Enables gyroscope Sleep mode.
    pub sleep_g, set_sleep_g: 6;
}

impl Ctrl4C {
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

impl Default for Ctrl4C {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Ctrl4C {}
impl Clone for Ctrl4C {
    fn clone(&self) -> Self {
        *self
    }
}
