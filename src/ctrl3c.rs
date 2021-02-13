use core::fmt;
use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::{read, write};

/// The CTRL3_C register.
///
/// Contains memory reboot, block data update, interruct activation level, push-pull/open-drain selection on INT1 and INT2 pins
/// SPI Serial Interface Mode selection, register address automatically incrementation and software reset
#[derive(Debug)]
pub struct Ctrl3C(u8);

impl fmt::Display for Ctrl3C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Binary for Ctrl3C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::LowerHex for Ctrl3C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

/// Sub-address of the register.
pub const ADDR: u8 = 0x12;

/// Reboots memory content.
///
/// Default value: 0
///
/// (0: normal mode; 1: reboot memory content)
///
/// Note: the accelerometer must be ON. This bit is automatically cleared.
pub const BOOT: u8 = 7;

/// Block Data Update.
///
/// Default value: 0
///
/// (0: continuous update; 1: output registers are not updated until MSB and LSB have been read)
pub const BDU: u8 = 6;

/// Interrupt activation level.
///
/// Default value: 0
///
/// (0: interrupt output pins active high; 1: interrupt output pins active low
pub const H_LACTIVE: u8 = 5;

/// Push-pull/open-drain selection on INT1 and INT2 pins. This bit must be set to '0' when H_LACTIVE is set to '1'.
///
/// Default value: 0
///
/// (0: push-pull mode; 1: open-drain mode)
pub const PP_OD: u8 = 4;

/// SPI Serial Interface Mode selection.
///
/// Default value: 0
///
/// (0: 4-wire interface; 1: 3-wire interface)
pub const SIM: u8 = 3;

/// Register address automatically incremented during a multiple byte access with a serial interface (I²C or SPI).
///
/// Default value: 1
///
/// (0: disabled; 1: enabled)
pub const IF_INC: u8 = 2;

///Software reset.
///
/// Default value: 0
///
///(0: normal mode; 1: reset device)
///
///This bit is automatically cleared.
pub const SW_RESET: u8 = 0;

impl Ctrl3C {
    pub fn new<I2C>(i2c: &mut I2C) -> Result<Self, I2C::Error>
    where
        I2C: WriteRead,
    {
        let bits = read(i2c, ADDR)?;
        Ok(Ctrl3C(bits))
    }

    pub fn get(&mut self) -> u8 {
        self.0
    }

    pub fn set_boot<I2C>(&mut self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= value << BOOT;
        write(i2c, ADDR, self.0)
    }

    pub fn set_bdu<I2C>(&mut self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= value << BDU;
        write(i2c, ADDR, self.0)
    }

    pub fn sw_reset<I2C>(&mut self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= 1 << SW_RESET;
        write(i2c, ADDR, self.0)
    }

    pub fn set_if_inc<I2C>(&mut self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= value << IF_INC;
        write(i2c, ADDR, self.0)
    }
}
