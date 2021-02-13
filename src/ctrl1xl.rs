use core::fmt;

use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::{write, write_read};

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
pub const LPF2_XL_EN: u8 = 1;

const FS_MASK: u8 = 0b11;
const FS_OFFSET: u8 = 2;

/// Accelerometer full-scale selection. Default value: 00
///
/// (00: ±2 g; 01: ±16 g; 10: ±4 g; 11: ±8 g)
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FS_XL {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ODR_XL {
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

impl Ctrl1Xl {
    pub fn new<I2C>(i2c: &mut I2C) -> Result<Self, I2C::Error>
    where
        I2C: WriteRead,
    {
        let bits = write_read(i2c, ADDR)?;
        let register = Ctrl1Xl(bits);

        Ok(register)
    }

    pub fn accelerometer_data_rate(&self) -> ODR_XL {
        match (self.0 >> ODR_XL_OFFSET) & ODR_XL_MASK {
            0 => ODR_XL::Off,
            1 => ODR_XL::Hz125,
            2 => ODR_XL::Hz26,
            3 => ODR_XL::Hz52,
            4 => ODR_XL::Hz104,
            5 => ODR_XL::Hz208,
            6 => ODR_XL::Hz416,
            7 => ODR_XL::Hz833,
            8 => ODR_XL::Hz166,
            9 => ODR_XL::Hz333,
            10 => ODR_XL::Hz666,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_accelerometer_data_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        value: ODR_XL,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(ODR_XL_MASK << ODR_XL_OFFSET);
        self.0 |= (value as u8) << ODR_XL_OFFSET;
        write(i2c, ADDR, self.0)
    }

    pub fn chain_full_scale(&self) -> FS_XL {
        match (self.0 >> FS_OFFSET) & FS_MASK {
            0 => FS_XL::G2,
            1 => FS_XL::G16,
            2 => FS_XL::G8,
            3 => FS_XL::G4,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_chain_full_scale<I2C>(
        &mut self,
        i2c: &mut I2C,
        value: FS_XL,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(FS_MASK << FS_OFFSET);
        self.0 |= (value as u8) << FS_OFFSET;
        write(i2c, ADDR, self.0)
    }

    pub fn set_lpf2_xl_en<I2C>(&mut self, i2c: &mut I2C, value: u8) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 |= value << LPF2_XL_EN;
        write(i2c, ADDR, self.0)
    }
}
