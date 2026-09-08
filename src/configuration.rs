use embedded_hal::i2c::I2c;

use crate::Ism330Dhcx;
use crate::registers::{Ctrl4C, Ctrl5C, Ctrl6C, Ctrl7G, Ctrl10C, PinCtrl, Register, Rounding};

/// Core sensor configuration methods.
pub trait Configuration {
    /// Set a core boolean configuration field.
    fn set_core_field<I2C>(
        &self,
        i2c: &mut I2C,
        field: CoreField,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Set output-register rounding mode.
    fn set_rounding<I2C>(&self, i2c: &mut I2C, rounding: Rounding) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Get the configured output-register rounding mode.
    fn get_rounding<I2C>(&self, i2c: &mut I2C) -> Result<Rounding, I2C::Error>
    where
        I2C: I2c;

    /// Enable or disable timestamping.
    fn set_timestamp_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enable or disable the SDO pin pull-up.
    fn set_sdo_pu_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enable or disable the OIS auxiliary pin pull-down.
    fn set_ois_pu_dis<I2C>(&self, i2c: &mut I2C, disable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

/// Boolean fields in the core control registers.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CoreField {
    /// Enable accelerometer DEN stamping.
    DenXlEn,
    /// Enable gyroscope sleep mode.
    SleepG,
    /// Disable the I2C interface.
    I2cDisable,
    /// Mask data-ready interrupts.
    DrdyMask,
    /// Route INT2 events to INT1.
    Int2OnInt1,
    /// Set DEN active level.
    DenLh,
    /// Enable accelerometer high-performance mode.
    XlHmMode,
    /// Enable user offset weighting.
    UsrOffW,
    /// Enable OIS output.
    OisOn,
    /// Enable OIS control through the primary interface.
    OisOnEn,
    /// Include user offsets in output.
    UsrOffOnOut,
    /// Enable gyroscope high-pass filtering.
    HpEnG,
    /// Set gyroscope high-performance mode disable bit.
    GHmMode,
    /// Set DEN active level in `CTRL10_C`.
    DenLhCore,
    /// Reset the step counter.
    PedometerReset,
    /// Enable significant motion detection.
    SignificantMotion,
    /// Enable embedded functions.
    EmbeddedFunctions,
}

impl Configuration for Ism330Dhcx {
    fn set_core_field<I2C>(
        &self,
        i2c: &mut I2C,
        field: CoreField,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        match field {
            CoreField::DenXlEn => self.modify_reg(i2c, Register::Ctrl4C, |value| {
                let mut reg = Ctrl4C::from_bytes([value]);
                reg.set_den_xl_en(enable);
                reg.into_bytes()[0]
            }),
            CoreField::SleepG => self.modify_reg(i2c, Register::Ctrl4C, |value| {
                let mut reg = Ctrl4C::from_bytes([value]);
                reg.set_sleep_g(enable);
                reg.into_bytes()[0]
            }),
            CoreField::I2cDisable => self.modify_reg(i2c, Register::Ctrl4C, |value| {
                let mut reg = Ctrl4C::from_bytes([value]);
                reg.set_i2c_disable(enable);
                reg.into_bytes()[0]
            }),
            CoreField::DrdyMask => self.modify_reg(i2c, Register::Ctrl4C, |value| {
                let mut reg = Ctrl4C::from_bytes([value]);
                reg.set_drdy_mask(enable);
                reg.into_bytes()[0]
            }),
            CoreField::Int2OnInt1 => self.modify_reg(i2c, Register::Ctrl4C, |value| {
                let mut reg = Ctrl4C::from_bytes([value]);
                reg.set_int2_on_int1(enable);
                reg.into_bytes()[0]
            }),
            CoreField::DenLh => self.modify_reg(i2c, Register::Ctrl4C, |value| {
                let mut reg = Ctrl4C::from_bytes([value]);
                reg.set_den_lh(enable);
                reg.into_bytes()[0]
            }),
            CoreField::XlHmMode => self.modify_reg(i2c, Register::Ctrl6C, |value| {
                let mut reg = Ctrl6C::from_bytes([value]);
                reg.set_xl_hm_mode(enable);
                reg.into_bytes()[0]
            }),
            CoreField::UsrOffW => self.modify_reg(i2c, Register::Ctrl6C, |value| {
                let mut reg = Ctrl6C::from_bytes([value]);
                reg.set_usr_off_w(enable);
                reg.into_bytes()[0]
            }),
            CoreField::OisOn => self.modify_reg(i2c, Register::Ctrl7G, |value| {
                let mut reg = Ctrl7G::from_bytes([value]);
                reg.set_ois_on(enable);
                reg.into_bytes()[0]
            }),
            CoreField::OisOnEn => self.modify_reg(i2c, Register::Ctrl7G, |value| {
                let mut reg = Ctrl7G::from_bytes([value]);
                reg.set_ois_on_en(enable);
                reg.into_bytes()[0]
            }),
            CoreField::UsrOffOnOut => self.modify_reg(i2c, Register::Ctrl7G, |value| {
                let mut reg = Ctrl7G::from_bytes([value]);
                reg.set_usr_off_on_out(enable);
                reg.into_bytes()[0]
            }),
            CoreField::HpEnG => self.modify_reg(i2c, Register::Ctrl7G, |value| {
                let mut reg = Ctrl7G::from_bytes([value]);
                reg.set_hp_en_g(enable);
                reg.into_bytes()[0]
            }),
            CoreField::GHmMode => self.modify_reg(i2c, Register::Ctrl7G, |value| {
                let mut reg = Ctrl7G::from_bytes([value]);
                reg.set_g_hm_mode(enable);
                reg.into_bytes()[0]
            }),
            CoreField::DenLhCore => self.modify_reg(i2c, Register::Ctrl10C, |value| {
                let mut reg = Ctrl10C::from_bytes([value]);
                reg.set_den_lh(enable);
                reg.into_bytes()[0]
            }),
            CoreField::PedometerReset => self.modify_reg(i2c, Register::Ctrl10C, |value| {
                let mut reg = Ctrl10C::from_bytes([value]);
                reg.set_pedo_rst_step(enable);
                reg.into_bytes()[0]
            }),
            CoreField::SignificantMotion => self.modify_reg(i2c, Register::Ctrl10C, |value| {
                let mut reg = Ctrl10C::from_bytes([value]);
                reg.set_sign_motion_en(enable);
                reg.into_bytes()[0]
            }),
            CoreField::EmbeddedFunctions => self.modify_reg(i2c, Register::Ctrl10C, |value| {
                let mut reg = Ctrl10C::from_bytes([value]);
                reg.set_func_en(enable);
                reg.into_bytes()[0]
            }),
        }
    }

    fn set_rounding<I2C>(&self, i2c: &mut I2C, rounding: Rounding) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl5C, |value| {
            let mut reg = Ctrl5C::from_bytes([value]);
            reg.set_rounding(rounding);
            reg.into_bytes()[0]
        })
    }

