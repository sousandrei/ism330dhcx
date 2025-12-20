use bitfield::bitfield;

bitfield! {
    /// Embedded function status register (35h)
    pub struct EmbFuncStatusMainpage(u8);
    impl Debug;
    /// Interrupt status bit for step detection.
    pub is_step_det, set_is_step_det: 3;
    /// Interrupt status bit for tilt detection.
    pub is_tilt, set_is_tilt: 4;
    /// Interrupt status bit for significant motion detection.
    pub is_sigmot, set_is_sigmot: 5;
    /// Interrupt status bit for FSM long counter timeout interrupt event.
    pub is_fsm_lc, set_is_fsm_lc: 7;
}

impl EmbFuncStatusMainpage {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for EmbFuncStatusMainpage {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for EmbFuncStatusMainpage {}
impl Clone for EmbFuncStatusMainpage {
    fn clone(&self) -> Self {
        *self
    }
}
