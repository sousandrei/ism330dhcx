use crate::RegisterBus;

use crate::Ism330Dhcx;
use crate::registers::{
    Ctrl1Ois, Ctrl2Ois, Ctrl3Ois, IntOis, OisAccelScale, OisAccelSelfTest, OisFilterCutoff,
    OisFilterType, OisGyroScale, OisSelfTest, Register, StatusReg,
};

/// Raw OIS-chain output registers.
#[derive(Copy, Clone, Debug, Eq, PartialEq, defmt::Format)]
pub struct OisOutput {
    /// Gyroscope X, Y, and Z output counts.
    pub gyro: [i16; 3],
    /// Accelerometer X, Y, and Z output counts.
    pub accel: [i16; 3],
}

/// OIS and auxiliary SPI configuration.
pub trait Ois {
    fn set_ois_spi2<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_gyro_scale<I2C>(&self, i2c: &mut I2C, scale: OisGyroScale) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_gyro_125<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_mode4<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_spi3<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_level1<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_high_pass<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_filter_type<I2C>(
        &self,
        i2c: &mut I2C,
        filter: OisFilterType,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_filter_cutoff<I2C>(
        &self,
        i2c: &mut I2C,
        cutoff: OisFilterCutoff,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_accel_scale<I2C>(
        &self,
        i2c: &mut I2C,
        scale: OisAccelScale,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_self_test<I2C>(&self, i2c: &mut I2C, mode: OisSelfTest) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_accel_self_test<I2C>(
        &self,
        i2c: &mut I2C,
        mode: OisAccelSelfTest,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_ois_interrupt<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the OIS interrupt and self-test register.
    fn get_ois_interrupt<I2C>(&self, i2c: &mut I2C) -> Result<IntOis, I2C::Error>
    where
        I2C: RegisterBus;
    fn get_ois_spi2<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus;
    fn get_ois_gyro_125<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus;
    fn get_ois_mode4<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus;
    fn get_ois_spi3<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus;
    fn get_ois_level1<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus;
    fn get_ois_high_pass<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the OIS gyroscope configuration.
    fn get_ois_gyro_scale<I2C>(&self, i2c: &mut I2C) -> Result<OisGyroScale, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the OIS accelerometer full-scale configuration.
    fn get_ois_accel_scale<I2C>(&self, i2c: &mut I2C) -> Result<OisAccelScale, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the OIS filter configuration.
    fn get_ois_filter_type<I2C>(&self, i2c: &mut I2C) -> Result<OisFilterType, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the OIS filter cutoff configuration.
    fn get_ois_filter_cutoff<I2C>(&self, i2c: &mut I2C) -> Result<OisFilterCutoff, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the OIS gyroscope self-test setting.
    fn get_ois_self_test<I2C>(&self, i2c: &mut I2C) -> Result<OisSelfTest, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the primary-interface status register used by the OIS chain.
    fn get_ois_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusReg, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the shared output addresses as raw OIS output counts.
    ///
    /// These addresses contain OIS-chain values only when read through the
    /// auxiliary SPI interface; primary I2C/SPI reads return GP-chain values.
    fn get_ois_output<I2C>(&self, i2c: &mut I2C) -> Result<OisOutput, I2C::Error>
    where
        I2C: RegisterBus;
}

macro_rules! ois_setters {
    ($(fn $method:ident, $register:ident, $config:ident, $setter:ident, $value:ty;)*) => {
        $(fn $method<I2C>(&self, i2c: &mut I2C, value: $value) -> Result<(), I2C::Error>
        where I2C: RegisterBus {
            self.modify_reg(i2c, Register::$register, |raw| {
                let mut reg = $config::from_bytes([raw]);
                reg.$setter(value);
                reg.into_bytes()[0]
            })
        })*
    };
}

impl Ois for Ism330Dhcx {
    ois_setters! {
        fn set_ois_spi2, Ctrl1Ois, Ctrl1Ois, set_ois_en_spi2, bool;
        fn set_ois_gyro_scale, Ctrl1Ois, Ctrl1Ois, set_fs_g_ois, OisGyroScale;
        fn set_ois_gyro_125, Ctrl1Ois, Ctrl1Ois, set_fs_125_ois, bool;
        fn set_ois_mode4, Ctrl1Ois, Ctrl1Ois, set_mode4_en, bool;
        fn set_ois_spi3, Ctrl1Ois, Ctrl1Ois, set_sim_ois, bool;
        fn set_ois_level1, Ctrl1Ois, Ctrl1Ois, set_lvl1_ois, bool;
        fn set_ois_high_pass, Ctrl2Ois, Ctrl2Ois, set_hp_en_ois, bool;
        fn set_ois_filter_type, Ctrl2Ois, Ctrl2Ois, set_ftype_ois, OisFilterType;
        fn set_ois_filter_cutoff, Ctrl2Ois, Ctrl2Ois, set_hpm_ois, OisFilterCutoff;
        fn set_ois_accel_scale, Ctrl3Ois, Ctrl3Ois, set_fs_xl_ois, OisAccelScale;
        fn set_ois_self_test, Ctrl3Ois, Ctrl3Ois, set_st_ois, OisSelfTest;
        fn set_ois_interrupt, IntOis, IntOis, set_int2_drdy_ois, bool;
    }

    fn set_ois_accel_self_test<I2C>(
        &self,
        i2c: &mut I2C,
        mode: OisAccelSelfTest,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_reg(i2c, Register::IntOis, |raw| {
            let mut reg = IntOis::from_bytes([raw]);
            reg.set_st_xl_ois(mode);
            reg.into_bytes()[0]
        })
    }

    fn get_ois_interrupt<I2C>(&self, i2c: &mut I2C) -> Result<IntOis, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(IntOis::from_bytes([self.read_reg(i2c, Register::IntOis)?]))
    }

    fn get_ois_spi2<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl1Ois::from_bytes([self.read_reg(i2c, Register::Ctrl1Ois)?]).ois_en_spi2())
    }

    fn get_ois_gyro_125<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl1Ois::from_bytes([self.read_reg(i2c, Register::Ctrl1Ois)?]).fs_125_ois())
    }

    fn get_ois_mode4<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl1Ois::from_bytes([self.read_reg(i2c, Register::Ctrl1Ois)?]).mode4_en())
    }

    fn get_ois_spi3<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl1Ois::from_bytes([self.read_reg(i2c, Register::Ctrl1Ois)?]).sim_ois())
    }

    fn get_ois_level1<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl1Ois::from_bytes([self.read_reg(i2c, Register::Ctrl1Ois)?]).lvl1_ois())
    }

    fn get_ois_high_pass<I2C>(&self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl2Ois::from_bytes([self.read_reg(i2c, Register::Ctrl2Ois)?]).hp_en_ois())
    }

    fn get_ois_gyro_scale<I2C>(&self, i2c: &mut I2C) -> Result<OisGyroScale, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl1Ois::from_bytes([self.read_reg(i2c, Register::Ctrl1Ois)?]).fs_g_ois())
    }

    fn get_ois_accel_scale<I2C>(&self, i2c: &mut I2C) -> Result<OisAccelScale, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl3Ois::from_bytes([self.read_reg(i2c, Register::Ctrl3Ois)?]).fs_xl_ois())
    }

    fn get_ois_filter_type<I2C>(&self, i2c: &mut I2C) -> Result<OisFilterType, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl2Ois::from_bytes([self.read_reg(i2c, Register::Ctrl2Ois)?]).ftype_ois())
    }

    fn get_ois_filter_cutoff<I2C>(&self, i2c: &mut I2C) -> Result<OisFilterCutoff, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl2Ois::from_bytes([self.read_reg(i2c, Register::Ctrl2Ois)?]).hpm_ois())
    }

    fn get_ois_self_test<I2C>(&self, i2c: &mut I2C) -> Result<OisSelfTest, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Ctrl3Ois::from_bytes([self.read_reg(i2c, Register::Ctrl3Ois)?]).st_ois())
    }

    fn get_ois_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusReg, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(StatusReg::from_bytes([
            self.read_reg(i2c, Register::StatusReg)?
        ]))
    }

    fn get_ois_output<I2C>(&self, i2c: &mut I2C) -> Result<OisOutput, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut gyro = [0u8; 6];
        let mut accel = [0u8; 6];
        self.read_register(i2c, Register::OutXLG.addr(), &mut gyro)?;
        self.read_register(i2c, Register::OutXLA.addr(), &mut accel)?;
        Ok(OisOutput {
            gyro: [
                i16::from_le_bytes([gyro[0], gyro[1]]),
                i16::from_le_bytes([gyro[2], gyro[3]]),
                i16::from_le_bytes([gyro[4], gyro[5]]),
            ],
            accel: [
                i16::from_le_bytes([accel[0], accel[1]]),
                i16::from_le_bytes([accel[2], accel[3]]),
                i16::from_le_bytes([accel[4], accel[5]]),
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn enables_auxiliary_ois_spi() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl1Ois.addr()],
                vec![0x20],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl1Ois.addr(), 0x21]),
        ]);

        sensor.set_ois_spi2(&mut i2c, true).unwrap();
        i2c.done();
    }

    #[test]
    fn reads_typed_ois_configuration_and_outputs() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl1Ois.addr()],
                vec![0x0c],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3Ois.addr()],
                vec![0xc0],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl2Ois.addr()],
                vec![0x12],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl2Ois.addr()],
                vec![0x12],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::IntOis.addr()],
                vec![0x81],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::StatusReg.addr()],
                vec![0x03],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::OutXLG.addr()],
                vec![0x34, 0x12, 0x78, 0x56, 0xbc, 0x9a],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::OutXLA.addr()],
                vec![0x01, 0x00, 0xfe, 0xff, 0x00, 0x80],
            ),
        ]);

        assert_eq!(
            sensor.get_ois_gyro_scale(&mut i2c).unwrap(),
            OisGyroScale::Dps2000
        );
        assert_eq!(
            sensor.get_ois_accel_scale(&mut i2c).unwrap(),
            OisAccelScale::G8
        );
        assert_eq!(
            sensor.get_ois_filter_type(&mut i2c).unwrap(),
            OisFilterType::Bw222Hz
        );
        assert_eq!(
            sensor.get_ois_filter_cutoff(&mut i2c).unwrap(),
            OisFilterCutoff::Mhz65
        );
        assert_eq!(
            sensor.get_ois_interrupt(&mut i2c).unwrap().into_bytes(),
            [0x81]
        );
        assert_eq!(
            sensor.get_ois_status(&mut i2c).unwrap().into_bytes(),
            [0x03]
        );
        assert_eq!(
            sensor.get_ois_output(&mut i2c).unwrap(),
            OisOutput {
                gyro: [0x1234, 0x5678, -0x6544],
                accel: [1, -2, -32768],
            }
        );
        i2c.done();
    }
}
