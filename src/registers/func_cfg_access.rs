use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Enable embedded functions register (01h)
    pub struct FuncCfgAccess(u8);
    impl Debug;
    /// Enable access to the sensor hub (I2C master) registers.
    pub shub_reg_access, set_shub_reg_access: 6;
    /// Enable access to the embedded functions configuration registers.
    pub func_cfg_access, set_func_cfg_access: 7;
}

impl FuncCfgAccess {
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

impl Default for FuncCfgAccess {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for FUNC_CFG_ACCESS register.
pub trait FuncCfgAccessConfig {
    /// Enable access to the sensor hub (I2C master) registers.
    fn set_shub_reg_access<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enable access to the embedded functions configuration registers.
    fn set_func_cfg_access<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl FuncCfgAccessConfig for Ism330Dhcx {
    fn set_shub_reg_access<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FuncCfgAccess, |v| {
            let mut reg = FuncCfgAccess::from_bytes([v]);
            reg.set_shub_reg_access(val);
            reg.into_bytes()[0]
        })
    }

    fn set_func_cfg_access<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FuncCfgAccess, |v| {
            let mut reg = FuncCfgAccess::from_bytes([v]);
            reg.set_func_cfg_access(val);
            reg.into_bytes()[0]
        })
    }
}
