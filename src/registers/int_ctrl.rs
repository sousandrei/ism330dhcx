use bitfield::bitfield;

bitfield! {
    /// INT1 pin control register (0Dh)
    pub struct Int1Ctrl(u8);
    impl Debug;
    /// Enables accelerometer data-ready interrupt on INT1 pin.
    pub int1_drdy_xl, set_int1_drdy_xl: 0;
    /// Enables gyroscope data-ready interrupt on INT1 pin.
    pub int1_drdy_g, set_int1_drdy_g: 1;
    /// Enables boot status on INT1 pin.
    pub int1_boot, set_int1_boot: 2;
    /// Enables FIFO threshold interrupt on INT1 pin.
    pub int1_fifo_th, set_int1_fifo_th: 3;
    /// Enables FIFO overrun interrupt on INT1 pin.
    pub int1_fifo_ovr, set_int1_fifo_ovr: 4;
    /// Enables FIFO full flag interrupt on INT1 pin.
    pub int1_fifo_full, set_int1_fifo_full: 5;
    /// Enables COUNTER_BDR_IA interrupt on INT1.
    pub int1_cnt_bdr, set_int1_cnt_bdr: 6;
    /// Sends DEN_DRDY (DEN stamped on Sensor Data flag) to INT1 pin.
    pub den_drdy_flag, set_den_drdy_flag: 7;
}

impl Int1Ctrl {
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

impl Default for Int1Ctrl {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Int1Ctrl {}
impl Clone for Int1Ctrl {
    fn clone(&self) -> Self {
        *self
    }
}

bitfield! {
    /// INT2 pin control register (0Eh)
    pub struct Int2Ctrl(u8);
    impl Debug;
    /// Enables accelerometer data-ready interrupt on INT2 pin.
    pub int2_drdy_xl, set_int2_drdy_xl: 0;
    /// Enables gyroscope data-ready interrupt on INT2 pin.
    pub int2_drdy_g, set_int2_drdy_g: 1;
    /// Enables DRDY on INT2 pin.
    pub int2_drdy_temp, set_int2_drdy_temp: 2;
    /// Enables FIFO threshold interrupt on INT2 pin.
    pub int2_fifo_th, set_int2_fifo_th: 3;
    /// Enables FIFO overrun interrupt on INT2 pin.
    pub int2_fifo_ovr, set_int2_fifo_ovr: 4;
    /// Enables FIFO full flag interrupt on INT2 pin.
    pub int2_fifo_full, set_int2_fifo_full: 5;
    /// Enables COUNTER_BDR_IA interrupt on INT2.
    pub int2_cnt_bdr, set_int2_cnt_bdr: 6;
}

impl Int2Ctrl {
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

impl Default for Int2Ctrl {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Int2Ctrl {}
impl Clone for Int2Ctrl {
    fn clone(&self) -> Self {
        *self
    }
}
