use core::fmt;

use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::{write, write_read};

/// The CTRL7_G register. Control register 7.
///
/// Contains high-performance operating mode for gyroscope,
/// gyroscope digital high-pass filter, gyroscope digital HP filter cutoff selection,
/// enabling and disabling the OIS chain and accelerometer user offset correction block
// #[derive(Debug)]
pub struct Ctrl7G(u8);

impl fmt::Display for Ctrl7G {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Binary for Ctrl7G {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::LowerHex for Ctrl7G {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

/// Sub-address of the register.
pub const ADDR: u8 = 0x16u8;

/// Disables high-performance operating mode for gyroscope. Default: 0
///
///(0: high-performance operating mode enabled; 1: high-performance operating mode disabled)
pub const G_HM_MODE: u8 = 7;

/// Enables gyroscope digital high-pass filter. The filter is enabled only if the gyro is in HP mode. Default value: 0
///
///(0: HPF disabled; 1: HPF enabled)
pub const HP_EN_G: u8 = 6;

const HPM_G_MASK: u8 = 0b11;
const HPM_G_OFFSET: u8 = 3;
/// Gyroscope digital HP filter cutoff selection. Default: 00
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HPM_G {
    HPMG16,  // ±250 mHz
    HPMG65,  // ±500 mHz
    HPMG260, // ±1000 mHz
    HPMG104, // ±4000 Hz
}

/// Selects how to enable and disable the OIS chain, after first configuration and enabling through SPI2.
///
///(0: OIS chain is enabled/disabled with SPI2 interface; 1: OIS chain is enabled/disabled with primary interface)
pub const OIS_ON_EN: u8 = 2;

/// Enables accelerometer user offset correction block; it's valid for the low-pass path - see Figure 16. Accelerometer
///
/// composite filter. Default value: 0
/// (0: accelerometer user offset correction block bypassed; 1: accelerometer user offset correction block enabled)
pub const USR_OFF_ON_OUT: u8 = 1;

/// Enables/disables the OIS chain from primary interface when the OIS_ON_EN bit is '1'.
///
/// (0: OIS disabled; 1: OIS enabled)
pub const OIS_ON: u8 = 0;

impl Ctrl7G {
    pub fn new<I2C>(i2c: &mut I2C) -> Result<Self, I2C::Error>
    where
        I2C: WriteRead,
    {
        let bits = write_read(i2c, ADDR)?;
        let register = Ctrl7G(bits);

        Ok(register)
    }

    pub fn hpm_g(&self) -> HPM_G {
        match (self.0 >> HPM_G_OFFSET) & HPM_G_MASK {
            0 => HPM_G::HPMG16,
            1 => HPM_G::HPMG65,
            2 => HPM_G::HPMG260,
            3 => HPM_G::HPMG104,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_hpm_g<I2C>(&mut self, i2c: &mut I2C, value: HPM_G) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(HPM_G_MASK << HPM_G_OFFSET);
        self.0 |= (value as u8) << HPM_G_OFFSET;
        write(i2c, ADDR, self.0)
    }

    pub fn set_g_hm_mode<I2C>(&mut self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= value << G_HM_MODE;
        write(i2c, ADDR, self.0)
    }
}
