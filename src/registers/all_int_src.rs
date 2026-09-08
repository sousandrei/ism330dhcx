#![allow(non_snake_case)]
use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;
use modular_bitfield::{bitfield, specifiers::B1};
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct AllIntSrc {
    pub ff_ia: bool,
    pub wu_ia: bool,
    pub single_tap: bool,
    pub double_tap: bool,
    pub d6d_ia: bool,
    pub sleep_change_ia: bool,
    pub tilt_ia: bool,
    pub __: B1,
}
impl Default for AllIntSrc {
    fn default() -> Self {
        Self::new()
    }
}

pub trait AllIntSrcConfig {
    fn get_all_int_src<I2C>(&self, i2c: &mut I2C) -> Result<AllIntSrc, I2C::Error>
    where
        I2C: I2c;
}

impl AllIntSrcConfig for Ism330Dhcx {
    fn get_all_int_src<I2C>(&self, i2c: &mut I2C) -> Result<AllIntSrc, I2C::Error>
    where
        I2C: I2c,
    {
        Ok(AllIntSrc::from_bytes([
            self.read_reg(i2c, Register::AllIntSrc)?
        ]))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(AllIntSrc::default().into_bytes(), [0]);
        let mut r = AllIntSrc::new();
        r.set_tilt_ia(true);
        assert_eq!(r.into_bytes(), [0x40]);
    }
}
