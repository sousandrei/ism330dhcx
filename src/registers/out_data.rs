use bitfield::bitfield;

bitfield! {
    /// Gyroscope and Accelerometer output data registers (22h - 2Dh)
    pub struct OutData(u16);
    impl Debug;
    pub low, _: 7, 0;
    pub high, _: 15, 8;
}

impl OutData {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
    pub fn into_bytes(self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
    pub fn get_value(&self) -> i16 {
        self.0 as i16
    }
}

impl Copy for OutData {}
impl Clone for OutData {
    fn clone(&self) -> Self {
        *self
    }
}
