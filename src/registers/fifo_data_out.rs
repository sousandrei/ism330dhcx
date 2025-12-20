use bitfield::bitfield;

// TODO: one of this for each axis: X, Y, Z

bitfield! {
    /// FIFO data out registers
    pub struct FifoDataOut(u16);
    impl Debug;
    /// FIFO data output value
    pub fifo_data_out, set_fifo_data_out: 15, 0;
}

impl FifoDataOut {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u16; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u16; 1] {
        [self.0]
    }
}

impl Default for FifoDataOut {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for FifoDataOut {}
impl Clone for FifoDataOut {
    fn clone(&self) -> Self {
        *self
    }
}
