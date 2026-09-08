use crate::registers::FsXl;

/// Standard gravity constant [m/s²]
pub const SENSORS_GRAVITY_STANDARD: f32 = 9.80665;

/// High-level accelerometer reading.
#[derive(Copy, Clone, Debug, Eq, PartialEq, defmt::Format)]
pub struct AccelValue {
    range: FsXl,
    count: [i16; 3],
}

impl AccelValue {
    /// Create a new `AccelValue` from raw counts and range.
    pub fn new(range: FsXl, count: [i16; 3]) -> AccelValue {
        AccelValue { range, count }
    }

    /// Create a new `AccelValue` from raw byte measurements (little-endian).
    pub fn from_msr(range: FsXl, measurements: &[u8; 6]) -> AccelValue {
        let raw_acc_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_acc_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_acc_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);
        AccelValue {
            range,
            count: [raw_acc_x, raw_acc_y, raw_acc_z],
        }
    }

    /// Return the raw signed integer counts for X, Y, Z axes.
    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// Return acceleration in meters per second squared [m/s²].
    pub fn as_m_ss(&self) -> [f32; 3] {
        self.as_mg().map(|v| v * SENSORS_GRAVITY_STANDARD / 1000.)
    }

    /// Return acceleration in milli-g \[mg\].
    pub fn as_mg(&self) -> [f32; 3] {
        let sensitivity = self.range.sensitivity();
        self.count.map(|r| r as f32 * sensitivity)
    }

    /// Return acceleration in g \[g\].
    pub fn as_g(&self) -> [f32; 3] {
        self.as_mg().map(|v| v / 1000.)
    }
}

use crate::Ism330Dhcx;
use crate::RegisterBus;
use crate::registers::{Ctrl1Xl, Ctrl5C, Ctrl8Xl, Ctrl9Xl, OdrXl, Register, StXl};