    fn get_rounding<I2C>(&self, i2c: &mut I2C) -> Result<Rounding, I2C::Error>
    where
        I2C: I2c,
    {
        Ok(Ctrl5C::from_bytes([self.read_reg(i2c, Register::Ctrl5C)?]).rounding())
    }

    fn set_timestamp_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl10C, |value| {
            let mut reg = Ctrl10C::from_bytes([value]);
            reg.set_timestamp_en(enable);
            reg.into_bytes()[0]
        })
    }

    fn set_sdo_pu_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::PinCtrl, |value| {
            let mut reg = PinCtrl::from_bytes([value]);
            reg.set_sdo_pu_en(enable);
            reg.into_bytes()[0]
        })
    }

    fn set_ois_pu_dis<I2C>(&self, i2c: &mut I2C, disable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::PinCtrl, |value| {
            let mut reg = PinCtrl::from_bytes([value]);
            reg.set_ois_pu_dis(disable);
            reg.into_bytes()[0]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn core_configuration_preserves_unrelated_bits() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl4C.addr()],
                vec![0x80],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl4C.addr(), 0x82]),
            Transaction::write_read(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl5C.addr()], vec![0]),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl5C.addr(), 0x40]),
        ]);

        sensor
            .set_core_field(&mut i2c, CoreField::SleepG, true)
            .unwrap();
        sensor.set_rounding(&mut i2c, Rounding::Gyroscope).unwrap();
        i2c.done();
    }

    #[test]
    fn pin_configuration_preserves_unrelated_bits() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::PinCtrl.addr()],
                vec![0x20],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::PinCtrl.addr(), 0x60]),
        ]);

        sensor.set_sdo_pu_en(&mut i2c, true).unwrap();
        i2c.done();
    }
}
