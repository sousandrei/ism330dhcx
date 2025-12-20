use bitfield::bitfield;

bitfield! {
    /// Source register for all interrupts (1Ah)
    pub struct AllIntSrc(u8);
    impl Debug;
    /// Free-fall event status.
    pub ff_ia, set_ff_ia: 0;
    /// Wake-up event status.
    pub wu_ia, set_wu_ia: 1;
    /// Single-tap event status.
    pub single_tap, set_single_tap: 2;
    /// Double-tap event status.
    pub double_tap, set_double_tap: 3;
    /// 6D orientation change event status.
    pub d6d_ia, set_d6d_ia: 4;
    /// Detects change event in activity/inactivity status.
    pub sleep_change_ia, set_sleep_change_ia: 6;
    /// Alerts timestamp overflow.
    pub timestamp_endcount, set_timestamp_endcount: 7;
}

impl AllIntSrc {
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

impl Default for AllIntSrc {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for AllIntSrc {}
impl Clone for AllIntSrc {
    fn clone(&self) -> Self {
        *self
    }
}

bitfield! {
    /// Wake-up interrupt source register (1Bh)
    pub struct WakeUpSrc(u8);
    impl Debug;
    /// Wakeup event detection status on Z-axis.
    pub z_wu, set_z_wu: 0;
    /// Wakeup event detection status on Y-axis.
    pub y_wu, set_y_wu: 1;
    /// Wakeup event detection status on X-axis.
    pub x_wu, set_x_wu: 2;
    /// Wakeup event detection status.
    pub wu_ia, set_wu_ia: 3;
    /// Sleep event status.
    pub sleep_state, set_sleep_state: 4;
    /// Free-fall event detection status.
    pub ff_ia, set_ff_ia: 5;
    /// Detects change event in activity/inactivity status.
    pub sleep_change_ia, set_sleep_change_ia: 6;
}

impl WakeUpSrc {
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

impl Default for WakeUpSrc {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for WakeUpSrc {}
impl Clone for WakeUpSrc {
    fn clone(&self) -> Self {
        *self
    }
}

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

impl Copy for TapSrc {}
impl Clone for TapSrc {
    fn clone(&self) -> Self {
        *self
    }
}

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

impl Copy for D6dSrc {}
impl Clone for D6dSrc {
    fn clone(&self) -> Self {
        *self
    }
}
