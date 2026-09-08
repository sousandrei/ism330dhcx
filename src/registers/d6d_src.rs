#![allow(non_snake_case)]
use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;
use modular_bitfield::{bitfield, specifiers::B1};
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct D6dSrc {
    pub xl: bool,
    pub xh: bool,
    pub yl: bool,
    pub yh: bool,
    pub zl: bool,
    pub zh: bool,
    pub d6d_ia: bool,
    pub __: B1,
}
impl Default for D6dSrc {
    fn default() -> Self {
        Self::new()
    }
}

pub trait D6dSrcConfig {
    fn get_d6d_src<I2C>(&self, i2c: &mut I2C) -> Result<D6dSrc, I2C::Error>
    where
        I2C: I2c;
}

impl D6dSrcConfig for Ism330Dhcx {
    fn get_d6d_src<I2C>(&self, i2c: &mut I2C) -> Result<D6dSrc, I2C::Error>
    where
        I2C: I2c,
    {
        Ok(D6dSrc::from_bytes([self.read_reg(i2c, Register::D6dSrc)?]))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(D6dSrc::default().into_bytes(), [0]);
        let mut r = D6dSrc::new();
        r.set_d6d_ia(true);
        assert_eq!(r.into_bytes(), [0x40]);
    }
}
