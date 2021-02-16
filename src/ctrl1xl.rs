use core::fmt;
use embedded_hal::blocking::i2c::Write;

use crate::Register;

/// The CTRL1_XL register. Accelerometer control register 1 (r/w).
///
/// Contains the chain full-scale selection and output data rate selection and high-resolution selection
// #[derive(Debug)]
pub struct Ctrl1Xl(u8);

impl fmt::Display for Ctrl1Xl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Binary for Ctrl1Xl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::LowerHex for Ctrl1Xl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

/// Sub-address of the register.
pub const ADDR: u8 = 0x10u8;

/// Accelerometer high-resolution selection
///
/// (0: output from first stage digital filtering selected (default); 1: output from LPF2 second filtering stage selected)
const LPF2_XL_EN: u8 = 1;

const FS_MASK: u8 = 0b11;
const FS_OFFSET: u8 = 2;

/// Accelerometer full-scale selection.
///
/// Default value: 00
///
/// (00: ±2 g; 01: ±16 g; 10: ±4 g; 11: ±8 g)
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Fs_Xl {
    G2,  // ±2  g
    G16, // ±16 g
    G4,  // ±4  g
    G8,  // ±8  g
}

const ODR_XL_MASK: u8 = 0b1111;
const ODR_XL_OFFSET: u8 = 4;

/// Accelerometer ODR
///
/// Default value: `Off`
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Odr_Xl {
    Off,   // off
    Hz125, // 12.5 Hz
    Hz26,  // 26   Hz
    Hz52,  // 52   Hz
    Hz104, // 104  Hz
    Hz208, // 208  Hz
    Hz416, // 416  Hz
    Hz833, // 833  Hz
    Hz166, // 1.66 Hz
    Hz333, // 3.33 Hz
    Hz666, // 6.66 Hz
}

impl Register for Ctrl1Xl {}

impl Ctrl1Xl {
    pub fn new(bits: u8) -> Self {
        Ctrl1Xl(bits)
    }

    pub fn accelerometer_data_rate(&self) -> f32 {
        match (self.0 >> ODR_XL_OFFSET) & ODR_XL_MASK {
            0 => 0.0,
            1 => 12.5,
            2 => 26.0,
            3 => 52.0,
            4 => 104.0,
            5 => 208.0,
            6 => 416.0,
            7 => 833.0,
            8 => 1.66,
            9 => 3.33,
            10 => 6.66,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_accelerometer_data_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        value: Odr_Xl,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(ODR_XL_MASK << ODR_XL_OFFSET);
        self.0 |= (value as u8) << ODR_XL_OFFSET;
        self.write(i2c, ADDR, self.0)
    }

    pub fn chain_full_scale(&self) -> f64 {
        match (self.0 >> FS_OFFSET) & FS_MASK {
            0 => 0.061,
            1 => 0.488,
            2 => 0.122,
            3 => 0.244,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_chain_full_scale<I2C>(
        &mut self,
        i2c: &mut I2C,
        value: Fs_Xl,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(FS_MASK << FS_OFFSET);
        self.0 |= (value as u8) << FS_OFFSET;
        self.write(i2c, ADDR, self.0)
    }

    pub fn lpf2_xl_en(&mut self) -> bool {
        self.0 & (1 << LPF2_XL_EN) != 0
    }

    pub fn set_lpf2_xl_en<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= (value as u8) << LPF2_XL_EN;
        self.write(i2c, ADDR, self.0)
    }
}
