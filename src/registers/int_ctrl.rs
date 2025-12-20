use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

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

/// Configuration methods for INT1_CTRL register.
pub trait Int1CtrlConfig {
    /// Enables accelerometer data-ready interrupt on INT1 pin.
    fn set_int1_drdy_xl<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables gyroscope data-ready interrupt on INT1 pin.
    fn set_int1_drdy_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables boot status on INT1 pin.
    fn set_int1_boot<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables FIFO threshold interrupt on INT1 pin.
    fn set_int1_fifo_th<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables FIFO overrun interrupt on INT1 pin.
    fn set_int1_fifo_ovr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables FIFO full flag interrupt on INT1 pin.
    fn set_int1_fifo_full<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables COUNTER_BDR_IA interrupt on INT1.
    fn set_int1_cnt_bdr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Sends DEN_DRDY (DEN stamped on Sensor Data flag) to INT1 pin.
    fn set_den_drdy_flag<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Int1CtrlConfig for Ism330Dhcx {
    fn set_int1_drdy_xl<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int1Ctrl, |v| {
            let mut reg = Int1Ctrl::from_bytes([v]);
            reg.set_int1_drdy_xl(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_drdy_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int1Ctrl, |v| {
            let mut reg = Int1Ctrl::from_bytes([v]);
            reg.set_int1_drdy_g(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_boot<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int1Ctrl, |v| {
            let mut reg = Int1Ctrl::from_bytes([v]);
            reg.set_int1_boot(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_fifo_th<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int1Ctrl, |v| {
            let mut reg = Int1Ctrl::from_bytes([v]);
            reg.set_int1_fifo_th(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_fifo_ovr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int1Ctrl, |v| {
            let mut reg = Int1Ctrl::from_bytes([v]);
            reg.set_int1_fifo_ovr(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_fifo_full<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int1Ctrl, |v| {
            let mut reg = Int1Ctrl::from_bytes([v]);
            reg.set_int1_fifo_full(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_cnt_bdr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int1Ctrl, |v| {
            let mut reg = Int1Ctrl::from_bytes([v]);
            reg.set_int1_cnt_bdr(val);
            reg.into_bytes()[0]
        })
    }
    fn set_den_drdy_flag<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int1Ctrl, |v| {
            let mut reg = Int1Ctrl::from_bytes([v]);
            reg.set_den_drdy_flag(val);
            reg.into_bytes()[0]
        })
    }
}

/// Configuration methods for INT2_CTRL register.
pub trait Int2CtrlConfig {
    /// Enables accelerometer data-ready interrupt on INT2 pin.
    fn set_int2_drdy_xl<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables gyroscope data-ready interrupt on INT2 pin.
    fn set_int2_drdy_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables DRDY on INT2 pin.
    fn set_int2_drdy_temp<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables FIFO threshold interrupt on INT2 pin.
    fn set_int2_fifo_th<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables FIFO overrun interrupt on INT2 pin.
    fn set_int2_fifo_ovr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables FIFO full flag interrupt on INT2 pin.
    fn set_int2_fifo_full<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables COUNTER_BDR_IA interrupt on INT2.
    fn set_int2_cnt_bdr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Int2CtrlConfig for Ism330Dhcx {
    fn set_int2_drdy_xl<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int2Ctrl, |v| {
            let mut reg = Int2Ctrl::from_bytes([v]);
            reg.set_int2_drdy_xl(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_drdy_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int2Ctrl, |v| {
            let mut reg = Int2Ctrl::from_bytes([v]);
            reg.set_int2_drdy_g(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_drdy_temp<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int2Ctrl, |v| {
            let mut reg = Int2Ctrl::from_bytes([v]);
            reg.set_int2_drdy_temp(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_fifo_th<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int2Ctrl, |v| {
            let mut reg = Int2Ctrl::from_bytes([v]);
            reg.set_int2_fifo_th(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_fifo_ovr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int2Ctrl, |v| {
            let mut reg = Int2Ctrl::from_bytes([v]);
            reg.set_int2_fifo_ovr(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_fifo_full<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int2Ctrl, |v| {
            let mut reg = Int2Ctrl::from_bytes([v]);
            reg.set_int2_fifo_full(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_cnt_bdr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Int2Ctrl, |v| {
            let mut reg = Int2Ctrl::from_bytes([v]);
            reg.set_int2_cnt_bdr(val);
            reg.into_bytes()[0]
        })
    }
}
