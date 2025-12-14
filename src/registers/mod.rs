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

/// Register addresses
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    FifoCtrl1 = 0x07,
    FifoCtrl2 = 0x08,
    FifoCtrl3 = 0x09,
    FifoCtrl4 = 0x0A,
    Ctrl1Xl = 0x10,
    Ctrl2G = 0x11,
    Ctrl3C = 0x12,
    Ctrl7G = 0x16,
    Ctrl9Xl = 0x18,
    FifoStatus1 = 0x3A,
    FifoStatus2 = 0x3B,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}
