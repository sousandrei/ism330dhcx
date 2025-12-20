use bitfield::bitfield;

bitfield! {
    /// Portrait, landscape, face-up and face-down source register (1Dh)
    pub struct D6dSrc(u8);
    impl Debug;
    /// X-axis low event.
    pub xl, set_xl: 0;
    /// X-axis high event.
    pub xh, set_xh: 1;
    /// Y-axis low event.
    pub yl, set_yl: 2;
    /// Y-axis high event.
    pub yh, set_yh: 3;
    /// Z-axis low event.
    pub zl, set_zl: 4;
    /// Z-axis high event.
    pub zh, set_zh: 5;
    /// 6D orientation change event status.
    pub d6d_ia, set_d6d_ia: 6;
    /// DEN data-ready signal.
    pub den_drdy, set_den_drdy: 7;
}

impl D6dSrc {
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

impl Default for D6dSrc {
    fn default() -> Self {
        Self::new()
    }
}
