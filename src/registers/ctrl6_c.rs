use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

/// Gyroscope low-pass filter (LPF1) bandwidth selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum Ftype {
    /// Bandwidth 0
    Bw0 = 0b000,
    /// Bandwidth 1
    Bw1 = 0b001,
    /// Bandwidth 2
    Bw2 = 0b010,
    /// Bandwidth 3
    Bw3 = 0b011,
    /// Bandwidth 4
    Bw4 = 0b100,
    /// Bandwidth 5
    Bw5 = 0b101,
    /// Bandwidth 6
    Bw6 = 0b110,
    /// Bandwidth 7
    Bw7 = 0b111,
}

impl From<u8> for Ftype {
    fn from(val: u8) -> Self {
        match val {
            0b000 => Ftype::Bw0,
            0b001 => Ftype::Bw1,
            0b010 => Ftype::Bw2,
            0b011 => Ftype::Bw3,
            0b100 => Ftype::Bw4,
            0b101 => Ftype::Bw5,
            0b110 => Ftype::Bw6,
            0b111 => Ftype::Bw7,
            _ => Ftype::Bw0,
        }
    }
}

impl From<Ftype> for u8 {
    fn from(val: Ftype) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Control register 6 (15h)
    pub struct Ctrl6C(u8);
    impl Debug;
    /// Gyroscope low-pass filter (LPF1) bandwidth selection.
    pub from into Ftype, ftype, set_ftype: 2, 0;
    /// Weight of XL user offset bits.
    pub usr_off_w, set_usr_off_w: 3;
    /// Disables high-performance operating mode for accelerometer.
    pub xl_hm_mode, set_xl_hm_mode: 4;
    /// Enables DEN level-sensitive latched mode.
    pub lvl2_en, set_lvl2_en: 5;
    /// Enables DEN data level-sensitive trigger mode.
    pub lvl1_en, set_lvl1_en: 6;
    /// Enables DEN data edge-sensitive trigger mode.
    pub trig_en, set_trig_en: 7;
}

impl Ctrl6C {
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

impl Default for Ctrl6C {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for CTRL6_C register.
pub trait Ctrl6CConfig {
    /// Gyroscope low-pass filter (LPF1) bandwidth selection.
    fn set_ftype<I2C>(&self, i2c: &mut I2C, val: Ftype) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Weight of XL user offset bits.
    fn set_usr_off_w<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Disables high-performance operating mode for accelerometer.
    fn set_xl_hm_mode<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enables DEN level-sensitive latched mode.
    fn set_lvl2_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enables DEN data level-sensitive trigger mode.
    fn set_lvl1_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enables DEN data edge-sensitive trigger mode.
    fn set_trig_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ctrl6CConfig for Ism330Dhcx {
    fn set_ftype<I2C>(&self, i2c: &mut I2C, val: Ftype) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl6C, |v| {
            let mut reg = Ctrl6C::from_bytes([v]);
            reg.set_ftype(val);
            reg.into_bytes()[0]
        })
    }

    fn set_usr_off_w<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl6C, |v| {
            let mut reg = Ctrl6C::from_bytes([v]);
            reg.set_usr_off_w(val);
            reg.into_bytes()[0]
        })
    }

    fn set_xl_hm_mode<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl6C, |v| {
            let mut reg = Ctrl6C::from_bytes([v]);
            reg.set_xl_hm_mode(val);
            reg.into_bytes()[0]
        })
    }

    fn set_lvl2_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl6C, |v| {
            let mut reg = Ctrl6C::from_bytes([v]);
            reg.set_lvl2_en(val);
            reg.into_bytes()[0]
        })
    }

    fn set_lvl1_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl6C, |v| {
            let mut reg = Ctrl6C::from_bytes([v]);
            reg.set_lvl1_en(val);
            reg.into_bytes()[0]
        })
    }

    fn set_trig_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl6C, |v| {
            let mut reg = Ctrl6C::from_bytes([v]);
            reg.set_trig_en(val);
            reg.into_bytes()[0]
        })
    }
}
