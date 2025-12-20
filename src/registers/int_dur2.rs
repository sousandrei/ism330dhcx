use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Tap recognition function setting register (5Ah)
    pub struct IntDur2(u8);
    impl Debug;
    /// Maximum duration of overthreshold event.
    pub shock, set_shock: 1, 0;
    /// Expected quiet time after a tap detection.
    pub quiet, set_quiet: 3, 2;
    /// Duration of maximum time gap for double-tap recognition.
    pub dur, set_dur: 7, 4;
}

impl IntDur2 {
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

impl Default for IntDur2 {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for INT_DUR2 register.
pub trait IntDur2Config {
    /// Maximum duration of overthreshold event.
    fn set_shock<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Expected quiet time after a tap detection.
    fn set_quiet<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Duration of maximum time gap for double-tap recognition.
    fn set_dur<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl IntDur2Config for Ism330Dhcx {
    fn set_shock<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::IntDur2, |v| {
            let mut reg = IntDur2::from_bytes([v]);
            reg.set_shock(val);
            reg.into_bytes()[0]
        })
    }

    fn set_quiet<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::IntDur2, |v| {
            let mut reg = IntDur2::from_bytes([v]);
            reg.set_quiet(val);
            reg.into_bytes()[0]
        })
    }

    fn set_dur<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::IntDur2, |v| {
            let mut reg = IntDur2::from_bytes([v]);
            reg.set_dur(val);
            reg.into_bytes()[0]
        })
    }
}
