use modular_bitfield::prelude::*;

/// Activity/inactivity functions, configuration of filtering, and tap recognition functions (56h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct TapCfg0 {
    /// Latched Interrupt.
    pub lir: bool,
    /// Enable Z direction in tap recognition.
    pub tap_z_en: bool,
    /// Enable Y direction in tap recognition.
    pub tap_y_en: bool,
    /// Enable X direction in tap recognition.
    pub tap_x_en: bool,
    /// HPF or SLOPE filter selection on wake-up and Activity/Inactivity functions.
    pub slope_fds: bool,
    /// Activity/inactivity interrupt mode configuration.
    pub sleep_status_on_int: bool,
    /// Immediately clearing the latched interrupts upon the read of status register.
    pub int_clr_on_read: bool,
    #[skip]
    pub __: B1,
}

impl Default for TapCfg0 {
    fn default() -> Self {
        Self::new()
    }
}

/// Selection of axis priority for TAP detection.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 3]
pub enum TapPriority {
    /// X, Y, Z
    XYZ = 0b000,
    /// Y, X, Z
    YXZ = 0b001,
    /// X, Z, Y
    XZY = 0b010,
    /// Z, Y, X
    ZYX = 0b011,
    /// X, Y, Z (same as 000)
    XYZ2 = 0b100,
    /// Y, Z, X
    YZX = 0b101,
    /// Z, X, Y
    ZXY = 0b110,
    /// Z, Y, X (same as 011)
    ZYX2 = 0b111,
}

/// Tap configuration register (57h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct TapCfg1 {
    /// X-axis tap recognition threshold.
    pub tap_ths_x: B5,
    /// Selection of axis priority for TAP detection.
    pub tap_priority: TapPriority,
}

impl Default for TapCfg1 {
    fn default() -> Self {
        Self::new()
    }
}

/// Activity/inactivity (sleep) function enable selection.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 2]
pub enum InactEn {
    /// Stationary/motion-only interrupts, XL/Gyro no change.
    None = 0b00,
    /// XL 12.5Hz (LP), Gyro no change.
    Xl12_5 = 0b01,
    /// XL 12.5Hz (LP), Gyro sleep.
    Xl12_5GyroSleep = 0b10,
    /// XL 12.5Hz (LP), Gyro power-down.
    Xl12_5GyroOff = 0b11,
}

/// Enables interrupt and inactivity functions, and tap recognition functions (58h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct TapCfg2 {
    /// Y-axis tap recognition threshold.
    pub tap_ths_y: B5,
    /// Enable activity/inactivity (sleep) function.
    pub inact_en: InactEn,
    /// Enable basic interrupts.
    pub interrupts_enable: bool,
}

impl Default for TapCfg2 {
    fn default() -> Self {
        Self::new()
    }
}
