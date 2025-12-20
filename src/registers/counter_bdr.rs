use modular_bitfield::prelude::*;

/// Counter batch data rate register 1 (0Bh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct CounterBdrReg1 {
    /// In conjunction with CNT_BDR_TH_[7:0] in COUNTER_BDR_REG2 (0Ch), sets the threshold for the internal counter of batch events.
    pub cnt_bdr_th_msb: B3,
    #[skip]
    pub __: B2,
    /// Selects the trigger for the internal counter of batch events between XL and gyro.
    pub trig_counter_bdr: bool,
    /// Resets the internal counter of batch events for a single sensor.
    pub rst_counter_bdr: bool,
    /// Enables pulsed data-ready mode.
    pub dataready_pulsed: bool,
}

impl Default for CounterBdrReg1 {
    fn default() -> Self {
        Self::new()
    }
}

/// Counter batch data rate register 2 (0Ch)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct CounterBdrReg2 {
    /// In conjunction with CNT_BDR_TH_[10:8] in COUNTER_BDR_REG1 (0Bh), sets the threshold for the internal counter of batch events.
    pub cnt_bdr_th_lsb: B8,
}

impl Default for CounterBdrReg2 {
    fn default() -> Self {
        Self::new()
    }
}
