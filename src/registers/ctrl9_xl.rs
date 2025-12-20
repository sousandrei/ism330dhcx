use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Control register 9 (XL).
    pub struct Ctrl9Xl(u8);
    impl Debug;
    /// Device configuration.
    pub device_conf, set_device_conf: 1;
    /// DEN active level.
    pub den_lh, set_den_lh: 2;
    /// DEN stamping on accelerometer axis.
    pub den_xl_en, set_den_xl_en: 3;
    /// DEN stamping on gyroscope axis.
    pub den_xl_g, set_den_xl_g: 4;
    /// DEN value stored in LSB of Z-axis.
    pub den_z, set_den_z: 5;
    /// DEN value stored in LSB of Y-axis.
    pub den_y, set_den_y: 6;
    /// DEN value stored in LSB of X-axis.
    pub den_x, set_den_x: 7;
}

impl Ctrl9Xl {
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

impl Default for Ctrl9Xl {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CTRL9_XL register.
pub trait Ctrl9XlConfig {
    /// Device configuration.
    fn set_device_conf<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// DEN active level.
    fn set_den_lh<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// DEN stamping on accelerometer axis.
    fn set_den_xl_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// DEN stamping on gyroscope axis.
    fn set_den_xl_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// DEN value stored in LSB of Z-axis.
    fn set_den_z<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// DEN value stored in LSB of Y-axis.
    fn set_den_y<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// DEN value stored in LSB of X-axis.
    fn set_den_x<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ctrl9XlConfig for Ism330Dhcx {
    fn set_device_conf<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_device_conf(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_lh<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_den_lh(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_xl_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_den_xl_en(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_xl_g<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_den_xl_g(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_z<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_den_z(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_y<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_den_y(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_x<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = Ctrl9Xl::from_bytes([v]);
            reg.set_den_x(val);
            reg.into_bytes()[0]
        })
    }
}
