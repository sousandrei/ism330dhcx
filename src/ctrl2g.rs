use core::fmt;
use embedded_hal::blocking::i2c::Write;

use crate::Register;

/// The CTRL2_G register. Gyroscope control register 2.
///
/// Contains the chain full-scale selection and output data rate selection
pub struct Ctrl2G {
    pub address: u8,
    value: u8,
}

impl fmt::Display for Ctrl2G {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Binary for Ctrl2G {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.value)
    }
}

impl fmt::LowerHex for Ctrl2G {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.value, f)
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Fs {
    Dps250,  // ±250 dps
    Dps500,  // ±500 dps
    Dps1000, // ±1000 dps
    Dps2000, // ±2000 dps
    Dps4000, // ±4000 dps
    Dps125,  // ±125 dps
}

const ODR_MASK: u8 = 0b1111;
const ODR_OFFSET: u8 = 4;

/// Gyroscope ODR selection
///
/// Default value: `Off`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Odr {
    Off,    // off
    Hz125,  // 12.5 Hz
    Hz26,   // 26   Hz
    Hz52,   // 52   Hz
    Hz104,  // 104  Hz
    Hz208,  // 208  Hz
    Hz416,  // 416  Hz
    Hz833,  // 833  Hz
    KHz166, // 1.66 kHz
    KHz333, // 3.33 kHz
    KHz666, // 6.66 kHz
}

impl Register for Ctrl2G {}

impl Ctrl2G {
    pub fn new(value: u8, address: u8) -> Self {
        Ctrl2G { address, value }
    }

    pub fn gyroscope_data_rate(&self) -> f32 {
        match (self.value >> ODR_OFFSET) & ODR_MASK {
            0 => 0.0,
            1 => 12.5,
            2 => 26.0,
            3 => 52.2,
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

    pub fn set_gyroscope_data_rate<I2C>(
        &mut self,
        i2c: &mut I2C,
        value: Odr,
    ) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.value &= !(ODR_MASK << ODR_OFFSET);
        self.value |= (value as u8) << ODR_OFFSET;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn chain_full_scale(&self) -> f64 {
        if (self.value & 1 << FS4000) > 0 {
            return 4000.0;
        }

        if (self.value & 1 << FS125) > 0 {
            return 125.0;
        }

        match (self.value >> FS_OFFSET) & FS_MASK {
            0 => 250.0,
            1 => 500.0,
            2 => 1000.0,
            3 => 2000.0,
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_chain_full_scale<I2C>(&mut self, i2c: &mut I2C, value: Fs) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.value &= 0b1111_0000;

        if value == Fs::Dps4000 {
            self.value |= 1;
        } else if value == Fs::Dps125 {
            self.value |= 2;
        } else {
            self.value |= (value as u8) << FS_OFFSET;
        }

        self.write(i2c, self.address, ADDR, self.value)
    }
}
