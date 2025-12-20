use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Single/double-tap selection and wake-up configuration (5Bh)
    pub struct WakeUpThs(u8);
    impl Debug;
    /// Threshold for wakeup.
    pub wk_ths, set_wk_ths: 5, 0;
    /// Sends the low-pass filtered data with user offset correction to the wakeup function.
    pub usr_off_on_wu, set_usr_off_on_wu: 6;
    /// Single/double-tap event enable.
    pub single_double_tap, set_single_double_tap: 7;
}

impl WakeUpThs {
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

impl Default for WakeUpThs {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for WAKE_UP_THS register.
pub trait WakeUpThsConfig {
    /// Threshold for wakeup.
    fn set_wk_ths<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Sends the low-pass filtered data with user offset correction to the wakeup function.
    fn set_usr_off_on_wu<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Single/double-tap event enable.
    fn set_single_double_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl WakeUpThsConfig for Ism330Dhcx {
    fn set_wk_ths<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::WakeUpThs, |v| {
            let mut reg = WakeUpThs::from_bytes([v]);
            reg.set_wk_ths(val);
            reg.into_bytes()[0]
        })
    }

    fn set_usr_off_on_wu<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::WakeUpThs, |v| {
            let mut reg = WakeUpThs::from_bytes([v]);
            reg.set_usr_off_on_wu(val);
            reg.into_bytes()[0]
        })
    }

    fn set_single_double_tap<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::WakeUpThs, |v| {
            let mut reg = WakeUpThs::from_bytes([v]);
            reg.set_single_double_tap(val);
            reg.into_bytes()[0]
        })
    }
}
