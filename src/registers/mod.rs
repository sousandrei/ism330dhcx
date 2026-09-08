//! Register modules and address definitions.

pub mod counter_bdr;
pub mod ctrl1_xl;
pub mod ctrl2_g;
pub mod ctrl3_c;
pub mod ctrl7_g;
pub mod ctrl9_xl;
pub mod fifo_ctrl;
pub mod fifo_status;
pub mod func_cfg_access;
pub mod pin_ctrl;

pub use counter_bdr::*;
pub use ctrl1_xl::*;
pub use ctrl2_g::*;
pub use ctrl3_c::*;
pub use ctrl7_g::*;
pub use ctrl9_xl::*;
pub use fifo_ctrl::*;
pub use fifo_status::*;
pub use func_cfg_access::*;
pub use pin_ctrl::*;

/// Register addresses.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    /// Enable embedded functions register
    FuncCfgAccess = 0x01,
    /// Pin control register
    PinCtrl = 0x02,
    /// Counter batch data rate register 1
    CounterBdrReg1 = 0x0B,
    /// Counter batch data rate register 2
    CounterBdrReg2 = 0x0C,
    /// FIFO control register 1
    FifoCtrl1 = 0x07,
    /// FIFO control register 2
    FifoCtrl2 = 0x08,
    /// FIFO control register 3
    FifoCtrl3 = 0x09,
    /// FIFO control register 4
    FifoCtrl4 = 0x0A,
    /// Who am I register
    WhoAmI = 0x0F,
    /// Accelerometer control register 1
    Ctrl1Xl = 0x10,
    /// Gyroscope control register 2
    Ctrl2G = 0x11,
    /// Control register 3
    Ctrl3C = 0x12,
    /// Temperature output register (low)
    OutTempL = 0x20,
    /// Temperature output register (high)
    OutTempH = 0x21,
    /// Gyroscope X-axis output register (low)
    OutXLG = 0x22,
    /// Gyroscope X-axis output register (high)
    OutXHG = 0x23,
    /// Gyroscope Y-axis output register (low)
    OutYLG = 0x24,
    /// Gyroscope Y-axis output register (high)
    OutYHG = 0x25,
    /// Gyroscope Z-axis output register (low)
    OutZLG = 0x26,
    /// Gyroscope Z-axis output register (high)
    OutZHG = 0x27,
    /// Accelerometer X-axis output register (low)
    OutXLA = 0x28,
    /// Accelerometer X-axis output register (high)
    OutXHA = 0x29,
    /// Accelerometer Y-axis output register (low)
    OutYLA = 0x2A,
    /// Accelerometer Y-axis output register (high)
    OutYHA = 0x2B,
    /// Accelerometer Z-axis output register (low)
    OutZLA = 0x2C,
    /// Accelerometer Z-axis output register (high)
    OutZHA = 0x2D,
    /// Gyroscope control register 7
    Ctrl7G = 0x16,
    /// Accelerometer control register 9
    Ctrl9Xl = 0x18,
    /// FIFO status register 1
    FifoStatus1 = 0x3A,
    /// FIFO status register 2
    FifoStatus2 = 0x3B,
    /// FIFO data output tag register
    FifoDataOutTag = 0x78,
    /// FIFO data output X-axis register (low)
    FifoDataOutXL = 0x79,
    /// FIFO data output X-axis register (high)
    FifoDataOutXH = 0x7A,
    /// FIFO data output Y-axis register (low)
    FifoDataOutYL = 0x7B,
    /// FIFO data output Y-axis register (high)
    FifoDataOutYH = 0x7C,
    /// FIFO data output Z-axis register (low)
    FifoDataOutZL = 0x7D,
    /// FIFO data output Z-axis register (high)
    FifoDataOutZH = 0x7E,
}

impl Register {
    /// Returns the register address as a u8.
    pub fn addr(self) -> u8 {
        self as u8
    }
}
