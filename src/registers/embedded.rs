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
    FsmOuts2,
    FsmOuts3,
    FsmOuts4,
    FsmOuts5,
    FsmOuts6,
    FsmOuts7,
    FsmOuts8,
    FsmOuts9,
    FsmOuts10,
    FsmOuts11,
    FsmOuts12,
    FsmOuts13,
    FsmOuts14,
    FsmOuts15,
    FsmOuts16 = 0x5B,
    EmbFuncOdrCfgB = 0x5F,
    EmbFuncOdrCfgC = 0x60,
    StepCounterL = 0x62,
    StepCounterH = 0x63,
    EmbFuncSrc = 0x64,
    EmbFuncInitA = 0x66,
    EmbFuncInitB = 0x67,
    Mlc0Src = 0x70,
    Mlc1Src,
    Mlc2Src,
    Mlc3Src,
    Mlc4Src,
    Mlc5Src,
    Mlc6Src,
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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PageSel {
    #[skip]
    pub __: B4,
    pub page_sel: B4,
}

impl Default for PageSel {
    fn default() -> Self {
        Self::from_bytes([0x01])
    }
}

/// Embedded-function enable register A.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EmbFuncEnA {
    #[skip]
    pub __: B3,
    pub pedo_en: bool,
    pub tilt_en: bool,
    pub sign_motion_en: bool,
    #[skip]
    pub ___: B2,
}

impl Default for EmbFuncEnA {
    fn default() -> Self {
        Self::new()
    }
}

/// Embedded-function enable register B.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EmbFuncEnB {
    pub fsm_en: bool,
    #[skip]
    pub __: B2,
    pub fifo_compr_en: bool,
    pub mlc_en: bool,
    #[skip]
    pub ___: B3,
}

impl Default for EmbFuncEnB {
    fn default() -> Self {
        Self::new()
    }
}

/// Embedded-function interrupt routing register.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

impl Default for EmbFuncInt {
    fn default() -> Self {
        Self::new()
    }
}

/// Embedded-function status register.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

impl Default for EmbFuncStatus {
    fn default() -> Self {
        Self::new()
    }
}

/// FSM enable or status bits for eight FSM programs.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct FsmBits {
    pub bits: B8,
}

impl Default for FsmBits {
    fn default() -> Self {
        Self::new()
    }
}

/// Embedded FIFO configuration.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EmbFuncFifoCfg {
    #[skip]
    pub __: B6,
    pub pedo_fifo_en: bool,
    #[skip]
    pub ___: B1,
}

impl Default for EmbFuncFifoCfg {
    fn default() -> Self {
        Self::new()
    }
}

/// Embedded-function FSM ODR configuration register.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EmbFuncOdrCfgB {
    #[skip]
    pub __: B1,
    pub reserved_6: bool,
    #[skip]
    pub ___: B1,
    pub odr: B2,
    #[skip]
    pub ____: B1,
    pub reserved_1: bool,
    pub reserved_0: bool,
}

impl Default for EmbFuncOdrCfgB {
    fn default() -> Self {
        Self::from_bytes([0x4b])
    }
}

/// Embedded-function MLC ODR configuration register.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EmbFuncOdrCfgC {
    #[skip]
    pub __: B2,
    pub odr: B2,
    #[skip]
    pub ___: B1,
    pub reserved_2: bool,
    #[skip]
    pub ____: B1,
    pub reserved_0: bool,
}

impl Default for EmbFuncOdrCfgC {
    fn default() -> Self {
        Self::from_bytes([0x15])
    }
}

/// Embedded-function source register.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EmbFuncSrc {
    #[skip]
    pub __: B2,
    pub step_counter_bit_set: bool,
    pub step_overflow: bool,
    pub step_count_delta: bool,
    pub step_detected: bool,
    #[skip]
    pub ___: B1,
    pub pedo_rst_step: bool,
}

impl Default for EmbFuncSrc {
    fn default() -> Self {
        Self::new()
    }
}

/// Embedded-function initialization register A.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EmbFuncInitA {
    #[skip]
    pub __: B3,
    pub step_detector: bool,
    pub tilt: bool,
    pub significant_motion: bool,
    #[skip]
    pub ___: B2,
}

impl Default for EmbFuncInitA {
    fn default() -> Self {
        Self::new()
    }
}

/// Embedded-function initialization register B.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct EmbFuncInitB {
    pub fsm: bool,
    #[skip]
    pub __: B2,
    pub fifo_compression: bool,
    pub mlc: bool,
    #[skip]
    pub ___: B3,
}

