use bitfield::bitfield;

bitfield! {
    /// Status register (1Eh)
    pub struct StatusReg(u8);
    impl Debug;
    /// Accelerometer new data available.
    pub xlda, _: 0;
    /// Gyroscope new data available.
    pub gda, _: 1;
    /// Temperature new data available.
    pub tda, _: 2;
}

impl StatusReg {
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

impl Default for StatusReg {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for StatusReg {}
impl Clone for StatusReg {
    fn clone(&self) -> Self {
        *self
    }
}
