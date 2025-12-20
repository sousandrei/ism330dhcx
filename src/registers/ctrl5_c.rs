use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

/// Linear acceleration sensor self-test mode selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum StXl {
    /// Normal mode
    Normal = 0b00,
    /// Positive sign self-test
    Positive = 0b01,
    /// Negative sign self-test
    Negative = 0b10,
}

impl From<u8> for StXl {
    fn from(val: u8) -> Self {
        match val {
            0b00 => StXl::Normal,
            0b01 => StXl::Positive,
            0b10 => StXl::Negative,
            _ => StXl::Normal,
        }
    }
}

impl From<StXl> for u8 {
    fn from(val: StXl) -> u8 {
        val as u8
    }
}

/// Angular rate sensor self-test mode selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum StG {
    /// Normal mode
    Normal = 0b00,
    /// Positive sign self-test
    Positive = 0b01,
    /// Negative sign self-test
    Negative = 0b11,
}

impl From<u8> for StG {
    fn from(val: u8) -> Self {
        match val {
            0b00 => StG::Normal,
            0b01 => StG::Positive,
            0b11 => StG::Negative,
            _ => StG::Normal,
        }
    }
}

impl From<StG> for u8 {
    fn from(val: StG) -> u8 {
        val as u8
    }
}

/// Circular burst-mode (rounding) read selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum Rounding {
    /// No rounding
    None = 0b00,
    /// Accelerometer only
    AccelOnly = 0b01,
    /// Gyroscope only
    GyroOnly = 0b10,
    /// Gyroscope + Accelerometer
    GyroAccel = 0b11,
}

impl From<u8> for Rounding {
    fn from(val: u8) -> Self {
        match val {
            0b00 => Rounding::None,
            0b01 => Rounding::AccelOnly,
            0b10 => Rounding::GyroOnly,
            0b11 => Rounding::GyroAccel,
            _ => Rounding::None,
        }
    }
}

impl From<Rounding> for u8 {
    fn from(val: Rounding) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Control register 5 (14h)
    pub struct Ctrl5C(u8);
    impl Debug;
    /// Linear acceleration sensor self-test enable.
    pub from into StXl, st_xl, set_st_xl: 1, 0;
    /// Angular rate sensor self-test enable.
    pub from into StG, st_g, set_st_g: 3, 2;
    /// Circular burst-mode (rounding) read of the output registers.
    pub from into Rounding, rounding, set_rounding: 6, 5;
}

impl Ctrl5C {
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

impl Default for Ctrl5C {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CTRL5_C register.
pub trait Ctrl5CConfig {
    /// Linear acceleration sensor self-test enable.
    fn set_st_xl<I2C>(&self, i2c: &mut I2C, val: StXl) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Angular rate sensor self-test enable.
    fn set_st_g<I2C>(&self, i2c: &mut I2C, val: StG) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Circular burst-mode (rounding) read of the output registers.
    fn set_rounding<I2C>(&self, i2c: &mut I2C, val: Rounding) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ctrl5CConfig for Ism330Dhcx {
    fn set_st_xl<I2C>(&self, i2c: &mut I2C, val: StXl) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl5C, |v| {
            let mut reg = Ctrl5C::from_bytes([v]);
            reg.set_st_xl(val);
            reg.into_bytes()[0]
        })
    }

    fn set_st_g<I2C>(&self, i2c: &mut I2C, val: StG) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl5C, |v| {
            let mut reg = Ctrl5C::from_bytes([v]);
            reg.set_st_g(val);
            reg.into_bytes()[0]
        })
    }

    fn set_rounding<I2C>(&self, i2c: &mut I2C, val: Rounding) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl5C, |v| {
            let mut reg = Ctrl5C::from_bytes([v]);
            reg.set_rounding(val);
            reg.into_bytes()[0]
        })
    }
}
