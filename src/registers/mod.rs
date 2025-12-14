//! Register modules and address definitions.

pub mod ctrl1_xl;
pub mod ctrl2_g;
pub mod ctrl3_c;
pub mod ctrl7_g;
pub mod ctrl9_xl;
pub mod fifo_ctrl;
pub mod fifo_status;

pub use ctrl1_xl::*;
pub use ctrl2_g::*;
pub use ctrl3_c::*;
pub use ctrl7_g::*;
pub use ctrl9_xl::*;
pub use fifo_ctrl::*;
pub use fifo_status::*;

/// Register addresses.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    /// FIFO control register 1
    FifoCtrl1 = 0x07,
    /// FIFO control register 2
    FifoCtrl2 = 0x08,
    /// FIFO control register 3
    FifoCtrl3 = 0x09,
    /// FIFO control register 4
    FifoCtrl4 = 0x0A,
    /// Accelerometer control register 1
    Ctrl1Xl = 0x10,
    /// Gyroscope control register 2
    Ctrl2G = 0x11,
    /// Control register 3
    Ctrl3C = 0x12,
    /// Gyroscope control register 7
    Ctrl7G = 0x16,
    /// Accelerometer control register 9
    Ctrl9Xl = 0x18,
    /// FIFO status register 1
    FifoStatus1 = 0x3A,
    /// FIFO status register 2
    FifoStatus2 = 0x3B,
}

impl Register {
    /// Returns the register address as a u8.
    pub fn addr(self) -> u8 {
        self as u8
    }
}
