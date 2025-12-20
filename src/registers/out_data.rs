use modular_bitfield::prelude::*;

/// Gyroscope and Accelerometer output data registers (22h - 2Dh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct OutData {
    pub low: B8,
    pub high: B8,
}

impl OutData {
    pub fn get_value(&self) -> i16 {
        ((self.high() as u16) << 8 | (self.low() as u16)) as i16
    }
}
