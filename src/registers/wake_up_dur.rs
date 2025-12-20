use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Free-fall, wakeup and sleep mode functions duration setting register (5Ch)
    pub struct WakeUpDur(u8);
    impl Debug;
    /// Duration to go in sleep mode.
    pub sleep_dur, set_sleep_dur: 3, 0;
    /// Weight of 1 LSB of wakeup threshold.
    pub wake_ths_w, set_wake_ths_w: 4;
    /// Wake up duration event.
    pub wake_dur, set_wake_dur: 6, 5;
    /// Free fall duration event (MSB).
    pub ff_dur5, set_ff_dur5: 7;
}

impl WakeUpDur {
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

impl Default for WakeUpDur {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for WAKE_UP_DUR register.
pub trait WakeUpDurConfig {
    /// Duration to go in sleep mode.
    fn set_sleep_dur<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Weight of 1 LSB of wakeup threshold.
    fn set_wake_ths_w<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Wake up duration event.
    fn set_wake_dur<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Free fall duration event (MSB).
    fn set_ff_dur5<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl WakeUpDurConfig for Ism330Dhcx {
    fn set_sleep_dur<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::WakeUpDur, |v| {
            let mut reg = WakeUpDur::from_bytes([v]);
            reg.set_sleep_dur(val);
            reg.into_bytes()[0]
        })
    }

    fn set_wake_ths_w<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::WakeUpDur, |v| {
            let mut reg = WakeUpDur::from_bytes([v]);
            reg.set_wake_ths_w(val);
            reg.into_bytes()[0]
        })
    }

    fn set_wake_dur<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::WakeUpDur, |v| {
            let mut reg = WakeUpDur::from_bytes([v]);
            reg.set_wake_dur(val);
            reg.into_bytes()[0]
        })
    }

    fn set_ff_dur5<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::WakeUpDur, |v| {
            let mut reg = WakeUpDur::from_bytes([v]);
            reg.set_ff_dur5(val);
            reg.into_bytes()[0]
        })
    }
}
