use bitfield::bitfield;

bitfield! {
    /// Activity/inactivity functions, configuration of filtering, and tap recognition functions (56h)
    pub struct TapCfg0(u8);
    impl Debug;
    /// Latched Interrupt.
    pub lir, set_lir: 0;
    /// Enable Z direction in tap recognition.
    pub tap_z_en, set_tap_z_en: 1;
    /// Enable Y direction in tap recognition.
    pub tap_y_en, set_tap_y_en: 2;
    /// Enable X direction in tap recognition.
    pub tap_x_en, set_tap_x_en: 3;
    /// HPF or SLOPE filter selection on wake-up and Activity/Inactivity functions.
    pub slope_fds, set_slope_fds: 4;
    /// Activity/inactivity interrupt mode configuration.
    pub sleep_status_on_int, set_sleep_status_on_int: 5;
    /// Immediately clearing the latched interrupts upon the read of status register.
    pub int_clr_on_read, set_int_clr_on_read: 6;
}

impl TapCfg0 {
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

impl Default for TapCfg0 {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for TapCfg0 {}
impl Clone for TapCfg0 {
    fn clone(&self) -> Self {
        *self
    }
}

/// Selection of axis priority for TAP detection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
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

impl From<u8> for TapPriority {
    fn from(val: u8) -> Self {
        match val {
            0b000 => TapPriority::XYZ,
            0b001 => TapPriority::YXZ,
            0b010 => TapPriority::XZY,
            0b011 => TapPriority::ZYX,
            0b100 => TapPriority::XYZ2,
            0b101 => TapPriority::YZX,
            0b110 => TapPriority::ZXY,
            0b111 => TapPriority::ZYX2,
            _ => TapPriority::XYZ,
        }
    }
}

impl From<TapPriority> for u8 {
    fn from(val: TapPriority) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Tap configuration register (57h)
    pub struct TapCfg1(u8);
    impl Debug;
    /// X-axis tap recognition threshold.
    pub tap_ths_x, set_tap_ths_x: 4, 0;
    /// Selection of axis priority for TAP detection.
    pub from into TapPriority, tap_priority, set_tap_priority: 7, 5;
}

impl TapCfg1 {
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

impl Default for TapCfg1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for TapCfg1 {}
impl Clone for TapCfg1 {
    fn clone(&self) -> Self {
        *self
    }
}

/// Activity/inactivity (sleep) function enable selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
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

impl From<u8> for InactEn {
    fn from(val: u8) -> Self {
        match val {
            0b00 => InactEn::None,
            0b01 => InactEn::Xl12_5,
            0b10 => InactEn::Xl12_5GyroSleep,
            0b11 => InactEn::Xl12_5GyroOff,
            _ => InactEn::None,
        }
    }
}

impl From<InactEn> for u8 {
    fn from(val: InactEn) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Enables interrupt and inactivity functions, and tap recognition functions (58h)
    pub struct TapCfg2(u8);
    impl Debug;
    /// Y-axis tap recognition threshold.
    pub tap_ths_y, set_tap_ths_y: 4, 0;
    /// Enable activity/inactivity (sleep) function.
    pub from into InactEn, inact_en, set_inact_en: 6, 5;
    /// Enable basic interrupts.
    pub interrupts_enable, set_interrupts_enable: 7;
}

impl TapCfg2 {
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

impl Default for TapCfg2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for TapCfg2 {}
impl Clone for TapCfg2 {
    fn clone(&self) -> Self {
        *self
    }
}
