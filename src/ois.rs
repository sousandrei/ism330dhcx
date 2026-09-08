use embedded_hal::i2c::I2c;

use crate::Ism330Dhcx;
use crate::registers::{Ctrl1Ois, Ctrl2Ois, Ctrl3Ois, IntOis, Register};

/// OIS and auxiliary SPI configuration.
pub trait Ois {
    fn set_ois_spi2<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_gyro_scale<I2C>(&self, i2c: &mut I2C, scale: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_mode4<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_spi3<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_level1<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_high_pass<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_filter_type<I2C>(&self, i2c: &mut I2C, filter: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_filter_cutoff<I2C>(&self, i2c: &mut I2C, cutoff: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_accel_scale<I2C>(&self, i2c: &mut I2C, scale: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_self_test<I2C>(&self, i2c: &mut I2C, mode: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_ois_interrupt<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

macro_rules! ois_setters {
    ($(fn $method:ident, $register:ident, $config:ident, $setter:ident, $value:ty;)*) => {
        $(fn $method<I2C>(&self, i2c: &mut I2C, value: $value) -> Result<(), I2C::Error>
        where I2C: I2c {
            self.modify_reg(i2c, Register::$register, |raw| {
                let mut reg = $config::from_bytes([raw]);
                reg.$setter(value);
                reg.into_bytes()[0]
            })
        })*
    };
}

impl Ois for Ism330Dhcx {
    ois_setters! {
        fn set_ois_spi2, Ctrl1Ois, Ctrl1Ois, set_ois_en_spi2, bool;
        fn set_ois_gyro_scale, Ctrl1Ois, Ctrl1Ois, set_fs_g_ois, u8;
        fn set_ois_mode4, Ctrl1Ois, Ctrl1Ois, set_mode4_en, bool;
        fn set_ois_spi3, Ctrl1Ois, Ctrl1Ois, set_sim_ois, bool;
        fn set_ois_level1, Ctrl1Ois, Ctrl1Ois, set_lvl1_ois, bool;
        fn set_ois_high_pass, Ctrl2Ois, Ctrl2Ois, set_hp_en_ois, bool;
        fn set_ois_filter_type, Ctrl2Ois, Ctrl2Ois, set_ftype_ois, u8;
        fn set_ois_filter_cutoff, Ctrl2Ois, Ctrl2Ois, set_hpm_ois, u8;
        fn set_ois_accel_scale, Ctrl3Ois, Ctrl3Ois, set_fs_xl_ois, u8;
        fn set_ois_self_test, Ctrl3Ois, Ctrl3Ois, set_st_ois, u8;
        fn set_ois_interrupt, IntOis, IntOis, set_int2_drdy_ois, bool;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn enables_auxiliary_ois_spi() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl1Ois.addr()],
                vec![0x20],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl1Ois.addr(), 0x21]),
        ]);

        sensor.set_ois_spi2(&mut i2c, true).unwrap();
        i2c.done();
    }
}
