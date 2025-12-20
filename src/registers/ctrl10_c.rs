use bitfield::bitfield;

bitfield! {
    /// Control register 10 (19h)
    pub struct Ctrl10C(u8);
    impl Debug;
    /// Enables timestamp counter.
    pub timestamp_en, set_timestamp_en: 2;
}

impl Ctrl10C {
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

impl Default for Ctrl10C {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Ctrl10C {}
impl Clone for Ctrl10C {
    fn clone(&self) -> Self {
        *self
    }
}
