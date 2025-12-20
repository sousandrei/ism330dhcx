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
pub mod drd_src;
pub mod emb_func_status;
pub mod fifo;
pub mod free_fall;
pub mod fsm_status;
pub mod func_cfg_access;
pub mod int_ctrl;
pub mod int_dur2;
pub mod internal_freq_fine;
pub mod md_cfg;
pub mod mlc_status;
pub mod ofs_usr;
pub mod out_g;
pub mod out_temp;
pub mod out_xl;
pub mod pin_ctrl;
pub mod status_master;
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
pub use ctrl1_xl::*;
pub use ctrl2_g::*;
pub use ctrl3_c::*;
pub use ctrl4_c::*;
pub use ctrl5_c::*;
pub use ctrl6_c::*;
pub use ctrl7_g::*;
pub use ctrl8_xl::*;
pub use ctrl9_xl::*;
pub use ctrl10_c::*;
pub use drd_src::*;
pub use emb_func_status::*;
pub use fifo::*;
pub use free_fall::*;
pub use fsm_status::*;
pub use func_cfg_access::*;
pub use int_ctrl::*;
pub use int_dur2::*;
pub use internal_freq_fine::*;
pub use md_cfg::*;
pub use mlc_status::*;
pub use ofs_usr::*;
pub use out_g::*;
pub use out_temp::*;
pub use out_xl::*;
pub use pin_ctrl::*;
pub use status_master::*;
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
    /// SDO, OCS_AUX, SDO_AUX pins pull-up enable/disable register
    PinCtrl = 0x02,
    /// FIFO control register 1
    FifoCtrl1 = 0x07,
    /// FIFO control register 2
    FifoCtrl2 = 0x08,
    /// FIFO control register 3
    FifoCtrl3 = 0x09,
    /// FIFO control register 4
    FifoCtrl4 = 0x0A,
    /// Counter batch data rate register 1
    CounterBdrReg1 = 0x0B,
    /// Counter batch data rate register 2
    CounterBdrReg2 = 0x0C,
    /// INT1 pin control register
    Int1Ctrl = 0x0D,
    /// INT2 pin control register
    Int2Ctrl = 0x0E,
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
    /// Embedded function status register
    EmbFuncStatusMainpage = 0x35,
    /// FSM status register A
    FsmStatusAMainpage = 0x36,
    /// FSM status register B
    FsmStatusBMainpage = 0x37,
    /// MLC status register
    MlcStatusMainpage = 0x38,
    /// Master status register
    StatusMasterMainpage = 0x39,
    /// FIFO status register 1
    FifoStatus1 = 0x3A,
    /// FIFO status register 2
    FifoStatus2 = 0x3B,
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
    /// Tap threshold and 6D orientation threshold register
    TapThs6d = 0x59,
    /// Interrupt duration register 2
    IntDur2 = 0x5A,
    /// Wake-up threshold register
    WakeUpThs = 0x5B,
    /// Wake-up duration register
    WakeUpDur = 0x5C,
    /// Free-fall threshold register
    FreeFall = 0x5D,
    /// Functions routing on INT1 register
    Md1Cfg = 0x5E,
    /// Functions routing on INT2 register
    Md2Cfg = 0x5F,
    /// Internal frequency fine-tuning register
    InternalFreqFine = 0x63,
    /// OIS interrupt register
    IntOis = 0x6F,
    /// OIS control register 1
    Ctrl1Ois = 0x70,
    /// OIS control register 2
    Ctrl2Ois = 0x71,
    /// OIS control register 3
    Ctrl3Ois = 0x72,
    /// User offset correction register X
    XOfsUsr = 0x73,
    /// User offset correction register Y
    YOfsUsr = 0x74,
    /// User offset correction register Z
    ZOfsUsr = 0x75,
    /// FIFO data output tag register
    FifoDataOutTag = 0x78,
    /// FIFO data output X (low)
    FifoDataOutXL = 0x79,
    /// FIFO data output X (high)
    FifoDataOutXH = 0x7A,
    /// FIFO data output Y (low)
    FifoDataOutYL = 0x7B,
    /// FIFO data output Y (high)
    FifoDataOutYH = 0x7C,
    /// FIFO data output Z (low)
    FifoDataOutZL = 0x7D,
    /// FIFO data output Z (high)
    FifoDataOutZH = 0x7E,
}

impl Register {
    /// Returns the register address as a u8.
    pub fn addr(self) -> u8 {
        self as u8
    }
}
