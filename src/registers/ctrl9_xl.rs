use bitfield::bitfield;

bitfield! {
    /// Control register 9 (XL).
    pub struct Ctrl9Xl(u8);
    impl Debug;
    /// Device configuration.
    pub device_conf, set_device_conf: 1;
    /// DEN active level.
    pub den_lh, set_den_lh: 2;
    /// DEN stamping on accelerometer axis.
    pub den_xl_en, set_den_xl_en: 3;
    /// DEN stamping on gyroscope axis.
    pub den_xl_g, set_den_xl_g: 4;
    /// DEN value stored in LSB of Z-axis.
    pub den_z, set_den_z: 5;
    /// DEN value stored in LSB of Y-axis.
    pub den_y, set_den_y: 6;
    /// DEN value stored in LSB of X-axis.
    pub den_x, set_den_x: 7;
}

impl Ctrl9Xl {
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

impl Default for Ctrl9Xl {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Ctrl9Xl {}
impl Clone for Ctrl9Xl {
    fn clone(&self) -> Self {
        *self
    }
}
