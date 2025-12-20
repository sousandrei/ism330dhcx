use bitfield::bitfield;

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

impl Copy for FuncCfgAccess {}
impl Clone for FuncCfgAccess {
    fn clone(&self) -> Self {
        *self
    }
}
