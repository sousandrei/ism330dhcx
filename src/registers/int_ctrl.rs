use modular_bitfield::prelude::*;

/// INT1 pin control register (0Dh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Int1Ctrl {
    /// Enables accelerometer data-ready interrupt on INT1 pin.
    pub int1_drdy_xl: bool,
    /// Enables gyroscope data-ready interrupt on INT1 pin.
    pub int1_drdy_g: bool,
    /// Enables boot status on INT1 pin.
    pub int1_boot: bool,
    /// Enables FIFO threshold interrupt on INT1 pin.
    pub int1_fifo_th: bool,
    /// Enables FIFO overrun interrupt on INT1 pin.
    pub int1_fifo_ovr: bool,
    /// Enables FIFO full flag interrupt on INT1 pin.
    pub int1_fifo_full: bool,
    /// Enables COUNTER_BDR_IA interrupt on INT1.
    pub int1_cnt_bdr: bool,
    /// Sends DEN_DRDY (DEN stamped on Sensor Data flag) to INT1 pin.
    pub den_drdy_flag: bool,
}

impl Default for Int1Ctrl {
    fn default() -> Self {
        Self::new()
    }
}

/// INT2 pin control register (0Eh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Int2Ctrl {
    /// Enables accelerometer data-ready interrupt on INT2 pin.
    pub int2_drdy_xl: bool,
    /// Enables gyroscope data-ready interrupt on INT2 pin.
    pub int2_drdy_g: bool,
    /// Enables DRDY on INT2 pin.
    pub int2_drdy_temp: bool,
    /// Enables FIFO threshold interrupt on INT2 pin.
    pub int2_fifo_th: bool,
    /// Enables FIFO overrun interrupt on INT2 pin.
    pub int2_fifo_ovr: bool,
    /// Enables FIFO full flag interrupt on INT2 pin.
    pub int2_fifo_full: bool,
    /// Enables COUNTER_BDR_IA interrupt on INT2.
    pub int2_cnt_bdr: bool,
    /// Enables sleep change state interrupt on INT2 pin.
    pub int2_sleep_change: bool,
}

impl Default for Int2Ctrl {
    fn default() -> Self {
        Self::new()
    }
}
