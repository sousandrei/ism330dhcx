use bitfield::bitfield;

// TODO: one of this for each axis: X, Y, Z

bitfield! {
    /// Accelerometer user offset correction registers (73h - 75h)
    pub struct OfsUsr(u8);
    impl Debug;
    /// Accelerometer user offset correction value
    pub ofs_usr, set_ofs_usr: 7, 0;
}

impl OfsUsr {
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

impl Default for OfsUsr {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for OfsUsr {}
impl Clone for OfsUsr {
    fn clone(&self) -> Self {
        *self
    }
}
