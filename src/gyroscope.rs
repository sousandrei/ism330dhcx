use crate::registers::FsG;

pub const SENSORS_DPS_TO_RADS: f64 = 0.017453292;

#[derive(Copy, Clone, Debug, defmt::Format)]
pub struct GyroValue {
    range: FsG,
    count: [i16; 3],
}

impl GyroValue {
    pub fn new(range: FsG, count: [i16; 3]) -> GyroValue {
        GyroValue { range, count }
    }

    pub fn from_msr(range: FsG, measurements: &[u8; 6]) -> GyroValue {
        let raw_gyro_x = (measurements[1] as i16) << 8 | (measurements[0] as i16);
        let raw_gyro_y = (measurements[3] as i16) << 8 | (measurements[2] as i16);
        let raw_gyro_z = (measurements[5] as i16) << 8 | (measurements[4] as i16);
        GyroValue {
            range,
            count: [raw_gyro_x, raw_gyro_y, raw_gyro_z],
        }
    }

    pub fn count(&self) -> [i16; 3] {
        self.count
    }

    /// As radians [rad]
    pub fn as_rad(&self) -> [f64; 3] {
        self.as_mdps().map(|v| v * SENSORS_DPS_TO_RADS / 1000.)
    }

    /// As milli degrees per second [mdps]
    pub fn as_mdps(&self) -> [f64; 3] {
        let sensitivity = self.range.sensitivity() as f64;
        self.count.map(|r| r as f64 * sensitivity)
    }

    /// As degrees per second [dps]
    pub fn as_dps(&self) -> [f64; 3] {
        self.as_mdps().map(|v| v / 1000.)
    }
}
