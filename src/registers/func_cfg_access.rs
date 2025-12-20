use modular_bitfield::prelude::*;

/// Enable embedded functions register (01h)
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
