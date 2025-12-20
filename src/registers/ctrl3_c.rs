use bitfield::bitfield;

bitfield! {
    /// Control register 3.
    pub struct Ctrl3C(u8);
    impl Debug;
    /// Software reset.
    pub sw_reset, set_sw_reset: 0;
    /// Register address increment enable.
    pub if_inc, set_if_inc: 2;
    /// SPI serial interface mode selection.
    pub sim, set_sim: 3;
    /// Push-pull/open-drain selection on INT1 and INT2 pads.
    pub pp_od, set_pp_od: 4;
    /// Interrupt activation level.
    pub h_lactive, set_h_lactive: 5;
    /// Block Data Update.
    pub bdu, set_bdu: 6;
    /// Reboot memory content.
    pub boot, set_boot: 7;
}

impl Ctrl3C {
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