impl Default for EmbFuncInitB {
    fn default() -> Self {
        Self::new()
    }
}

/// MLC result source bits.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MlcStatus {
    pub results: B8,
}

impl Default for MlcStatus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_addresses_and_layouts() {
        assert_eq!(PageSel::default().into_bytes(), [0x01]);
        assert_eq!(EmbFuncOdrCfgB::default().into_bytes(), [0x4b]);
        assert_eq!(EmbFuncOdrCfgC::default().into_bytes(), [0x15]);

        let addresses = [
            (EmbeddedRegister::PageSel, 0x02),
            (EmbeddedRegister::EmbFuncEnA, 0x04),
            (EmbeddedRegister::EmbFuncEnB, 0x05),
            (EmbeddedRegister::PageAddress, 0x08),
            (EmbeddedRegister::PageValue, 0x09),
            (EmbeddedRegister::EmbFuncInt1, 0x0A),
            (EmbeddedRegister::FsmInt1A, 0x0B),
            (EmbeddedRegister::FsmInt1B, 0x0C),
            (EmbeddedRegister::MlcInt1, 0x0D),
            (EmbeddedRegister::EmbFuncInt2, 0x0E),
            (EmbeddedRegister::FsmInt2A, 0x0F),
            (EmbeddedRegister::FsmInt2B, 0x10),
            (EmbeddedRegister::MlcInt2, 0x11),
            (EmbeddedRegister::EmbFuncStatus, 0x12),
            (EmbeddedRegister::FsmStatusA, 0x13),
            (EmbeddedRegister::FsmStatusB, 0x14),
            (EmbeddedRegister::MlcStatus, 0x15),
            (EmbeddedRegister::PageRw, 0x17),
            (EmbeddedRegister::EmbFuncFifoCfg, 0x44),
            (EmbeddedRegister::FsmEnableA, 0x46),
            (EmbeddedRegister::FsmEnableB, 0x47),
            (EmbeddedRegister::FsmLongCounterL, 0x48),
            (EmbeddedRegister::FsmLongCounterH, 0x49),
            (EmbeddedRegister::FsmLongCounterClear, 0x4A),
            (EmbeddedRegister::FsmOuts1, 0x4C),
            (EmbeddedRegister::FsmOuts2, 0x4D),
            (EmbeddedRegister::FsmOuts3, 0x4E),
            (EmbeddedRegister::FsmOuts4, 0x4F),
            (EmbeddedRegister::FsmOuts5, 0x50),
            (EmbeddedRegister::FsmOuts6, 0x51),
            (EmbeddedRegister::FsmOuts7, 0x52),
            (EmbeddedRegister::FsmOuts8, 0x53),
            (EmbeddedRegister::FsmOuts9, 0x54),
            (EmbeddedRegister::FsmOuts10, 0x55),
            (EmbeddedRegister::FsmOuts11, 0x56),
            (EmbeddedRegister::FsmOuts12, 0x57),
            (EmbeddedRegister::FsmOuts13, 0x58),
            (EmbeddedRegister::FsmOuts14, 0x59),
            (EmbeddedRegister::FsmOuts15, 0x5A),
            (EmbeddedRegister::FsmOuts16, 0x5B),
            (EmbeddedRegister::EmbFuncOdrCfgB, 0x5F),
            (EmbeddedRegister::EmbFuncOdrCfgC, 0x60),
            (EmbeddedRegister::StepCounterL, 0x62),
            (EmbeddedRegister::StepCounterH, 0x63),
            (EmbeddedRegister::EmbFuncSrc, 0x64),
            (EmbeddedRegister::EmbFuncInitA, 0x66),
            (EmbeddedRegister::EmbFuncInitB, 0x67),
            (EmbeddedRegister::Mlc0Src, 0x70),
            (EmbeddedRegister::Mlc1Src, 0x71),
            (EmbeddedRegister::Mlc2Src, 0x72),
            (EmbeddedRegister::Mlc3Src, 0x73),
            (EmbeddedRegister::Mlc4Src, 0x74),
            (EmbeddedRegister::Mlc5Src, 0x75),
            (EmbeddedRegister::Mlc6Src, 0x76),
            (EmbeddedRegister::Mlc7Src, 0x77),
        ];

        for (register, address) in addresses {
            assert_eq!(register.addr(), address);
        }

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
