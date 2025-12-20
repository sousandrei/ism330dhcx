use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Control register 4 (13h)
    pub struct Ctrl4C(u8);
    impl Debug;
    /// Enables gyroscope digital LPF1.
    pub lpf1_sel_g, set_lpf1_sel_g: 1;
    /// Disables I2C interface.
    pub i2c_disable, set_i2c_disable: 2;
    /// Enables data available mask until filter settling ends.
    pub drdy_mask, set_drdy_mask: 3;
    /// All interrupt signals available on INT1 pin enable.
    pub int2_on_int1, set_int2_on_int1: 5;
    /// Enables gyroscope Sleep mode.
    pub sleep_g, set_sleep_g: 6;
}

impl Ctrl4C {
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

impl Default for Ctrl4C {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CTRL4_C register.
pub trait Ctrl4CConfig {
    /// Enables gyroscope digital LPF1.
    fn set_lpf1_sel_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Disables I2C interface.
    fn set_i2c_disable<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enables data available mask until filter settling ends.
    fn set_drdy_mask<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// All interrupt signals available on INT1 pin enable.
    fn set_int2_on_int1<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enables gyroscope Sleep mode.
    fn set_sleep_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ctrl4CConfig for Ism330Dhcx {
    fn set_lpf1_sel_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl4C, |v| {
            let mut reg = Ctrl4C::from_bytes([v]);
            reg.set_lpf1_sel_g(val);
            reg.into_bytes()[0]
        })
    }

    fn set_i2c_disable<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl4C, |v| {
            let mut reg = Ctrl4C::from_bytes([v]);
            reg.set_i2c_disable(val);
            reg.into_bytes()[0]
        })
    }

    fn set_drdy_mask<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl4C, |v| {
            let mut reg = Ctrl4C::from_bytes([v]);
            reg.set_drdy_mask(val);
            reg.into_bytes()[0]
        })
    }

    fn set_int2_on_int1<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl4C, |v| {
            let mut reg = Ctrl4C::from_bytes([v]);
            reg.set_int2_on_int1(val);
            reg.into_bytes()[0]
        })
    }

    fn set_sleep_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl4C, |v| {
            let mut reg = Ctrl4C::from_bytes([v]);
            reg.set_sleep_g(val);
            reg.into_bytes()[0]
        })
    }
}
