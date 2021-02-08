use core::fmt;

use embedded_hal::blocking::i2c::{Write, WriteRead};

use super::sensor::{write, write_read, Sensor};

/// The CTRL3_C register.
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

/// Reboots memory content. Default value: 0
///
/// (0: normal mode; 1: reboot memory content)
///
/// Note: the accelerometer must be ON. This bit is automatically cleared.
pub const BOOT: u8 = 7;

/// Block Data Update. Default value: 0
///
/// (0: continuous update; 1: output registers are not updated until MSB and LSB have been read)
pub const BDU: u8 = 6;

/// Interrupt activation level. Default value: 0
///
/// (0: interrupt output pins active high; 1: interrupt output pins active low
pub const H_LACTIVE: u8 = 5;

/// Push-pull/open-drain selection on INT1 and INT2 pins. This bit must be set to '0' when H_LACTIVE is set to '1'.
///
/// Default value: 0
///
/// (0: push-pull mode; 1: open-drain mode)
pub const PP_OD: u8 = 4;

/// SPI Serial Interface Mode selection. Default value: 0
///
/// (0: 4-wire interface; 1: 3-wire interface)
pub const SIM: u8 = 3;

/// Register address automatically incremented during a multiple byte access with a serial interface (I²C or SPI).
///
/// Default value: 1
///
/// (0: disabled; 1: enabled)
pub const IF_INC: u8 = 2;

///Software reset. Default value: 0
///
///(0: normal mode; 1: reset device)
///
///This bit is automatically cleared.
pub const SW_RESET: u8 = 0;

impl Ctrl3C {
    /// Blocking read of the CTRL3_C register from `Sensor`.
    pub fn new<Comm>(sensor: &mut Sensor<Comm>) -> Result<Self, Comm::Error>
    where
        Comm: WriteRead,
    {
        let bits = write_read(sensor, ADDR)?;
        Ok(Ctrl3C(bits))
    }

    /// Updates the register using `f`, then writes the new value out to the chip.
    pub fn modify<Comm>(&mut self, sensor: &mut Sensor<Comm>) -> Result<(), Comm::Error>
    where
        Comm: Write,
    {
        write(sensor, ADDR, self.0)
    }

    /// Returns true if the chip is active.
    pub fn is_powered_up(&self) -> bool {
        (self.0 & BOOT) > 0
    }

    /// Clears the power-down bit. The Sensor is in power-down mode when BOOT = 0.
    pub fn power_down(&mut self) {
        self.0 &= !(1 << BOOT);
    }

    /// Sets the power-down bit. The Sensor is active when BOOT = 1.
    pub fn power_up(&mut self) {
        self.0 |= 1 << BOOT;
    }

    /// Returns true if the chip is using block-update mode.
    pub fn is_block_update(&self) -> bool {
        (self.0 & BDU) > 0
    }

    /// Clears the block-update mode bit. In default (continuous) mode, the lower and upper parts
    /// of the output registers are updated continuously. If it is not certain whether the read will
    /// be faster than output data rate, it is recommended to use block-update mode.
    pub fn set_continuous_update(&mut self) {
        self.0 &= !(1 << BDU);
    }

    /// Sets the block-update mode bit. In block-update mode, after the reading of the lower
    /// (upper) register part, the content of that output register is not updated until the upper
    /// (lower) part is read also. This feature prevents the reading of LSB and MSB related to
    /// different samples.
    pub fn set_block_update(&mut self) {
        self.0 |= 1 << BDU;
    }
}
