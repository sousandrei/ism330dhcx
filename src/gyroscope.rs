use crate::registers::FsG;

/// Conversion factor from Degrees Per Second to Radians Per Second.
pub const SENSORS_DPS_TO_RADS: f32 = 0.017453292;

/// High-level gyroscope reading.
#[derive(Copy, Clone, Debug, Eq, PartialEq, defmt::Format)]
pub struct GyroValue {
    range: FsG,
    count: [i16; 3],
}

impl GyroValue {
    /// Create a new `GyroValue` from raw counts and range.
    pub fn new(range: FsG, count: [i16; 3]) -> GyroValue {
        GyroValue { range, count }
    }

    /// Create a new `GyroValue` from raw byte measurements (little-endian).
    pub fn from_msr(range: FsG, measurements: &[u8; 6]) -> GyroValue {
        let raw_gyro_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_gyro_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_gyro_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);
        GyroValue {
            range,
            count: [raw_gyro_x, raw_gyro_y, raw_gyro_z],
        }
    }

    /// Return the raw signed integer counts for X, Y, Z axes.
    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// Return angular velocity in radians per second [rad/s].
    pub fn as_rad(&self) -> [f32; 3] {
        self.as_mdps().map(|v| v * SENSORS_DPS_TO_RADS / 1000.)
    }

    /// Return angular velocity in milli-degrees per second \[mdps\].
    pub fn as_mdps(&self) -> [f32; 3] {
        let sensitivity = self.range.sensitivity();
        self.count.map(|r| r as f32 * sensitivity)
    }

    /// Return angular velocity in degrees per second \[dps\].
    pub fn as_dps(&self) -> [f32; 3] {
        self.as_mdps().map(|v| v / 1000.)
    }
}

use crate::Ism330Dhcx;
use crate::RegisterBus;
use crate::registers::{
    Ctrl2G, Ctrl5C, Ctrl6C, Ctrl7G, FsGScale, Ftype, HpmG, OdrG, Register, StG,
};

/// Gyroscope sensor methods.
pub trait Gyroscope {
    /// Set gyroscope output data rate.
    fn set_gyro_odr<I2C>(&mut self, i2c: &mut I2C, odr: OdrG) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Get the configured gyroscope output data rate.
    fn get_gyro_odr<I2C>(&self, i2c: &mut I2C) -> Result<OdrG, I2C::Error>
    where
        I2C: RegisterBus;
    /// Set gyroscope full-scale range.
    fn set_gyro_scale<I2C>(&mut self, i2c: &mut I2C, scale: FsG) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Get current gyroscope full-scale range.
    fn get_gyro_scale<I2C>(&self, i2c: &mut I2C) -> Result<FsG, I2C::Error>
    where
        I2C: RegisterBus;
    /// Enable Gyroscope High-Performance mode (disable g_hm_mode bit).
    fn set_g_hm_mode<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_gyro_filter_type<I2C>(&self, i2c: &mut I2C, filter: Ftype) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_gyro_hpf<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
        cutoff: HpmG,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_gyro_self_test<I2C>(&self, i2c: &mut I2C, mode: StG) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Get gyroscope reading.
    fn get_gyroscope<I2C>(&self, i2c: &mut I2C) -> Result<GyroValue, I2C::Error>
    where
        I2C: RegisterBus;
}

