use modular_bitfield::prelude::*;

/// FIFO sensor identifiers.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 5]
pub enum TagSensor {
    GyroNC = 0x01,
    AccelNC = 0x02,
    Temperature = 0x03,
    Timestamp = 0x04,
    CfgChange = 0x05,
    AccelNcT2 = 0x06,
    AccelNcT1 = 0x07,
    Accel2xC = 0x08,
    Accel3xC = 0x09,
    GyroNcT2 = 0x0A,
    GyroNcT1 = 0x0B,
    Gyro2xC = 0x0C,
    Gyro3xC = 0x0D,
    SensorHubSlave0 = 0x0E,
    SensorHubSlave1 = 0x0F,
    SensorHubSlave2 = 0x10,
    SensorHubSlave3 = 0x11,
    StepCounter = 0x12,
    SensorHubNack = 0x19,
}

/// FIFO tag register (78h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FifoDataOutTag {
    /// Parity check of TAG content.
    pub tag_parity: bool,
    /// 2-bit counter which identifies sensor time slot.
    pub tag_cnt: B2,
    /// Identifies the sensor.
    pub tag_sensor: TagSensor,
}

impl Default for FifoDataOutTag {
    fn default() -> Self {
        Self::new()
    }
}