/// Accelerometer sensor methods.
pub trait Accelerometer {
    /// Set accelerometer output data rate.
    fn set_accel_odr<I2C>(&mut self, i2c: &mut I2C, odr: OdrXl) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Get the configured accelerometer output data rate.
    fn get_accel_odr<I2C>(&self, i2c: &mut I2C) -> Result<OdrXl, I2C::Error>
    where
        I2C: RegisterBus;
    /// Set accelerometer full-scale range.
    fn set_accel_scale<I2C>(&mut self, i2c: &mut I2C, scale: FsXl) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Get current accelerometer full-scale range.
    fn get_accel_scale<I2C>(&self, i2c: &mut I2C) -> Result<FsXl, I2C::Error>
    where
        I2C: RegisterBus;
    /// Enable low-pass filter 2 for accelerometer.
    fn set_lpf2_xl_en<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Get whether accelerometer low-pass filter 2 is enabled.
    fn get_lpf2_xl_en<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus;
    /// Configure the accelerometer high-pass/low-pass filter cutoff.
    fn set_hpcf_xl<I2C>(
        &self,
        i2c: &mut I2C,
        cutoff: crate::registers::HpcfXl,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set the accelerometer low-pass filter output used by 6D detection.
    fn set_low_pass_on_6d<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_hp_slope_xl_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_fast_settling_mode_xl<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_slope_fds<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_hp_ref_mode<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Select accelerometer self-test mode.
    fn set_accel_self_test<I2C>(&self, i2c: &mut I2C, mode: StXl) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set DEN value on X axis.
    fn set_den_x<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set DEN value on Y axis.
    fn set_den_y<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set DEN value on Z axis.
    fn set_den_z<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Set DEN device configuration.
    fn set_den_device_conf<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_den_lh<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_den_xl_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_den_xl_g<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Get accelerometer reading.
    fn get_accelerometer<I2C>(&self, i2c: &mut I2C) -> Result<AccelValue, I2C::Error>
    where
        I2C: RegisterBus;
}

impl Accelerometer for Ism330Dhcx {
    fn set_accel_odr<I2C>(&mut self, i2c: &mut I2C, odr: OdrXl) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_odr_xl(odr);
            reg.into_bytes()[0]
        })
    }

    fn get_accel_odr<I2C>(&self, i2c: &mut I2C) -> Result<OdrXl, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl1Xl::from_bytes([self.read_reg(i2c, Register::Ctrl1Xl)?]).odr_xl())
    }

    fn set_accel_scale<I2C>(&mut self, i2c: &mut I2C, scale: FsXl) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_fs_xl(scale);
            reg.into_bytes()[0]
        })
    }

    fn get_accel_scale<I2C>(&self, i2c: &mut I2C) -> Result<FsXl, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let v = self.read_reg(i2c, Register::Ctrl1Xl)?;
        let reg = Ctrl1Xl::from_bytes([v]);
        Ok(reg.fs_xl())
    }

    fn set_lpf2_xl_en<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl1Xl, |v| {
            let mut reg = Ctrl1Xl::from_bytes([v]);
            reg.set_lpf2_xl_en(enable);
            reg.into_bytes()[0]
        })
    }

    fn get_lpf2_xl_en<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl1Xl::from_bytes([self.read_reg(i2c, Register::Ctrl1Xl)?]).lpf2_xl_en())
    }

    fn set_hpcf_xl<I2C>(
        &self,
        i2c: &mut I2C,
        cutoff: crate::registers::HpcfXl,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl8Xl, |v| {
            let mut r = Ctrl8Xl::from_bytes([v]);
            r.set_hpcf_xl(cutoff);
            r.into_bytes()[0]
        })
    }

    fn set_low_pass_on_6d<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl8Xl, |v| {
            let mut r = Ctrl8Xl::from_bytes([v]);
            r.set_low_pass_on_6d(enable);
            r.into_bytes()[0]
        })
    }
    fn set_hp_slope_xl_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl8Xl, |v| {
            let mut r = Ctrl8Xl::from_bytes([v]);
            r.set_hp_slope_xl_en(enable);
            r.into_bytes()[0]
        })
    }
    fn set_fast_settling_mode_xl<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl8Xl, |v| {
            let mut r = Ctrl8Xl::from_bytes([v]);
            r.set_fast_settling_mode_xl(enable);
            r.into_bytes()[0]
        })
    }
    fn set_slope_fds<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl8Xl, |v| {
            let mut r = Ctrl8Xl::from_bytes([v]);
            r.set_slope_fds(enable);
            r.into_bytes()[0]
        })
    }
    fn set_hp_ref_mode<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl8Xl, |v| {
            let mut r = Ctrl8Xl::from_bytes([v]);
            r.set_hp_ref_mode(enable);
            r.into_bytes()[0]
        })
    }
    fn set_accel_self_test<I2C>(&self, i2c: &mut I2C, mode: StXl) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl5C, |v| {
            let mut r = Ctrl5C::from_bytes([v]);
            r.set_st_xl(mode);
            r.into_bytes()[0]
        })
    }

    fn set_den_x<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = crate::registers::Ctrl9Xl::from_bytes([v]);
            reg.set_den_x(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_y<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = crate::registers::Ctrl9Xl::from_bytes([v]);
            reg.set_den_y(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_z<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = crate::registers::Ctrl9Xl::from_bytes([v]);
            reg.set_den_z(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_device_conf<I2C>(&mut self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut reg = crate::registers::Ctrl9Xl::from_bytes([v]);
            reg.set_device_conf(val);
            reg.into_bytes()[0]
        })
    }

    fn set_den_lh<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut r = Ctrl9Xl::from_bytes([v]);
            r.set_den_lh(enable);
            r.into_bytes()[0]
        })
    }
    fn set_den_xl_en<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut r = Ctrl9Xl::from_bytes([v]);
            r.set_den_xl_en(enable);
            r.into_bytes()[0]
        })
    }
    fn set_den_xl_g<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl9Xl, |v| {
            let mut r = Ctrl9Xl::from_bytes([v]);
            r.set_den_xl_g(enable);
            r.into_bytes()[0]
        })
    }

    fn get_accelerometer<I2C>(&self, i2c: &mut I2C) -> Result<AccelValue, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let scale = self.get_accel_scale(i2c)?;

        let mut measurements = [0u8; 6];
        self.read_register(i2c, Register::OutXLA.addr(), &mut measurements)?;

        Ok(AccelValue::from_msr(scale, &measurements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::HpcfXl;
    use crate::{DEFAULT_I2C_ADDRESS, Ism330Dhcx};
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn test_get_accelerometer_reads_output_register() {
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::WhoAmI.addr()],
                vec![0x6b],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr()],
                vec![0x00],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl3C.addr(), 0x40]),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr()],
                vec![0x40],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl3C.addr(), 0x44]),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl1Xl.addr()],
                vec![0x00],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::OutXLA.addr()],
                vec![0x34, 0x12, 0x78, 0x56, 0xbc, 0x9a],
            ),
        ]);

        let sensor = Ism330Dhcx::new(&mut i2c).unwrap();
        let reading = sensor.get_accelerometer(&mut i2c).unwrap();

        assert_eq!(reading.count(), [0x1234, 0x5678, -0x6544]);
        i2c.done();
    }

    macro_rules! ctrl8_modify_test {
        ($name:ident, $method:ident, $value:expr, $read:expr, $write:expr) => {
            #[test]
            fn $name() {
                let mut i2c = Mock::new(&[
                    Transaction::write_read(
                        DEFAULT_I2C_ADDRESS,
                        vec![Register::Ctrl8Xl.addr()],
                        vec![$read],
                    ),
                    Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl8Xl.addr(), $write]),
                ]);
                let sensor = Ism330Dhcx {
                    address: DEFAULT_I2C_ADDRESS,
                };

                sensor.$method(&mut i2c, $value).unwrap();
                i2c.done();
            }
        };
    }

    ctrl8_modify_test!(test_set_hpcf_xl, set_hpcf_xl, HpcfXl::OdrDiv45, 0xa1, 0x61);
    ctrl8_modify_test!(
        test_set_low_pass_on_6d,
        set_low_pass_on_6d,
        true,
        0xa0,
        0xa1
    );
    ctrl8_modify_test!(
        test_set_hp_slope_xl_en,
        set_hp_slope_xl_en,
        true,
        0xa0,
        0xa4
    );
    ctrl8_modify_test!(
        test_set_fast_settling_mode_xl,
        set_fast_settling_mode_xl,
        true,
        0xa0,
        0xa8
    );
    ctrl8_modify_test!(test_set_slope_fds, set_slope_fds, true, 0xa0, 0xa2);
    ctrl8_modify_test!(test_set_hp_ref_mode, set_hp_ref_mode, true, 0xa0, 0xb0);

    macro_rules! ctrl9_modify_test {
        ($name:ident, $method:ident, $read:expr, $write:expr) => {
            #[test]
            fn $name() {
                let mut i2c = Mock::new(&[
                    Transaction::write_read(
                        DEFAULT_I2C_ADDRESS,
                        vec![Register::Ctrl9Xl.addr()],
                        vec![$read],
                    ),
                    Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl9Xl.addr(), $write]),
                ]);
                let sensor = Ism330Dhcx {
                    address: DEFAULT_I2C_ADDRESS,
                };

                sensor.$method(&mut i2c, true).unwrap();
                i2c.done();
            }
        };
    }

    ctrl9_modify_test!(test_set_den_lh, set_den_lh, 0x00, 0x04);
    ctrl9_modify_test!(test_set_den_xl_en, set_den_xl_en, 0x00, 0x08);
    ctrl9_modify_test!(test_set_den_xl_g, set_den_xl_g, 0x00, 0x10);

    macro_rules! ctrl9_modify_mut_test {
        ($name:ident, $method:ident, $write:expr) => {
            #[test]
            fn $name() {
                let mut i2c = Mock::new(&[
                    Transaction::write_read(
                        DEFAULT_I2C_ADDRESS,
                        vec![Register::Ctrl9Xl.addr()],
                        vec![0x00],
                    ),
                    Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl9Xl.addr(), $write]),
                ]);
                let mut sensor = Ism330Dhcx {
                    address: DEFAULT_I2C_ADDRESS,
                };

                sensor.$method(&mut i2c, true).unwrap();
                i2c.done();
            }
        };
    }

    ctrl9_modify_mut_test!(test_set_den_x, set_den_x, 0x20);
    ctrl9_modify_mut_test!(test_set_den_y, set_den_y, 0x40);
    ctrl9_modify_mut_test!(test_set_den_z, set_den_z, 0x80);

    #[test]
    fn test_set_accel_self_test() {
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl5C.addr()],
                vec![0xa8],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl5C.addr(), 0xaa]),
        ]);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        sensor
            .set_accel_self_test(&mut i2c, StXl::Negative)
            .unwrap();
        i2c.done();
    }
}
