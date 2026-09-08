#![allow(unused_parens)]
use modular_bitfield::{bitfield, specifiers::B1};

/// Control register 3.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
        Self::from_bytes([0x04])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_match_register_bits() {
        let mut reg = Ctrl3C::new();
        reg.set_sw_reset(true);
        reg.set_if_inc(true);
        reg.set_sim(true);
        reg.set_pp_od(true);
        reg.set_h_lactive(true);
        reg.set_bdu(true);
        reg.set_boot(true);
        assert_eq!(reg.into_bytes(), [0xfd]);
        assert_eq!(Ctrl3C::default().into_bytes(), [0x04]);
    }
}
