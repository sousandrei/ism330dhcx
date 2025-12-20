use modular_bitfield::prelude::*;

/// Functions routing on INT1 register (5Eh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Md1Cfg {
    /// Routing of sensor hub communication concluded event on INT1.
    pub int1_shub: bool,
    /// Routing of embedded functions event on INT1.
    pub int1_emb_func: bool,
    /// Routing of 6D event on INT1.
    pub int1_6d: bool,
    /// Routing of double-tap event on INT1.
    pub int1_double_tap: bool,
    /// Routing of free-fall event on INT1.
    pub int1_ff: bool,
    /// Routing of wakeup event on INT1.
    pub int1_wu: bool,
    /// Routing of single-tap recognition event on INT1.
    pub int1_single_tap: bool,
    /// Routing of activity/inactivity recognition event on INT1.
    pub int1_sleep_change: bool,
}

impl Default for Md1Cfg {
    fn default() -> Self {
        Self::new()
    }
}

/// Functions routing on INT2 register (5Fh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Md2Cfg {
    /// Enables routing on INT2 pin of the alert for timestamp overflow.
    pub int2_timestamp: bool,
    /// Routing of embedded functions event on INT2.
    pub int2_emb_func: bool,
    /// Routing of 6D event on INT2.
    pub int2_6d: bool,
    /// Routing of double-tap event on INT2.
    pub int2_double_tap: bool,
    /// Routing of free-fall event on INT2.
    pub int2_ff: bool,
    /// Routing of wakeup event on INT2.
    pub int2_wu: bool,
    /// Single-tap recognition routing on INT2.
    pub int2_single_tap: bool,
    /// Routing of activity/inactivity recognition event on INT2.
    pub int2_sleep_change: bool,
}

impl Default for Md2Cfg {
    fn default() -> Self {
        Self::new()
    }
}
