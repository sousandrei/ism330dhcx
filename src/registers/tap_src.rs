use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;
use modular_bitfield::{bitfield, specifiers::B1};
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TapSrc {
    pub z_tap: bool,
    pub y_tap: bool,
    pub x_tap: bool,
    pub tap_sign: bool,
    pub double_tap: bool,
    pub single_tap: bool,
    pub tap_ia: bool,
    #[skip]
    pub __: B1,
}
impl Default for TapSrc {
    fn default() -> Self {
        Self::new()
    }
}

pub trait TapSrcConfig {
    fn get_tap_src<I2C>(&self, i2c: &mut I2C) -> Result<TapSrc, I2C::Error>
    where
        I2C: I2c;
}

impl TapSrcConfig for Ism330Dhcx {
    fn get_tap_src<I2C>(&self, i2c: &mut I2C) -> Result<TapSrc, I2C::Error>
    where
        I2C: I2c,
    {
        Ok(TapSrc::from_bytes([self.read_reg(i2c, Register::TapSrc)?]))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(TapSrc::default().into_bytes(), [0]);
        let mut r = TapSrc::new();
        r.set_tap_ia(true);
        assert_eq!(r.into_bytes(), [0x40]);
        r.set_single_tap(true);
        r.set_double_tap(true);
        r.set_tap_sign(true);
        r.set_x_tap(true);
        r.set_y_tap(true);
        r.set_z_tap(true);
        assert_eq!(r.into_bytes(), [0x7f]);
    }
}
