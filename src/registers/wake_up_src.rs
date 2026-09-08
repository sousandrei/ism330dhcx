use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;
use modular_bitfield::{bitfield, specifiers::B1};
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct WakeUpSrc {
    pub sleep_change_ia: bool,
    pub ff_ia: bool,
    pub sleep_state: bool,
    pub wu_ia: bool,
    pub x_wu: bool,
    pub y_wu: bool,
    pub z_wu: bool,
    #[skip]
    pub __: B1,
}
impl Default for WakeUpSrc {
    fn default() -> Self {
        Self::new()
    }
}

pub trait WakeUpSrcConfig {
    fn get_wake_up_src<I2C>(&self, i2c: &mut I2C) -> Result<WakeUpSrc, I2C::Error>
    where
        I2C: I2c;
}

impl WakeUpSrcConfig for Ism330Dhcx {
    fn get_wake_up_src<I2C>(&self, i2c: &mut I2C) -> Result<WakeUpSrc, I2C::Error>
    where
        I2C: I2c,
    {
        Ok(WakeUpSrc::from_bytes([
            self.read_reg(i2c, Register::WakeUpSrc)?
        ]))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(WakeUpSrc::default().into_bytes(), [0]);
        let mut r = WakeUpSrc::new();
        r.set_wu_ia(true);
        assert_eq!(r.into_bytes(), [8]);
        r.set_sleep_change_ia(true);
        r.set_ff_ia(true);
        r.set_sleep_state(true);
        r.set_x_wu(true);
        r.set_y_wu(true);
        r.set_z_wu(true);
        assert_eq!(r.into_bytes(), [0x7f]);
    }
}
