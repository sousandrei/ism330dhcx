use modular_bitfield::prelude::*;

/// Embedded function status register (35h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct EmbFuncStatusMainpage {
    #[skip]
    pub __1: B3,
    /// Interrupt status bit for step detection.
    pub is_step_det: bool,
    /// Interrupt status bit for tilt detection.
    pub is_tilt: bool,
    /// Interrupt status bit for significant motion detection.
    pub is_sigmot: bool,
    #[skip]
    pub __2: B1,
    /// Interrupt status bit for FSM long counter timeout interrupt event.
    pub is_fsm_lc: bool,
}

impl Default for EmbFuncStatusMainpage {
    fn default() -> Self {
        Self::new()
    }
}

/// Finite State Machine status register A (36h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FsmStatusAMainpage {
    pub is_fsm1: bool,
    pub is_fsm2: bool,
    pub is_fsm3: bool,
    pub is_fsm4: bool,
    pub is_fsm5: bool,
    pub is_fsm6: bool,
    pub is_fsm7: bool,
    pub is_fsm8: bool,
}

impl Default for FsmStatusAMainpage {
    fn default() -> Self {
        Self::new()
    }
}

/// Finite State Machine status register B (37h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FsmStatusBMainpage {
    pub is_fsm9: bool,
    pub is_fsm10: bool,
    pub is_fsm11: bool,
    pub is_fsm12: bool,
    pub is_fsm13: bool,
    pub is_fsm14: bool,
    pub is_fsm15: bool,
    pub is_fsm16: bool,
}

impl Default for FsmStatusBMainpage {
    fn default() -> Self {
        Self::new()
    }
}

/// Machine Learning Core status register (38h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct MlcStatusMainpage {
    pub is_mlc1: bool,
    pub is_mlc2: bool,
    pub is_mlc3: bool,
    pub is_mlc4: bool,
    pub is_mlc5: bool,
    pub is_mlc6: bool,
    pub is_mlc7: bool,
    pub is_mlc8: bool,
}

impl Default for MlcStatusMainpage {
    fn default() -> Self {
        Self::new()
    }
}