impl Gyroscope for Ism330Dhcx {
    fn set_gyro_odr<I2C>(&mut self, i2c: &mut I2C, odr: OdrG) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl2G, |v| {
            let mut reg = Ctrl2G::from_bytes([v]);
            reg.set_odr_g(odr);
            reg.into_bytes()[0]
        })
    }

    fn get_gyro_odr<I2C>(&self, i2c: &mut I2C) -> Result<OdrG, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl2G::from_bytes([self.read_reg(i2c, Register::Ctrl2G)?]).odr_g())
    }

    fn set_gyro_scale<I2C>(&mut self, i2c: &mut I2C, scale: FsG) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl2G, |v| {
            let mut reg = Ctrl2G::from_bytes([v]);

            // Reset fields
            reg.set_fs_125(false);
            reg.set_fs_4000(false);

            match scale {
                FsG::Dps125 => reg.set_fs_125(true),
                FsG::Dps4000 => reg.set_fs_4000(true),
                FsG::Dps250 => reg.set_fs_g(FsGScale::Dps250),
                FsG::Dps500 => reg.set_fs_g(FsGScale::Dps500),
                FsG::Dps1000 => reg.set_fs_g(FsGScale::Dps1000),
                FsG::Dps2000 => reg.set_fs_g(FsGScale::Dps2000),
            }
            reg.into_bytes()[0]
        })
    }

    fn get_gyro_scale<I2C>(&self, i2c: &mut I2C) -> Result<FsG, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let v = self.read_reg(i2c, Register::Ctrl2G)?;
        let reg = Ctrl2G::from_bytes([v]);

        if reg.fs_4000() {
            return Ok(FsG::Dps4000);
        }
        if reg.fs_125() {
            return Ok(FsG::Dps125);
        }
        Ok(match reg.fs_g() {
            FsGScale::Dps250 => FsG::Dps250,
            FsGScale::Dps500 => FsG::Dps500,
            FsGScale::Dps1000 => FsG::Dps1000,
            FsGScale::Dps2000 => FsG::Dps2000,
        })
    }

    fn set_g_hm_mode<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl7G, |v| {
            let mut reg = Ctrl7G::from_bytes([v]);
            // Logic inverted: bit 1 means disabled.
            reg.set_g_hm_mode(!enable);
            reg.into_bytes()[0]
        })
    }

    fn set_gyro_filter_type<I2C>(&self, i2c: &mut I2C, filter: Ftype) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl6C, |v| {
            let mut r = Ctrl6C::from_bytes([v]);
            r.set_ftype(filter);
            r.into_bytes()[0]
        })
    }
    fn set_gyro_hpf<I2C>(&self, i2c: &mut I2C, enable: bool, cutoff: HpmG) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl7G, |v| {
            let mut r = Ctrl7G::from_bytes([v]);
            r.set_hp_en_g(enable);
            r.set_hpm_g(cutoff);
            r.into_bytes()[0]
        })
    }
    fn set_gyro_self_test<I2C>(&self, i2c: &mut I2C, mode: StG) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::Ctrl5C, |v| {
            let mut r = Ctrl5C::from_bytes([v]);
            r.set_st_g(mode);
            r.into_bytes()[0]
        })
    }

    fn get_gyroscope<I2C>(&self, i2c: &mut I2C) -> Result<GyroValue, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let scale = self.get_gyro_scale(i2c)?;

        let mut measurements = [0u8; 6];
        self.read_register(i2c, Register::OutXLG.addr(), &mut measurements)?;

        Ok(GyroValue::from_msr(scale, &measurements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DEFAULT_I2C_ADDRESS, Ism330Dhcx};
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn test_get_gyroscope_reads_output_register() {
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
                vec![Register::Ctrl2G.addr()],
                vec![0x00],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::OutXLG.addr()],
                vec![0x34, 0x12, 0x78, 0x56, 0xbc, 0x9a],
            ),
        ]);

        let sensor = Ism330Dhcx::new(&mut i2c).unwrap();
        let reading = sensor.get_gyroscope(&mut i2c).unwrap();

        assert_eq!(reading.count(), [0x1234, 0x5678, -0x6544]);
        i2c.done();
    }

    macro_rules! gyro_modify_test {
        ($name:ident, $method:ident, $value:expr, $register:expr, $read:expr, $write:expr) => {
            #[test]
            fn $name() {
                let mut i2c = Mock::new(&[
                    Transaction::write_read(
                        DEFAULT_I2C_ADDRESS,
                        vec![$register.addr()],
                        vec![$read],
                    ),
                    Transaction::write(DEFAULT_I2C_ADDRESS, vec![$register.addr(), $write]),
                ]);
                let sensor = Ism330Dhcx {
                    address: DEFAULT_I2C_ADDRESS,
                };

                sensor.$method(&mut i2c, $value).unwrap();
                i2c.done();
            }
        };
    }

    gyro_modify_test!(
        test_set_gyro_filter_type,
        set_gyro_filter_type,
        Ftype::Bw3,
        Register::Ctrl6C,
        0xa8,
        0xab
    );
    gyro_modify_test!(
        test_set_gyro_self_test,
        set_gyro_self_test,
        StG::Negative,
        Register::Ctrl5C,
        0xa8,
        0xac
    );

    #[test]
    fn test_set_gyro_hpf() {
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl7G.addr()],
                vec![0x01],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl7G.addr(), 0x49]),
        ]);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        sensor.set_gyro_hpf(&mut i2c, true, HpmG::Hpmg65).unwrap();
        i2c.done();
    }
}
