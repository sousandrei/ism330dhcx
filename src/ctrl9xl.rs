use core::fmt;

use crate::Register;

/// The CTRL9_XL (control 9) register
pub struct Ctrl9Xl {
    pub address: u8,
    value: u8,
}

impl fmt::Display for Ctrl9Xl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Binary for Ctrl9Xl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.value)
    }
}

impl fmt::LowerHex for Ctrl9Xl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.value, f)
    }
}

pub const ADDR: u8 = 0x18u8;

/// DEN value stored in LSB of Z-axis.
///
/// Default value: 1
///
///(0: DEN not stored in Z-axis LSB; 1: DEN stored in Z-axis LSB)
pub const DEN_Z: u8 = 7;

/// DEN value stored in LSB of Y-axis.
///
/// Default value: 1
///
///(0: DEN not stored in Y-axis LSB; 1: DEN stored in Y-axis LSB)
pub const DEN_Y: u8 = 6;

/// DEN value stored in LSB of X-axis.
///
/// Default value: 1
///
///(0: DEN not stored in X-axis LSB; 1: DEN stored in X-axis LSB)
pub const DEN_X: u8 = 5;

/// DEN stamping sensor selection.
///
/// Default value: 0
///
/// (0: DEN pin info stamped in the gyroscope axis selected by bits \[7:5\]; 1: DEN pin info stamped in the accelerometer axis selected by bits \[7:5\])
pub const DEN_XL_G: u8 = 4;

/// Extends DEN functionality to accelerometer sensor.
///
/// Default value: 0
///
/// (0: disabled; 1: enabled)
pub const DEN_XL_EN: u8 = 3;

/// DEN active level configuration.
///
/// Default value: 0
///
/// (0: active low; 1: active high)
pub const DEN_LH: u8 = 2;

/// Enables the proper device configuration.
///
/// Default value: 0
///
/// (0: default; 1: enabled)
///
/// It is recommended to always set this bit to 1 during device configuration.
pub const DEVICE_CONF: u8 = 1;

impl Register for Ctrl9Xl {}

impl Ctrl9Xl {
    pub fn new(value: u8, address: u8) -> Self {
        Ctrl9Xl { address, value }
    }

    pub fn den_x(&mut self) -> bool {
        self.value & (1 << DEN_X) != 0
    }

    pub fn set_den_x<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.value &= !(1 << DEN_X);
        self.value |= (value as u8) << DEN_X;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn den_y(&mut self) -> bool {
        self.value & (1 << DEN_Y) != 0
    }

    pub fn set_den_y<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.value &= !(1 << DEN_Y);
        self.value |= (value as u8) << DEN_Y;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn den_z(&mut self) -> bool {
        self.value & (1 << DEN_Z) != 0
    }

    pub fn set_den_z<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.value &= !(1 << DEN_Z);
        self.value |= (value as u8) << DEN_Z;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn den_xl_g(&mut self) -> bool {
        self.value & (1 << DEN_XL_G) != 0
    }

    pub fn set_den_xl_g<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.value &= !(1 << DEN_XL_G);
        self.value |= (value as u8) << DEN_XL_G;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn den_xl_en(&mut self) -> bool {
        self.value & (1 << DEN_XL_EN) != 0
    }

    pub fn set_den_xl_en<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.value &= !(1 << DEN_XL_EN);
        self.value |= (value as u8) << DEN_XL_EN;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn den_lh(&mut self) -> bool {
        self.value & (1 << DEN_LH) != 0
    }

    pub fn set_den_lh<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.value &= !(1 << DEN_LH);
        self.value |= (value as u8) << DEN_LH;
        self.write(i2c, self.address, ADDR, self.value)
    }

    pub fn device_conf(&mut self) -> bool {
        self.value & (1 << DEVICE_CONF) != 0
    }

    pub fn set_device_conf<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        self.value &= !(1 << DEVICE_CONF);
        self.value |= (value as u8) << DEVICE_CONF;
        self.write(i2c, self.address, ADDR, self.value)
    }
}
