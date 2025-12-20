use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Functions routing on INT1 register (5Eh)
    pub struct Md1Cfg(u8);
    impl Debug;
    /// Routing of sensor hub communication concluded event on INT1.
    pub int1_shub, set_int1_shub: 0;
    /// Routing of embedded functions event on INT1.
    pub int1_emb_func, set_int1_emb_func: 1;
    /// Routing of 6D event on INT1.
    pub int1_6d, set_int1_6d: 2;
    /// Routing of double-tap event on INT1.
    pub int1_double_tap, set_int1_double_tap: 3;
    /// Routing of free-fall event on INT1.
    pub int1_ff, set_int1_ff: 4;
    /// Routing of wakeup event on INT1.
    pub int1_wu, set_int1_wu: 5;
    /// Routing of single-tap recognition event on INT1.
    pub int1_single_tap, set_int1_single_tap: 6;
    /// Routing of activity/inactivity recognition event on INT1.
    pub int1_sleep_change, set_int1_sleep_change: 7;
}

impl Md1Cfg {
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

impl Default for Md1Cfg {
    fn default() -> Self {
        Self::new()
    }
}

bitfield! {
    /// Functions routing on INT2 register (5Fh)
    pub struct Md2Cfg(u8);
    impl Debug;
    /// Enables routing on INT2 pin of the alert for timestamp overflow.
    pub int2_timestamp, set_int2_timestamp: 0;
    /// Routing of embedded functions event on INT2.
    pub int2_emb_func, set_int2_emb_func: 1;
    /// Routing of 6D event on INT2.
    pub int2_6d, set_int2_6d: 2;
    /// Routing of double-tap event on INT2.
    pub int2_double_tap, set_int2_double_tap: 3;
    /// Routing of free-fall event on INT2.
    pub int2_ff, set_int2_ff: 4;
    /// Routing of wakeup event on INT2.
    pub int2_wu, set_int2_wu: 5;
    /// Single-tap recognition routing on INT2.
    pub int2_single_tap, set_int2_single_tap: 6;
    /// Routing of activity/inactivity recognition event on INT2.
    pub int2_sleep_change, set_int2_sleep_change: 7;
}

impl Md2Cfg {
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

impl Default for Md2Cfg {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for MD1_CFG register.
pub trait Md1CfgConfig {
    /// Routing of sensor hub communication concluded event on INT1.
    fn set_int1_shub<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of embedded functions event on INT1.
    fn set_int1_emb_func<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of 6D event on INT1.
    fn set_int1_6d<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of double-tap event on INT1.
    fn set_int1_double_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of free-fall event on INT1.
    fn set_int1_ff<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of wakeup event on INT1.
    fn set_int1_wu<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of single-tap recognition event on INT1.
    fn set_int1_single_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of activity/inactivity recognition event on INT1.
    fn set_int1_sleep_change<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Md1CfgConfig for Ism330Dhcx {
    fn set_int1_shub<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md1Cfg, |v| {
            let mut reg = Md1Cfg::from_bytes([v]);
            reg.set_int1_shub(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_emb_func<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md1Cfg, |v| {
            let mut reg = Md1Cfg::from_bytes([v]);
            reg.set_int1_emb_func(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_6d<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md1Cfg, |v| {
            let mut reg = Md1Cfg::from_bytes([v]);
            reg.set_int1_6d(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_double_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md1Cfg, |v| {
            let mut reg = Md1Cfg::from_bytes([v]);
            reg.set_int1_double_tap(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_ff<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md1Cfg, |v| {
            let mut reg = Md1Cfg::from_bytes([v]);
            reg.set_int1_ff(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_wu<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md1Cfg, |v| {
            let mut reg = Md1Cfg::from_bytes([v]);
            reg.set_int1_wu(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_single_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md1Cfg, |v| {
            let mut reg = Md1Cfg::from_bytes([v]);
            reg.set_int1_single_tap(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int1_sleep_change<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md1Cfg, |v| {
            let mut reg = Md1Cfg::from_bytes([v]);
            reg.set_int1_sleep_change(val);
            reg.into_bytes()[0]
        })
    }
}

/// Configuration methods for MD2_CFG register.
pub trait Md2CfgConfig {
    /// Enables routing on INT2 pin of the alert for timestamp overflow.
    fn set_int2_timestamp<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of embedded functions event on INT2.
    fn set_int2_emb_func<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of 6D event on INT2.
    fn set_int2_6d<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of double-tap event on INT2.
    fn set_int2_double_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of free-fall event on INT2.
    fn set_int2_ff<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of wakeup event on INT2.
    fn set_int2_wu<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Single-tap recognition routing on INT2.
    fn set_int2_single_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Routing of activity/inactivity recognition event on INT2.
    fn set_int2_sleep_change<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Md2CfgConfig for Ism330Dhcx {
    fn set_int2_timestamp<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md2Cfg, |v| {
            let mut reg = Md2Cfg::from_bytes([v]);
            reg.set_int2_timestamp(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_emb_func<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md2Cfg, |v| {
            let mut reg = Md2Cfg::from_bytes([v]);
            reg.set_int2_emb_func(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_6d<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md2Cfg, |v| {
            let mut reg = Md2Cfg::from_bytes([v]);
            reg.set_int2_6d(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_double_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md2Cfg, |v| {
            let mut reg = Md2Cfg::from_bytes([v]);
            reg.set_int2_double_tap(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_ff<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md2Cfg, |v| {
            let mut reg = Md2Cfg::from_bytes([v]);
            reg.set_int2_ff(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_wu<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md2Cfg, |v| {
            let mut reg = Md2Cfg::from_bytes([v]);
            reg.set_int2_wu(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_single_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md2Cfg, |v| {
            let mut reg = Md2Cfg::from_bytes([v]);
            reg.set_int2_single_tap(val);
            reg.into_bytes()[0]
        })
    }
    fn set_int2_sleep_change<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Md2Cfg, |v| {
            let mut reg = Md2Cfg::from_bytes([v]);
            reg.set_int2_sleep_change(val);
            reg.into_bytes()[0]
        })
    }
}
