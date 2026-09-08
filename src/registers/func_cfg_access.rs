#![allow(unused_parens)]
use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;
use modular_bitfield::{bitfield, specifiers::B6};

/// Enable access to the embedded functions configuration registers. (01h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FuncCfgAccess {
    #[skip]
    pub __: B6,
    /// Enable access to the sensor hub (I2C master) registers.
    pub shub_reg_access: bool,
    /// Enable access to the embedded functions configuration registers.
    pub func_cfg_access: bool,
}

impl Default for FuncCfgAccess {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FuncCfgAccessExt {
    /// Enable access to the sensor hub (I2C master) registers.
    fn set_shub_reg_access<I2C>(&self, i2c: &mut I2C, access: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Enable access to the embedded functions configuration registers.
    fn set_func_cfg_access<I2C>(&self, i2c: &mut I2C, access: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl FuncCfgAccessExt for Ism330Dhcx {
    fn set_shub_reg_access<I2C>(&self, i2c: &mut I2C, access: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FuncCfgAccess, |v| {
            let mut reg = FuncCfgAccess::from_bytes([v]);
            reg.set_shub_reg_access(access);
            reg.into_bytes()[0]
        })
    }

    fn set_func_cfg_access<I2C>(&self, i2c: &mut I2C, access: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::FuncCfgAccess, |v| {
            let mut reg = FuncCfgAccess::from_bytes([v]);
            reg.set_func_cfg_access(access);
            reg.into_bytes()[0]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Ism330Dhcx;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn test_func_cfg_access() {
        let mut reg = FuncCfgAccess::new();
        reg.set_shub_reg_access(true);
        assert_eq!(reg.into_bytes()[0], 0x40);

        let mut reg = FuncCfgAccess::new();
        reg.set_func_cfg_access(true);
        assert_eq!(reg.into_bytes()[0], 0x80);

        let reg = FuncCfgAccess::from_bytes([0xC0]);
        assert!(reg.shub_reg_access());
        assert!(reg.func_cfg_access());
    }

    #[test]
    fn test_set_shub_reg_access() {
        let mut i2c = Mock::new(&[
            Transaction::write_read(DEFAULT_I2C_ADDRESS, vec![Register::WhoAmI.addr()], vec![0x6b]),
            Transaction::write_read(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl3C.addr()], vec![0x00]),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl3C.addr(), 0x40]),
            Transaction::write_read(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl3C.addr()], vec![0x40]),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl3C.addr(), 0x44]),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr()],
                vec![0x00],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::FuncCfgAccess.addr(), 0x40]),
        ]);

        let sensor = Ism330Dhcx::new(&mut i2c).unwrap();
        sensor.set_shub_reg_access(&mut i2c, true).unwrap();
        i2c.done();
    }
}
