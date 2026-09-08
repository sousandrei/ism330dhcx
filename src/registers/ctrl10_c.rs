use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;
use modular_bitfield::{bitfield, specifiers::B3};

/// Control register 10.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl10C {
    #[skip]
    pub reserved: B3,
    pub den_lh: bool,
    pub pedo_rst_step: bool,
    pub timestamp_en: bool,
    pub sign_motion_en: bool,
    pub func_en: bool,
}
impl Default for Ctrl10C {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Ctrl10CConfig {
    fn set_timestamp_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ctrl10CConfig for Ism330Dhcx {
    fn set_timestamp_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl10C, |v| {
            let mut reg = Ctrl10C::from_bytes([v]);
            reg.set_timestamp_en(enable);
            reg.into_bytes()[0]
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Ctrl10C::default().into_bytes(), [0]);
        let mut r = Ctrl10C::new();
        r.set_timestamp_en(true);
        assert_eq!(r.into_bytes(), [0x20]);
    }
}
