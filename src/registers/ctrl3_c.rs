#![allow(unused_parens)]
use modular_bitfield::{bitfield, specifiers::B1};

/// Control register 3.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl3C {
    /// Software reset.
    pub sw_reset: bool,
    #[skip]
    pub __: B1,
    /// Register address increment enable.
    pub if_inc: bool,
    /// SPI serial interface mode selection.
    pub sim: bool,
    /// Push-pull/open-drain selection on INT1 and INT2 pads.
    pub pp_od: bool,
    /// Interrupt activation level.
    pub h_lactive: bool,
    /// Block Data Update.
    pub bdu: bool,
    /// Reboot memory content.
    pub boot: bool,
}

impl Default for Ctrl3C {
    fn default() -> Self {
        Self::new()
    }
}
