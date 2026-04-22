use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Wake-up interrupt source register (1Bh)
    pub struct WakeUpSrc(u8);
    impl Debug;
    /// Wakeup event detection status on Z-axis.
    pub z_wu, set_z_wu: 0;
    /// Wakeup event detection status on Y-axis.
    pub y_wu, set_y_wu: 1;
    /// Wakeup event detection status on X-axis.
    pub x_wu, set_x_wu: 2;
    /// Wakeup event detection status.
    pub wu_ia, set_wu_ia: 3;
    /// Sleep event status.
    pub sleep_state, set_sleep_state: 4;
    /// Free-fall event detection status.
    pub ff_ia, set_ff_ia: 5;
    /// Detects change event in activity/inactivity status.
    pub sleep_change_ia, set_sleep_change_ia: 6;
}

impl WakeUpSrc {
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

impl Default for WakeUpSrc {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for WAKE_UP_SRC register.
pub trait WakeUpSrcConfig {
    /// Read the wake-up source register to check for sleep/activity state.
    fn get_wake_up_src<I2C>(&self, i2c: &mut I2C) -> Result<WakeUpSrc, I2C::Error>
    where
        I2C: I2c;
}

impl WakeUpSrcConfig for Ism330Dhcx {
    fn get_wake_up_src<I2C>(&self, i2c: &mut I2C) -> Result<WakeUpSrc, I2C::Error>
    where
        I2C: I2c,
    {
        let v = self.read_reg(i2c, Register::WakeUpSrc)?;
        Ok(WakeUpSrc::from_bytes([v]))
    }
}
