use bitfield::bitfield;

bitfield! {
    /// Counter batch data rate register 1 (0Bh)
    pub struct CounterBdrReg1(u8);
    impl Debug;
    /// In conjunction with CNT_BDR_TH_[7:0] in COUNTER_BDR_REG2 (0Ch), sets the threshold for the internal counter of batch events.
    pub cnt_bdr_th_msb, set_cnt_bdr_th_msb: 2, 0;
    /// Selects the trigger for the internal counter of batch events between XL and gyro.
    pub trig_counter_bdr, set_trig_counter_bdr: 5;
    /// Resets the internal counter of batch events for a single sensor.
    pub rst_counter_bdr, set_rst_counter_bdr: 6;
    /// Enables pulsed data-ready mode.
    pub dataready_pulsed, set_dataready_pulsed: 7;
}

impl CounterBdrReg1 {
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

impl Default for CounterBdrReg1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for CounterBdrReg1 {}
impl Clone for CounterBdrReg1 {
    fn clone(&self) -> Self {
        *self
    }
}

bitfield! {
    /// Counter batch data rate register 2 (0Ch)
    pub struct CounterBdrReg2(u8);
    impl Debug;
    /// In conjunction with CNT_BDR_TH_[10:8] in COUNTER_BDR_REG1 (0Bh), sets the threshold for the internal counter of batch events.
    pub cnt_bdr_th_lsb, set_cnt_bdr_th_lsb: 7, 0;
}

impl CounterBdrReg2 {
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

impl Default for CounterBdrReg2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for CounterBdrReg2 {}
impl Clone for CounterBdrReg2 {
    fn clone(&self) -> Self {
        *self
    }
}
