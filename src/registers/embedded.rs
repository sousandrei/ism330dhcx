//! Embedded-function register definitions.

use modular_bitfield::{
    bitfield,
    specifiers::{B1, B2, B3, B4, B6, B8},
};

/// Registers in the embedded-function bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum EmbeddedRegister {
    PageSel = 0x02,
    EmbFuncEnA = 0x04,
    EmbFuncEnB = 0x05,
    PageAddress = 0x08,
    PageValue = 0x09,
    EmbFuncInt1 = 0x0A,
    FsmInt1A = 0x0B,
    FsmInt1B = 0x0C,
    MlcInt1 = 0x0D,
    EmbFuncInt2 = 0x0E,
    FsmInt2A = 0x0F,
    FsmInt2B = 0x10,
    MlcInt2 = 0x11,
    EmbFuncStatus = 0x12,
    FsmStatusA = 0x13,
    FsmStatusB = 0x14,
    MlcStatus = 0x15,
    PageRw = 0x17,
    EmbFuncFifoCfg = 0x44,
    FsmEnableA = 0x46,
    FsmEnableB = 0x47,
    FsmLongCounterL = 0x48,
    FsmLongCounterH = 0x49,
    FsmLongCounterClear = 0x4A,
    FsmOuts1 = 0x4C,
    FsmOuts16 = 0x5B,
    EmbFuncOdrCfgB = 0x5F,
    EmbFuncOdrCfgC = 0x60,
    StepCounterL = 0x62,
    StepCounterH = 0x63,
    EmbFuncSrc = 0x64,
    EmbFuncInitA = 0x66,
    EmbFuncInitB = 0x67,
    Mlc0Src = 0x70,
    Mlc7Src = 0x77,
}

impl EmbeddedRegister {
    /// Return the register address.
    pub const fn addr(self) -> u8 {
        self as u8
    }
}

/// Embedded page selector.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct PageSel {
    #[skip]
    pub __: B4,
    pub page_sel: B4,
}

/// Embedded-function enable register A.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncEnA {
    #[skip]
    pub __: B3,
    pub pedo_en: bool,
    pub tilt_en: bool,
    pub sign_motion_en: bool,
    #[skip]
    pub ___: B2,
}

/// Embedded-function enable register B.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncEnB {
    pub fsm_en: bool,
    #[skip]
    pub __: B2,
    pub fifo_compr_en: bool,
    pub mlc_en: bool,
    #[skip]
    pub ___: B3,
}

/// Embedded-function interrupt routing register.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncInt {
    #[skip]
    pub __: B3,
    pub step_detector: bool,
    pub tilt: bool,
    pub significant_motion: bool,
    #[skip]
    pub ___: B1,
    pub fsm_long_counter: bool,
}

/// Embedded-function status register.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncStatus {
    #[skip]
    pub __: B3,
    pub step_detected: bool,
    pub tilt: bool,
    pub significant_motion: bool,
    #[skip]
    pub ___: B1,
    pub fsm_long_counter: bool,
}

/// FSM enable or status bits for eight FSM programs.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FsmBits {
    pub bits: B8,
}

/// Embedded FIFO configuration.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncFifoCfg {
    #[skip]
    pub __: B6,
    pub pedo_fifo_en: bool,
    #[skip]
    pub ___: B1,
}

/// Embedded-function ODR configuration.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncOdrCfg {
    #[skip]
    pub __: B3,
    pub odr: B2,
    #[skip]
    pub ___: B3,
}

/// Embedded-function source register.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncSrc {
    #[skip]
    pub __: B2,
    pub stepcounter_bit_set: bool,
    pub step_overflow: bool,
    pub step_count_delta: bool,
    pub step_detected: bool,
    #[skip]
    pub ___: B1,
    pub pedo_rst_step: bool,
}

/// Embedded-function initialization register A.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncInitA {
    #[skip]
    pub __: B3,
    pub step_detector: bool,
    pub tilt: bool,
    pub significant_motion: bool,
    #[skip]
    pub ___: B2,
}

/// Embedded-function initialization register B.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncInitB {
    pub fsm: bool,
    #[skip]
    pub __: B2,
    pub fifo_compression: bool,
    pub mlc: bool,
    #[skip]
    pub ___: B3,
}

/// MLC result source bits.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct MlcStatus {
    pub results: B8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_addresses_and_layouts() {
        assert_eq!(EmbeddedRegister::EmbFuncEnA.addr(), 0x04);
        assert_eq!(EmbeddedRegister::FsmOuts16.addr(), 0x5B);

        let mut en = EmbFuncEnA::new();
        en.set_pedo_en(true);
        en.set_tilt_en(true);
        en.set_sign_motion_en(true);
        assert_eq!(en.into_bytes(), [0x38]);

        let mut status = EmbFuncStatus::new();
        status.set_step_detected(true);
        status.set_tilt(true);
        status.set_significant_motion(true);
        assert_eq!(status.into_bytes(), [0x38]);
    }
}
