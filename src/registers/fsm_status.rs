use bitfield::bitfield;

bitfield! {
    /// Finite State Machine status register A (36h)
    pub struct FsmStatusAMainpage(u8);
    impl Debug;
    pub is_fsm1, set_is_fsm1: 0;
    pub is_fsm2, set_is_fsm2: 1;
    pub is_fsm3, set_is_fsm3: 2;
    pub is_fsm4, set_is_fsm4: 3;
    pub is_fsm5, set_is_fsm5: 4;
    pub is_fsm6, set_is_fsm6: 5;
    pub is_fsm7, set_is_fsm7: 6;
    pub is_fsm8, set_is_fsm8: 7;
}

impl FsmStatusAMainpage {
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

impl Default for FsmStatusAMainpage {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for FsmStatusAMainpage {}
impl Clone for FsmStatusAMainpage {
    fn clone(&self) -> Self {
        *self
    }
}

bitfield! {
    /// Finite State Machine status register B (37h)
    pub struct FsmStatusBMainpage(u8);
    impl Debug;
    pub is_fsm9, set_is_fsm9: 0;
    pub is_fsm10, set_is_fsm10: 1;
    pub is_fsm11, set_is_fsm11: 2;
    pub is_fsm12, set_is_fsm12: 3;
    pub is_fsm13, set_is_fsm13: 4;
    pub is_fsm14, set_is_fsm14: 5;
    pub is_fsm15, set_is_fsm15: 6;
    pub is_fsm16, set_is_fsm16: 7;
}

impl FsmStatusBMainpage {
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

impl Default for FsmStatusBMainpage {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for FsmStatusBMainpage {}
impl Clone for FsmStatusBMainpage {
    fn clone(&self) -> Self {
        *self
    }
}
