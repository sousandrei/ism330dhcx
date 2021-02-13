use core::fmt;

use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::{write, write_read};

/// The CTRL2_G register. Gyroscope control register 2.
///
/// Contains the chain full-scale selection and output data rate selection
// #[derive(Debug)]
pub struct Ctrl2G(u8);

impl fmt::Display for Ctrl2G {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Binary for Ctrl2G {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::LowerHex for Ctrl2G {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

/// Sub-address of the register.
pub const ADDR: u8 = 0x11u8;

///Selects gyro chain full-scale ±4000 dps
///
///(0: FS selected through bits FS\[1:0\]_G or FS_125; 1: FS set to ±4000 dps)
pub const FS4000: u8 = 0;

///Selects gyro chain full-scale ±125 dps
///
///(0: FS selected through bits FS\[1:0\]_G; 1: FS set to ±125 dps)
pub const FS125: u8 = 1;

const FS_MASK: u8 = 0b11;
const FS_OFFSET: u8 = 2;

/// Gyroscope chain full-scale selection in dps
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FS {
    Dps250,  // ±250 dps
    Dps500,  // ±500 dps
    Dps1000, // ±1000 dps
    Dps2000, // ±2000 dps
    Dps125,  // ±125 dps
    Dps4000, // ±4000 dps
}

const ODR_MASK: u8 = 0b1111;
const ODR_OFFSET: u8 = 4;

/// Gyroscope ODR selection
///
/// Default value: `Off`
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ODR {
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

impl Ctrl2G {
    pub fn new<I2C>(i2c: &mut I2C) -> Result<Self, I2C::Error>
    where
        I2C: WriteRead,
    {
        let bits = write_read(i2c, ADDR)?;
        let register = Ctrl2G(bits);

        Ok(register)
    }

    pub fn gyroscope_data_rate(&self) -> ODR {
        match (self.0 >> ODR_OFFSET) & ODR_MASK {
            0 => ODR::Off,
            1 => ODR::Hz125,
            2 => ODR::Hz26,
            3 => ODR::Hz52,
            4 => ODR::Hz104,
            5 => ODR::Hz208,
            6 => ODR::Hz416,
            7 => ODR::Hz833,
            8 => ODR::Hz166,
            9 => ODR::Hz333,
            10 => ODR::Hz666,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_gyroscope_data_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        value: ODR,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.0 &= !(ODR_MASK << ODR_OFFSET);
        self.0 |= (value as u8) << ODR_OFFSET;
        write(i2c, ADDR, self.0)
    }

    pub fn chain_full_scale(&self) -> FS {
        if (self.0 & FS4000) > 0 {
            return FS::Dps4000;
        }

        if (self.0 & FS125) > 0 {
            return FS::Dps125;
        }

        match (self.0 >> FS_OFFSET) & FS_MASK {
            0 => FS::Dps250,
            1 => FS::Dps500,
            2 => FS::Dps1000,
            3 => FS::Dps2000,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_chain_full_scale<I2C>(&mut self, i2c: &mut I2C, value: FS) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        if value == FS::Dps4000 {
            self.0 |= (value as u8) << FS4000;
        } else if value == FS::Dps125 {
            self.0 |= (value as u8) << FS125;
        } else {
            self.0 &= !(FS_MASK << FS_OFFSET);
            self.0 |= (value as u8) << FS_OFFSET;
        }

        write(i2c, ADDR, self.0)
    }
}
