use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;
use modular_bitfield::{bitfield, specifiers::B1};
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AllIntSrc {
    pub ff_ia: bool,
    pub wu_ia: bool,
    pub single_tap: bool,
    pub double_tap: bool,
    pub d6d_ia: bool,
    pub sleep_change_ia: bool,
    #[skip]
    pub __: B1,
    pub timestamp_endcount: bool,
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
        r.set_timestamp_endcount(true);
        assert_eq!(r.into_bytes(), [0x80]);

        r.set_sleep_change_ia(true);
        r.set_d6d_ia(true);
        r.set_double_tap(true);
        r.set_single_tap(true);
        r.set_wu_ia(true);
        r.set_ff_ia(true);
        assert_eq!(r.into_bytes(), [0xbf]);
    }
}
