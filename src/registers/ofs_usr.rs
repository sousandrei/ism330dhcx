use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

// TODO: one of this for each axis: X, Y, Z

bitfield! {
    /// Accelerometer user offset correction registers (73h - 75h)
    pub struct OfsUsr(u8);
    impl Debug;
    /// Accelerometer user offset correction value
    pub ofs_usr, set_ofs_usr: 7, 0;
}

impl OfsUsr {
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

impl Default for OfsUsr {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for OFS_USR registers.
pub trait OfsUsrConfig {
    /// Set accelerometer X-axis user offset correction.
    fn set_x_ofs_usr<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Set accelerometer Y-axis user offset correction.
    fn set_y_ofs_usr<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Set accelerometer Z-axis user offset correction.
    fn set_z_ofs_usr<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl OfsUsrConfig for Ism330Dhcx {
    fn set_x_ofs_usr<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::XOfsUsr, |v| {
            let mut reg = OfsUsr::from_bytes([v]);
            reg.set_ofs_usr(val);
            reg.into_bytes()[0]
        })
    }

    fn set_y_ofs_usr<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::YOfsUsr, |v| {
            let mut reg = OfsUsr::from_bytes([v]);
            reg.set_ofs_usr(val);
            reg.into_bytes()[0]
        })
    }

    fn set_z_ofs_usr<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::ZOfsUsr, |v| {
            let mut reg = OfsUsr::from_bytes([v]);
            reg.set_ofs_usr(val);
            reg.into_bytes()[0]
        })
    }
}
