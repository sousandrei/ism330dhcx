use bitfield::bitfield;

bitfield! {
    /// Timestamp registers (40h - 43h)
    pub struct Timestamp(u32);
    impl Debug;
    pub value, set_value: 32, 0;
}

impl Timestamp {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u32; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u32; 1] {
        [self.0]
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new()
    }
}
