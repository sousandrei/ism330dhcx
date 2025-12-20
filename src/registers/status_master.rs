use bitfield::bitfield;

bitfield! {
    /// Sensor hub source register (39h)
    pub struct StatusMasterMainpage(u8);
    impl Debug;
    /// Sensor hub communication status
    pub sh_end_op, set_sh_end_op: 0;
    /// Slave0 NACK status
    pub slave0_nack, set_slave0_nack: 1;
    /// Slave1 NACK status
    pub slave1_nack, set_slave1_nack: 2;
    /// Slave2 NACK status
    pub slave2_nack, set_slave2_nack: 3;
    /// Slave3 NACK status
    pub slave3_nack, set_slave3_nack: 4;
    /// Write Once status flag
    pub wr_once_done, set_wr_once_done: 5;
}

impl StatusMasterMainpage {
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

impl Default for StatusMasterMainpage {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for StatusMasterMainpage {}
impl Clone for StatusMasterMainpage {
    fn clone(&self) -> Self {
        *self
    }
}
