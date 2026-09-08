use crate::RegisterBus;

use crate::Ism330Dhcx;
use crate::registers::Register;

/// User offset access for the accelerometer axes.
pub trait Offsets {
    /// Set the signed X-axis user offset.
    fn set_user_offset_x<I2C>(&self, i2c: &mut I2C, offset: i8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set the signed Y-axis user offset.
    fn set_user_offset_y<I2C>(&self, i2c: &mut I2C, offset: i8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set the signed Z-axis user offset.
    fn set_user_offset_z<I2C>(&self, i2c: &mut I2C, offset: i8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the signed X-axis user offset.
    fn get_user_offset_x<I2C>(&self, i2c: &mut I2C) -> Result<i8, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the signed Y-axis user offset.
    fn get_user_offset_y<I2C>(&self, i2c: &mut I2C) -> Result<i8, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the signed Z-axis user offset.
    fn get_user_offset_z<I2C>(&self, i2c: &mut I2C) -> Result<i8, I2C::Error>
    where
        I2C: RegisterBus;
}

macro_rules! offset_accessors {
    ($set:ident, $get:ident, $register:ident) => {
        fn $set<I2C>(&self, i2c: &mut I2C, offset: i8) -> Result<(), I2C::Error>
        where
            I2C: RegisterBus,
        {
            self.write_reg(i2c, Register::$register, offset as u8)
        }

        fn $get<I2C>(&self, i2c: &mut I2C) -> Result<i8, I2C::Error>
        where
            I2C: RegisterBus,
        {
            Ok(self.read_reg(i2c, Register::$register)? as i8)
        }
    };
}

impl Offsets for Ism330Dhcx {
    offset_accessors!(set_user_offset_x, get_user_offset_x, XOfsUsr);
    offset_accessors!(set_user_offset_y, get_user_offset_y, YOfsUsr);
    offset_accessors!(set_user_offset_z, get_user_offset_z, ZOfsUsr);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn writes_signed_user_offset() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write(
            DEFAULT_I2C_ADDRESS,
            vec![Register::YOfsUsr.addr(), 0xa6],
        )]);

        sensor.set_user_offset_y(&mut i2c, -90).unwrap();
        i2c.done();
    }

    #[test]
    fn reads_signed_user_offset() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::ZOfsUsr.addr()],
            vec![0x7f],
        )]);

        assert_eq!(sensor.get_user_offset_z(&mut i2c).unwrap(), 127);
        i2c.done();
    }
}
