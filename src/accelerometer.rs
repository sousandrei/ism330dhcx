use crate::registers::FsXl;

pub const SENSORS_GRAVITY_STANDARD: f64 = 9.80665;

#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct AccelValue {
    range: FsXl,
    count: [i16; 3],
}

impl AccelValue {
    pub fn new(range: FsXl, count: [i16; 3]) -> AccelValue {
        AccelValue { range, count }
    }

    pub fn from_msr(range: FsXl, measurements: &[u8; 6]) -> AccelValue {
        let raw_acc_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_acc_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_acc_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);
        AccelValue {
            range,
            count: [raw_acc_x, raw_acc_y, raw_acc_z],
        }
    }

    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// As [m/s^2]
    pub fn as_m_ss(&self) -> [f64; 3] {
        self.as_mg().map(|v| v * SENSORS_GRAVITY_STANDARD / 1000.)
    }

    /// As [milli-g]
    pub fn as_mg(&self) -> [f64; 3] {
        let sensitivity = self.range.sensitivity() as f64;
        self.count.map(|r| r as f64 * sensitivity)
    }

    /// As [g]
    pub fn as_g(&self) -> [f64; 3] {
        self.as_mg().map(|v| v / 1000.)
    }
}
