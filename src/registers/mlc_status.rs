use bitfield::bitfield;

bitfield! {
    /// Machine Learning Core status register (38h)
    pub struct MlcStatusMainpage(u8);
    impl Debug;
    pub is_mlc1, set_is_mlc1: 0;
    pub is_mlc2, set_is_mlc2: 1;
    pub is_mlc3, set_is_mlc3: 2;
    pub is_mlc4, set_is_mlc4: 3;
    pub is_mlc5, set_is_mlc5: 4;
    pub is_mlc6, set_is_mlc6: 5;
    pub is_mlc7, set_is_mlc7: 6;
    pub is_mlc8, set_is_mlc8: 7;
}

impl MlcStatusMainpage {
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

impl Default for MlcStatusMainpage {
    fn default() -> Self {
        Self::new()
    }
}
