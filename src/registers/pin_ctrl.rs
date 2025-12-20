use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// SDO, OCS_AUX, SDO_AUX pins pull-up enable/disable register (02h)
    pub struct PinCtrl(u8);
    impl Debug;
    /// Enable pull-up on SDO pin.
    pub sdo_pu_en, set_sdo_pu_en: 6;
    /// Disable pull-up on both OCS_Aux and SDO_Aux pins.
    pub ois_pu_dis, set_ois_pu_dis: 7;
}

impl PinCtrl {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
    // Added for compatibility with original code's custom Default
    pub fn with_ois_pu_dis(mut self, val: bool) -> Self {
        self.set_ois_pu_dis(val);
        self
    }
    pub fn with_sdo_pu_en(mut self, val: bool) -> Self {
        self.set_sdo_pu_en(val);
        self
    }
}

impl Default for PinCtrl {
    fn default() -> Self {
        Self::new().with_ois_pu_dis(false).with_sdo_pu_en(false)
    }
}

/// Configuration methods for PIN_CTRL register.
pub trait PinCtrlConfig {
    /// Enable pull-up on SDO pin.
    fn set_sdo_pu_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Disable pull-up on both OCS_Aux and SDO_Aux pins.
    fn set_ois_pu_dis<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl PinCtrlConfig for Ism330Dhcx {
    fn set_sdo_pu_en<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::PinCtrl, |v| {
            let mut reg = PinCtrl::from_bytes([v]);
            reg.set_sdo_pu_en(val);
            reg.into_bytes()[0]
        })
    }

    fn set_ois_pu_dis<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::PinCtrl, |v| {
            let mut reg = PinCtrl::from_bytes([v]);
            reg.set_ois_pu_dis(val);
            reg.into_bytes()[0]
        })
    }
}
