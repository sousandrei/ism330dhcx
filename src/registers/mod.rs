//! Register modules and address definitions.

pub mod all_int_src;
pub mod counter_bdr;
pub mod ctrl10_c;
pub mod ctrl1_xl;
pub mod ctrl2_g;
pub mod ctrl3_c;
pub mod ctrl4_c;
pub mod ctrl5_c;
pub mod ctrl6_c;
pub mod ctrl7_g;
pub mod ctrl8_xl;
pub mod ctrl9_xl;
pub mod d6d_src;
pub mod fifo_ctrl;
pub mod fifo_status;
pub mod free_fall;
pub mod func_cfg_access;
pub mod int_ctrl;
pub mod int_dur2;
pub mod md_cfg;
pub mod ofs_usr;
pub mod pin_ctrl;
pub mod status_reg;
pub mod tap_cfg;
pub mod tap_src;
pub mod tap_ths_6d;
pub mod timestamp;
pub mod wake_up_dur;
pub mod wake_up_src;
pub mod wake_up_ths;

pub use all_int_src::*;
pub use counter_bdr::*;
pub use ctrl10_c::*;
pub use ctrl1_xl::*;
pub use ctrl2_g::*;
pub use ctrl3_c::*;
pub use ctrl4_c::*;
pub use ctrl5_c::*;
pub use ctrl6_c::*;
pub use ctrl7_g::*;
pub use ctrl8_xl::*;
pub use ctrl9_xl::*;
pub use d6d_src::*;
pub use fifo_ctrl::*;
pub use fifo_status::*;
pub use free_fall::*;
pub use func_cfg_access::*;
pub use int_ctrl::*;
pub use int_dur2::*;
pub use md_cfg::*;
pub use ofs_usr::*;
pub use pin_ctrl::*;
pub use status_reg::*;
pub use tap_cfg::*;
pub use tap_src::*;
pub use tap_ths_6d::*;
pub use timestamp::*;
pub use wake_up_dur::*;
pub use wake_up_src::*;
pub use wake_up_ths::*;

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
    /// Interrupt routing register 1
    Int1Ctrl = 0x0D,
    /// Interrupt routing register 2
    Int2Ctrl = 0x0E,
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
    /// Control register 4
    Ctrl4C = 0x13,
    /// Control register 5
    Ctrl5C = 0x14,
    /// Control register 6
    Ctrl6C = 0x15,
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
    /// Accelerometer control register 8
    Ctrl8Xl = 0x17,
    /// Accelerometer control register 9
    Ctrl9Xl = 0x18,
    /// Control register 10
    Ctrl10C = 0x19,
    /// All interrupt source register
    AllIntSrc = 0x1A,
    /// Wake-up source register
    WakeUpSrc = 0x1B,
    /// Tap source register
    TapSrc = 0x1C,
    /// 6D source register
    D6dSrc = 0x1D,
    /// Status register
    StatusReg = 0x1E,
    /// Timestamp register 0
    Timestamp0 = 0x40,
    /// Timestamp register 1
    Timestamp1 = 0x41,
    /// Timestamp register 2
    Timestamp2 = 0x42,
    /// Timestamp register 3
    Timestamp3 = 0x43,
    /// Tap configuration register 0
    TapCfg0 = 0x56,
    /// Tap configuration register 1
    TapCfg1 = 0x57,
    /// Tap configuration register 2
    TapCfg2 = 0x58,
    /// Tap threshold and 6D configuration register
    TapThs6d = 0x59,
    /// Interrupt duration register 2
    IntDur2 = 0x5A,
    /// Wake-up threshold register
    WakeUpThs = 0x5B,
    /// Wake-up duration register
    WakeUpDur = 0x5C,
    /// Free-fall configuration register
    FreeFall = 0x5D,
    /// Interrupt routing register 1 for embedded events
    Md1Cfg = 0x5E,
    /// Interrupt routing register 2 for embedded events
    Md2Cfg = 0x5F,
    /// User X-axis offset register
    XOfsUsr = 0x73,
    /// User Y-axis offset register
    YOfsUsr = 0x74,
    /// User Z-axis offset register
    ZOfsUsr = 0x75,
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

#[cfg(test)]
mod tests {
    use super::Register;

    #[test]
    fn register_addresses_match_datasheet() {
        let addresses = [
            (Register::FuncCfgAccess, 0x01),
            (Register::PinCtrl, 0x02),
            (Register::FifoCtrl1, 0x07),
            (Register::FifoCtrl2, 0x08),
            (Register::FifoCtrl3, 0x09),
            (Register::FifoCtrl4, 0x0A),
            (Register::CounterBdrReg1, 0x0B),
            (Register::CounterBdrReg2, 0x0C),
            (Register::Int1Ctrl, 0x0D),
            (Register::Int2Ctrl, 0x0E),
            (Register::WhoAmI, 0x0F),
            (Register::Ctrl1Xl, 0x10),
            (Register::Ctrl2G, 0x11),
            (Register::Ctrl3C, 0x12),
            (Register::Ctrl4C, 0x13),
            (Register::Ctrl5C, 0x14),
            (Register::Ctrl6C, 0x15),
            (Register::Ctrl7G, 0x16),
            (Register::Ctrl8Xl, 0x17),
            (Register::Ctrl9Xl, 0x18),
            (Register::Ctrl10C, 0x19),
            (Register::AllIntSrc, 0x1A),
            (Register::WakeUpSrc, 0x1B),
            (Register::TapSrc, 0x1C),
            (Register::D6dSrc, 0x1D),
            (Register::StatusReg, 0x1E),
            (Register::OutTempL, 0x20),
            (Register::OutTempH, 0x21),
            (Register::OutXLG, 0x22),
            (Register::OutXHG, 0x23),
            (Register::OutYLG, 0x24),
            (Register::OutYHG, 0x25),
            (Register::OutZLG, 0x26),
            (Register::OutZHG, 0x27),
            (Register::OutXLA, 0x28),
            (Register::OutXHA, 0x29),
            (Register::OutYLA, 0x2A),
            (Register::OutYHA, 0x2B),
            (Register::OutZLA, 0x2C),
            (Register::OutZHA, 0x2D),
            (Register::FifoStatus1, 0x3A),
            (Register::FifoStatus2, 0x3B),
            (Register::Timestamp0, 0x40),
            (Register::Timestamp1, 0x41),
            (Register::Timestamp2, 0x42),
            (Register::Timestamp3, 0x43),
            (Register::TapCfg0, 0x56),
            (Register::TapCfg1, 0x57),
            (Register::TapCfg2, 0x58),
            (Register::TapThs6d, 0x59),
            (Register::IntDur2, 0x5A),
            (Register::WakeUpThs, 0x5B),
            (Register::WakeUpDur, 0x5C),
            (Register::FreeFall, 0x5D),
            (Register::Md1Cfg, 0x5E),
            (Register::Md2Cfg, 0x5F),
            (Register::XOfsUsr, 0x73),
            (Register::YOfsUsr, 0x74),
            (Register::ZOfsUsr, 0x75),
            (Register::FifoDataOutTag, 0x78),
            (Register::FifoDataOutXL, 0x79),
            (Register::FifoDataOutXH, 0x7A),
            (Register::FifoDataOutYL, 0x7B),
            (Register::FifoDataOutYH, 0x7C),
            (Register::FifoDataOutZL, 0x7D),
            (Register::FifoDataOutZH, 0x7E),
        ];

        for (register, address) in addresses {
            assert_eq!(register.addr(), address);
        }
    }
}
