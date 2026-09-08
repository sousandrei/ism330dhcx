use modular_bitfield::{bitfield, specifiers::B6};

/// Enable access to the embedded functions configuration registers. (01h)
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
