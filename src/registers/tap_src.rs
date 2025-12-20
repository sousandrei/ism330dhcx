use bitfield::bitfield;

bitfield! {
    /// Tap source register (1Ch)
    pub struct TapSrc(u8);
    impl Debug;
    /// Tap event detection status on Z-axis.
    pub z_tap, set_z_tap: 0;
    /// Tap event detection status on Y-axis.
    pub y_tap, set_y_tap: 1;
    /// Tap event detection status on X-axis.
    pub x_tap, set_x_tap: 2;
    /// Sign of acceleration detected by tap event.
    pub tap_sign, set_tap_sign: 3;
    /// Double-tap event detection status.
    pub double_tap, set_double_tap: 4;
    /// Single-tap event status.
    pub single_tap, set_single_tap: 5;
    /// Tap event detection status.
    pub tap_ia, set_tap_ia: 6;
}

impl TapSrc {
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

impl Default for TapSrc {
    fn default() -> Self {
        Self::new()
    }
}
