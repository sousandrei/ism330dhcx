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

use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;

/// Configuration methods for CTRL3_C register.
pub trait Ctrl3CConfig {
    /// Reboot memory content.
    fn set_boot<I2C>(&mut self, i2c: &mut I2C, boot: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Block Data Update.
    ///
    /// If true, output registers are not updated until MSB and LSB have been read.
    fn set_bdu<I2C>(&self, i2c: &mut I2C, bdu: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Register address automatically incremented during a multiple byte access with a serial interface.
    fn set_if_inc<I2C>(&self, i2c: &mut I2C, if_inc: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ctrl3CConfig for Ism330Dhcx {
    fn set_boot<I2C>(&mut self, i2c: &mut I2C, boot: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_boot(boot);
            reg.into_bytes()[0]
        })
    }

    fn set_bdu<I2C>(&self, i2c: &mut I2C, bdu: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_bdu(bdu);
            reg.into_bytes()[0]
        })
    }

    fn set_if_inc<I2C>(&self, i2c: &mut I2C, if_inc: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_if_inc(if_inc);
            reg.into_bytes()[0]
        })
    }
}
