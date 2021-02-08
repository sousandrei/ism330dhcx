use core::fmt;

use embedded_hal::blocking::i2c::{Write, WriteRead};

use super::sensor::{write, write_read, Sensor};

/// The CTRL2_G register. Contains power on, block data update (bdu), interrupt activation level
/// push-pull/open-drain configuration, spi serial interface mode and reset.
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

pub const FS_4000: u8 = 3;

pub const FS_MASK: u8 = 0b1111;
pub const FS_OFFSET: u8 = 0;
pub enum FS {
    Fs250 = 0,  // ±250 dps
    Fs500 = 1,  // ±500 dps
    Fs1000 = 2, // ±1000 dps
    Fs2000 = 3, // ±2000 dps
}

const ODR_MASK: u8 = 0b1111;
const ODR_OFFSET: u8 = 4;

/// Gyroscope output data rate selection
///
/// Default value: 0000 (off)
pub enum ODR {
    /// off
    Off = 0,
    /// 12.5 Hz
    Hz125 = 1,
    /// 26   Hz
    Hz26 = 2,
    /// 52   Hz
    Hz52 = 3,
    /// 104  Hz
    Hz104 = 4,
    /// 208  Hz
    Hz208 = 5,
    /// 416  Hz
    Hz416 = 6,
    /// 833  Hz
    Hz833 = 7,
    /// 1.66 Hz
    Hz166 = 8,
    /// 3.33 Hz
    Hz333 = 9,
    /// 6.66 Hz
    Hz666 = 10,
}

impl Ctrl2G {
    pub fn new<Comm>(sensor: &mut Sensor<Comm>) -> Result<Self, Comm::Error>
    where
        Comm: WriteRead,
    {
        let bits = write_read(sensor, ADDR)?;
        let register = Ctrl2G(bits);

        Ok(register)
    }

    pub fn modify<Comm>(&mut self, sensor: &mut Sensor<Comm>) -> Result<(), Comm::Error>
    where
        Comm: Write,
    {
        write(sensor, ADDR, self.0)
    }

    pub fn gyroscope_data_rate(&self) -> u16 {
        match (self.0 >> ODR_OFFSET) & ODR_MASK {
            0 => 0,   // off
            1 => 1,   // 12.5 Hz
            2 => 2,   // 26   Hz
            3 => 3,   // 52   Hz
            4 => 4,   // 104  Hz
            5 => 5,   // 208  Hz
            6 => 6,   // 416  Hz
            7 => 7,   // 833  Hz
            8 => 8,   // 1.66 Hz
            9 => 9,   // 3.33 Hz
            10 => 10, // 6.66 Hz
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_gyroscope_data_rate(&mut self, samples: ODR) {
        self.0 &= !(ODR_MASK << ODR_OFFSET);
        self.0 |= (samples as u8) << ODR_OFFSET;
    }

    pub fn chain_full_scale(&self) -> u16 {
        match (self.0 >> FS_OFFSET) & FS_MASK {
            0 => 0, // ±250 dps
            1 => 1, // ±500 dps
            2 => 2, // ±1000 dps
            3 => 3, // ±2000 dps
            _ => panic!("Unreachable"),
        }
    }

    pub fn set_chain_full_scale(&mut self, samples: FS) {
        self.0 &= !(FS_MASK << FS_OFFSET);
        self.0 |= (samples as u8) << FS_OFFSET;
    }
}
