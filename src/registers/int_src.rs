use modular_bitfield::prelude::*;

/// Source register for all interrupts (1Ah)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct AllIntSrc {
    /// Free-fall event status.
    pub ff_ia: bool,
    /// Wake-up event status.
    pub wu_ia: bool,
    /// Single-tap event status.
    pub single_tap: bool,
    /// Double-tap event status.
    pub double_tap: bool,
    /// 6D orientation change event status.
    pub d6d_ia: bool,
    #[skip]
    pub __: B1,
    /// Detects change event in activity/inactivity status.
    pub sleep_change_ia: bool,
    /// Alerts timestamp overflow.
    pub timestamp_endcount: bool,
}

impl Default for AllIntSrc {
    fn default() -> Self {
        Self::new()
    }
}

/// Wake-up interrupt source register (1Bh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct WakeUpSrc {
    /// Wakeup event detection status on Z-axis.
    pub z_wu: bool,
    /// Wakeup event detection status on Y-axis.
    pub y_wu: bool,
    /// Wakeup event detection status on X-axis.
    pub x_wu: bool,
    /// Wakeup event detection status.
    pub wu_ia: bool,
    /// Sleep event status.
    pub sleep_state: bool,
    /// Free-fall event detection status.
    pub ff_ia: bool,
    /// Detects change event in activity/inactivity status.
    pub sleep_change_ia: bool,
    #[skip]
    pub __: B1,
}

impl Default for WakeUpSrc {
    fn default() -> Self {
        Self::new()
    }
}

/// Tap source register (1Ch)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct TapSrc {
    /// Tap event detection status on Z-axis.
    pub z_tap: bool,
    /// Tap event detection status on Y-axis.
    pub y_tap: bool,
    /// Tap event detection status on X-axis.
    pub x_tap: bool,
    /// Sign of acceleration detected by tap event.
    pub tap_sign: bool,
    /// Double-tap event detection status.
    pub double_tap: bool,
    /// Single-tap event status.
    pub single_tap: bool,
    /// Tap event detection status.
    pub tap_ia: bool,
    #[skip]
    pub __: B1,
}

impl Default for TapSrc {
    fn default() -> Self {
        Self::new()
    }
}

/// Portrait, landscape, face-up and face-down source register (1Dh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct D6dSrc {
    /// X-axis low event.
    pub xl: bool,
    /// X-axis high event.
    pub xh: bool,
    /// Y-axis low event.
    pub yl: bool,
    /// Y-axis high event.
    pub yh: bool,
    /// Z-axis low event.
    pub zl: bool,
    /// Z-axis high event.
    pub zh: bool,
    /// 6D orientation change event status.
    pub d6d_ia: bool,
    /// DEN data-ready signal.
    pub den_drdy: bool,
}

impl Default for D6dSrc {
    fn default() -> Self {
        Self::new()
    }
}
