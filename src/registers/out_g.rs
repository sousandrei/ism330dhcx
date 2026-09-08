use crate::Ism330Dhcx;
use crate::registers::{Ctrl2G, Ctrl7G, FsG, FsGScale, OdrG, Register};
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

/// Conversion factor from Degrees Per Second to Radians Per Second.
pub const SENSORS_DPS_TO_RADS: f64 = 0.017453292;

bitfield! {
    /// Gyroscope output register (r).
    pub struct OutG(u64);
    impl Debug;
    /// X-axis output.
    pub x, _: 15, 0;
    /// Y-axis output.
    pub y, _: 31, 16;
    /// Z-axis output.
    pub z, _: 47, 32;
}

impl OutG {
    pub fn from_bytes(bytes: [u8; 6]) -> Self {
        let mut full_bytes = [0u8; 8];
        full_bytes[..6].copy_from_slice(&bytes);
        Self(u64::from_le_bytes(full_bytes))
    }

    /// Returns raw counts as [x, y, z]
    pub fn counts(&self) -> [i16; 3] {
        [self.x() as i16, self.y() as i16, self.z() as i16]
    }
}

/// High-level gyroscope reading.
#[derive(Copy, Clone, Debug, defmt::Format)]
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
        let out = OutG::from_bytes(*measurements);
        GyroValue {
            range,
            count: out.counts(),
        }
    }

    /// Return the raw signed integer counts for X, Y, Z axes.
    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// Return angular velocity in radians per second [rad/s].
    pub fn as_rad(&self) -> [f64; 3] {
        self.as_mdps().map(|v| v * SENSORS_DPS_TO_RADS / 1000.)
    }

    /// Return angular velocity in milli-degrees per second \[mdps\].
    pub fn as_mdps(&self) -> [f64; 3] {
        let sensitivity = self.range.sensitivity() as f64;
        self.count.map(|r| r as f64 * sensitivity)
    }

    /// Return angular velocity in degrees per second \[dps\].
    pub fn as_dps(&self) -> [f64; 3] {
        self.as_mdps().map(|v| v / 1000.)
    }
}

/// Gyroscope sensor methods.
pub trait Gyroscope {
    /// Set gyroscope output data rate.
    fn set_gyro_odr<I2C>(&mut self, i2c: &mut I2C, odr: OdrG) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Set gyroscope full-scale range.
    fn set_gyro_scale<I2C>(&mut self, i2c: &mut I2C, scale: FsG) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Get current gyroscope full-scale range.
    fn get_gyro_scale<I2C>(&self, i2c: &mut I2C) -> Result<FsG, I2C::Error>
    where
        I2C: I2c;
    /// Enable Gyroscope High-Performance mode (disable g_hm_mode bit).
    fn set_g_hm_mode<I2C>(&mut self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Get gyroscope reading.
    fn get_gyroscope<I2C>(&self, i2c: &mut I2C) -> Result<GyroValue, I2C::Error>
    where
        I2C: I2c;

    /// Set chain full scale.
    fn set_chain_full_scale<I2C>(
        &mut self,
        i2c: &mut I2C,
        scale: FsG,
    ) -> Result<&mut Self, I2C::Error>
    where
        I2C: I2c;
}

impl Gyroscope for Ism330Dhcx {
    fn set_gyro_odr<I2C>(&mut self, i2c: &mut I2C, odr: OdrG) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl2G, |v| {
            let mut reg = Ctrl2G::from_bytes([v]);
            reg.set_odr_g(odr);
            reg.into_bytes()[0]
        })
    }

    fn set_gyro_scale<I2C>(&mut self, i2c: &mut I2C, scale: FsG) -> Result<(), I2C::Error>
    where
        I2C: I2c,
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
        I2C: I2c,
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
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::Ctrl7G, |v| {
            let mut reg = Ctrl7G::from_bytes([v]);
            // Logic inverted: bit 1 means disabled.
            reg.set_g_hm_mode(!enable);
            reg.into_bytes()[0]
        })
    }

    fn get_gyroscope<I2C>(&self, i2c: &mut I2C) -> Result<GyroValue, I2C::Error>
    where
        I2C: I2c,
    {
        let scale = self.get_gyro_scale(i2c)?;

        let mut measurements = [0u8; 6];
        i2c.write_read(self.address, &[Register::OutXLG.addr()], &mut measurements)?;

        let out = OutG::from_bytes(measurements);
        Ok(GyroValue::new(scale, out.counts()))
    }

    fn set_chain_full_scale<I2C>(
        &mut self,
        i2c: &mut I2C,
        scale: FsG,
    ) -> Result<&mut Self, I2C::Error>
    where
        I2C: I2c,
    {
        self.set_gyro_scale(i2c, scale)?;
        Ok(self)
    }
}
