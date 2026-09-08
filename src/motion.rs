use crate::RegisterBus;

use crate::Ism330Dhcx;
use crate::registers::{
    Ctrl10C, FreeFall, IntDur2, Md1Cfg, Md2Cfg, Register, TapCfg0, TapCfg1, TapCfg2, TapThs6d,
    WakeUpDur, WakeUpThs,
};

/// Tap, orientation, wake-up, and free-fall configuration.
pub trait Motion {
    fn set_tap_x<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_y<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_z<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_lir<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_slope_fds<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_sleep_status_on_int<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_int_clear_on_read<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_threshold_x<I2C>(&self, i2c: &mut I2C, threshold: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_priority<I2C>(&self, i2c: &mut I2C, priority: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_threshold_y<I2C>(&self, i2c: &mut I2C, threshold: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_inactivity<I2C>(&self, i2c: &mut I2C, inactivity: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_interrupts<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_threshold_z<I2C>(&self, i2c: &mut I2C, threshold: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_6d_threshold<I2C>(&self, i2c: &mut I2C, threshold: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_4d<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_shock<I2C>(&self, i2c: &mut I2C, duration: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_quiet<I2C>(&self, i2c: &mut I2C, duration: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tap_duration<I2C>(&self, i2c: &mut I2C, duration: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_wake_up_threshold<I2C>(&self, i2c: &mut I2C, threshold: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_wake_up_user_offset<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_wake_up_single_double_tap<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_sleep_duration<I2C>(&self, i2c: &mut I2C, duration: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_wake_threshold_weight<I2C>(&self, i2c: &mut I2C, weight: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_wake_duration<I2C>(&self, i2c: &mut I2C, duration: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_free_fall_duration<I2C>(&self, i2c: &mut I2C, duration: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_free_fall_threshold<I2C>(&self, i2c: &mut I2C, threshold: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_significant_motion<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_sleep_change<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_single_tap<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_wake_up<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_free_fall<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_double_tap<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_6d<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_sensor_hub<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_sleep_change<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_single_tap<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_wake_up<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_free_fall<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_double_tap<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_6d<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_timestamp<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
}

macro_rules! motion_setters {
    ($(fn $method:ident, $register:ident, $config:ident, $setter:ident, $value:ty;)*) => {
        $(
            fn $method<I2C>(&self, i2c: &mut I2C, value: $value) -> Result<(), I2C::Error>
            where
                I2C: RegisterBus,
            {
                self.modify_reg(i2c, Register::$register, |raw| {
                    let mut reg = $config::from_bytes([raw]);
                    reg.$setter(value);
                    reg.into_bytes()[0]
                })
            }
        )*
    };
}

impl Motion for Ism330Dhcx {
    motion_setters! {
        fn set_tap_x, TapCfg0, TapCfg0, set_tap_x_en, bool;
        fn set_tap_y, TapCfg0, TapCfg0, set_tap_y_en, bool;
        fn set_tap_z, TapCfg0, TapCfg0, set_tap_z_en, bool;
        fn set_tap_lir, TapCfg0, TapCfg0, set_lir, bool;
        fn set_tap_slope_fds, TapCfg0, TapCfg0, set_slope_fds, bool;
        fn set_tap_sleep_status_on_int, TapCfg0, TapCfg0, set_sleep_status_on_int, bool;
        fn set_tap_int_clear_on_read, TapCfg0, TapCfg0, set_int_clr_on_read, bool;
        fn set_tap_threshold_x, TapCfg1, TapCfg1, set_tap_ths_x, u8;
        fn set_tap_priority, TapCfg1, TapCfg1, set_tap_priority, u8;
        fn set_tap_threshold_y, TapCfg2, TapCfg2, set_tap_ths_y, u8;
        fn set_tap_inactivity, TapCfg2, TapCfg2, set_inact_en, u8;
        fn set_tap_interrupts, TapCfg2, TapCfg2, set_interrupts_enable, bool;
        fn set_tap_threshold_z, TapThs6d, TapThs6d, set_tap_ths_z, u8;
        fn set_6d_threshold, TapThs6d, TapThs6d, set_sixd_ths, u8;
        fn set_4d, TapThs6d, TapThs6d, set_d4d_en, bool;
        fn set_tap_shock, IntDur2, IntDur2, set_shock, u8;
        fn set_tap_quiet, IntDur2, IntDur2, set_quiet, u8;
        fn set_tap_duration, IntDur2, IntDur2, set_dur, u8;
        fn set_wake_up_threshold, WakeUpThs, WakeUpThs, set_wk_ths, u8;
        fn set_wake_up_user_offset, WakeUpThs, WakeUpThs, set_usr_off_on_wu, bool;
        fn set_wake_up_single_double_tap, WakeUpThs, WakeUpThs, set_single_double_tap, bool;
        fn set_sleep_duration, WakeUpDur, WakeUpDur, set_sleep_dur, u8;
        fn set_wake_threshold_weight, WakeUpDur, WakeUpDur, set_wake_ths_w, bool;
        fn set_wake_duration, WakeUpDur, WakeUpDur, set_wake_dur, u8;
        fn set_free_fall_duration, FreeFall, FreeFall, set_ff_dur, u8;
        fn set_free_fall_threshold, FreeFall, FreeFall, set_ff_ths, u8;
        fn set_significant_motion, Ctrl10C, Ctrl10C, set_sign_motion_en, bool;
        fn set_int1_sleep_change, Md1Cfg, Md1Cfg, set_int1_sleep_change, bool;
        fn set_int1_single_tap, Md1Cfg, Md1Cfg, set_int1_single_tap, bool;
        fn set_int1_wake_up, Md1Cfg, Md1Cfg, set_int1_wu, bool;
        fn set_int1_free_fall, Md1Cfg, Md1Cfg, set_int1_ff, bool;
        fn set_int1_double_tap, Md1Cfg, Md1Cfg, set_int1_double_tap, bool;
        fn set_int1_6d, Md1Cfg, Md1Cfg, set_int1_6d, bool;
        fn set_int1_sensor_hub, Md1Cfg, Md1Cfg, set_int1_shub, bool;
        fn set_int2_sleep_change, Md2Cfg, Md2Cfg, set_int2_sleep_change, bool;
        fn set_int2_single_tap, Md2Cfg, Md2Cfg, set_int2_single_tap, bool;
        fn set_int2_wake_up, Md2Cfg, Md2Cfg, set_int2_wu, bool;
        fn set_int2_free_fall, Md2Cfg, Md2Cfg, set_int2_ff, bool;
        fn set_int2_double_tap, Md2Cfg, Md2Cfg, set_int2_double_tap, bool;
        fn set_int2_6d, Md2Cfg, Md2Cfg, set_int2_6d, bool;
        fn set_int2_timestamp, Md2Cfg, Md2Cfg, set_int2_timestamp, bool;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn configures_tap_priority_without_losing_threshold() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::TapCfg1.addr()],
                vec![0x1f],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::TapCfg1.addr(), 0xbf]),
        ]);

        sensor.set_tap_priority(&mut i2c, 0b101).unwrap();
        i2c.done();
    }

    #[test]
    fn routes_double_tap_to_int2() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Md2Cfg.addr()],
                vec![0x01],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Md2Cfg.addr(), 0x11]),
        ]);

        sensor.set_int2_double_tap(&mut i2c, true).unwrap();
        i2c.done();
    }
}
