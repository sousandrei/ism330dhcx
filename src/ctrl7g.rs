use core::fmt;
use embedded_hal::blocking::i2c::Write;

use crate::Register;

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

/// Disables high-performance operating mode for gyroscope.
///
/// Default: 0
///
///(0: high-performance operating mode enabled; 1: high-performance operating mode disabled)
pub const G_HM_MODE: u8 = 7;

/// Enables gyroscope digital high-pass filter. The filter is enabled only if the gyro is in HP mode.
///
/// Default value: 0
///
///(0: HPF disabled; 1: HPF enabled)
pub const HP_EN_G: u8 = 6;

const HPM_G_MASK: u8 = 0b11;
const HPM_G_OFFSET: u8 = 3;
/// Gyroscope digital HP filter cutoff selection.
///
/// Default: 00
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Hpm_g {
    Hpmg16,  // ±250 mHz
    Hpmg65,  // ±500 mHz
    Hpmg260, // ±1000 mHz
    Hpmg104, // ±4000 Hz
}

/// Selects how to enable and disable the OIS chain, after first configuration and enabling through SPI2.
///
///(0: OIS chain is enabled/disabled with SPI2 interface; 1: OIS chain is enabled/disabled with primary interface)
pub const OIS_ON_EN: u8 = 2;

/// Enables accelerometer user offset correction block; it's valid for the low-pass path - see Figure 16. Accelerometer
///
/// composite filter.
///
/// Default value: 0
/// (0: accelerometer user offset correction block bypassed; 1: accelerometer user offset correction block enabled)
pub const USR_OFF_ON_OUT: u8 = 1;

/// Enables/disables the OIS chain from primary interface when the OIS_ON_EN bit is '1'.
///
/// (0: OIS disabled; 1: OIS enabled)
pub const OIS_ON: u8 = 0;

impl Register for Ctrl7G {}

impl Ctrl7G {
    pub fn new(bits: u8) -> Self {
        Ctrl7G(bits)
    }

    pub fn hpm_g(&self) -> f32 {
        match (self.0 >> HPM_G_OFFSET) & HPM_G_MASK {
            0 => 250.0,
            1 => 500.0,
            2 => 1000.0,
            3 => 4000.0,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_hpm_g<I2C>(&mut self, i2c: &mut I2C, value: Hpm_g) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(HPM_G_MASK << HPM_G_OFFSET);
        self.0 |= (value as u8) << HPM_G_OFFSET;
        self.write(i2c, ADDR, self.0)
    }

    pub fn g_hm_mode(&mut self) -> bool {
        self.0 & (1 << G_HM_MODE) != 0
    }

    pub fn set_g_hm_mode<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << G_HM_MODE);
        self.0 |= (value as u8) << G_HM_MODE;
        self.write(i2c, ADDR, self.0)
    }

    pub fn ois_on_en(&mut self) -> bool {
        self.0 & (1 << OIS_ON_EN) != 0
    }

    pub fn set_ois_on_en<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << OIS_ON_EN);
        self.0 |= (value as u8) << OIS_ON_EN;
        self.write(i2c, ADDR, self.0)
    }

    pub fn usr_off_on_out(&mut self) -> bool {
        self.0 & (1 << USR_OFF_ON_OUT) != 0
    }

    pub fn set_usr_off_on_out<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << USR_OFF_ON_OUT);
        self.0 |= (value as u8) << USR_OFF_ON_OUT;
        self.write(i2c, ADDR, self.0)
    }

    pub fn ois_on(&mut self) -> bool {
        self.0 & (1 << OIS_ON) != 0
    }

    pub fn set_ois_on<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(1 << OIS_ON);
        self.0 |= (value as u8) << OIS_ON;
        self.write(i2c, ADDR, self.0)
    }
}
