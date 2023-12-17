use core::fmt;
use embedded_hal::blocking::i2c::Write;

use crate::Register;

// Together with sensor data the device can provide timestamp information.
// To enable this functionality the TIMESTAMP_EN bit of the CTRL10_C register
// has to be set to 1.

pub struct Ctrl10C {
    pub address: u8,
    value: u8,
}

impl fmt::Display for Ctrl10C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Binary for Ctrl10C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.value)
    }
}

impl fmt::LowerHex for Ctrl10C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.value, f)
    }
}


/// Sub-address of the register.
pub const ADDR: u8 = 0x19u8;


pub const TIMESTAMP_EN: u8 = 5;


impl Register for Ctrl10C {}

impl Ctrl10C {
    pub fn new(value: u8, address: u8) -> Self {
        Ctrl2G { address, value }
    }
