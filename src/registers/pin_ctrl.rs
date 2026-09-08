#![allow(unused_parens)]
use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;
use modular_bitfield::{bitfield, specifiers::B6};

/// SDO, OCS_AUX, SDO_AUX pins pull-up enable/disable register. (02h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct PinCtrl {
    /// Reserved bits, must be set to 1 for correct operation.
    pub reserved: B6,
    /// Enable pull-up on SDO pin.
    pub sdo_pu_en: bool,
    /// Disable pull-up on both OCS_Aux and SDO_Aux pins.
    pub ois_pu_dis: bool,
}

impl Default for PinCtrl {
    fn default() -> Self {
        // Datasheet says bits 5:0 must be set to 1.
        Self::from_bytes([0x3F])
    }
}

pub trait PinCtrlExt {
    /// Enable pull-up on SDO pin.
    fn set_sdo_pu_en<I2C>(&self, i2c: &mut I2C, en: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Disable pull-up on both OCS_Aux and SDO_Aux pins.
    fn set_ois_pu_dis<I2C>(&self, i2c: &mut I2C, dis: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl PinCtrlExt for Ism330Dhcx {
    fn set_sdo_pu_en<I2C>(&self, i2c: &mut I2C, en: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::PinCtrl, |v| {
            let mut reg = PinCtrl::from_bytes([v]);
            reg.set_sdo_pu_en(en);
            reg.into_bytes()[0]
        })
    }

    fn set_ois_pu_dis<I2C>(&self, i2c: &mut I2C, dis: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::PinCtrl, |v| {
            let mut reg = PinCtrl::from_bytes([v]);
            reg.set_ois_pu_dis(dis);
            reg.into_bytes()[0]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_ctrl() {
        let mut reg = PinCtrl::default();
        assert_eq!(reg.into_bytes()[0], 0x3F);

        reg.set_sdo_pu_en(true);
        assert_eq!(reg.into_bytes()[0], 0x7F);

        reg.set_ois_pu_dis(true);
        assert_eq!(reg.into_bytes()[0], 0xFF);
    }
}
