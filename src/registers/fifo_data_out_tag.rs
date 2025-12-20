use bitfield::bitfield;

/// FIFO sensor identifiers.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
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
    StepCounter = 0x12, // Wait, SensorHubSlave4 and StepCounter share 0x12?
    SensorHubNack = 0x19,
}

impl From<u8> for TagSensor {
    fn from(val: u8) -> Self {
        match val {
            0x01 => TagSensor::GyroNC,
            0x02 => TagSensor::AccelNC,
            0x03 => TagSensor::Temperature,
            0x04 => TagSensor::Timestamp,
            0x05 => TagSensor::CfgChange,
            0x06 => TagSensor::AccelNcT2,
            0x07 => TagSensor::AccelNcT1,
            0x08 => TagSensor::Accel2xC,
            0x09 => TagSensor::Accel3xC,
            0x0A => TagSensor::GyroNcT2,
            0x0B => TagSensor::GyroNcT1,
            0x0C => TagSensor::Gyro2xC,
            0x0D => TagSensor::Gyro3xC,
            0x0E => TagSensor::SensorHubSlave0,
            0x0F => TagSensor::SensorHubSlave1,
            0x10 => TagSensor::SensorHubSlave2,
            0x11 => TagSensor::SensorHubSlave3,
            0x12 => TagSensor::StepCounter,
            0x19 => TagSensor::SensorHubNack,
            _ => TagSensor::GyroNC,
        }
    }
}

impl From<TagSensor> for u8 {
    fn from(val: TagSensor) -> u8 {
        val as u8
    }
}

bitfield! {
    /// FIFO tag register (78h)
    pub struct FifoDataOutTag(u8);
    impl Debug;
    /// Parity check of TAG content.
    pub tag_parity, set_tag_parity: 0;
    /// 2-bit counter which identifies sensor time slot.
    pub tag_cnt, set_tag_cnt: 2, 1;
    /// Identifies the sensor.
    pub from into TagSensor, tag_sensor, set_tag_sensor: 7, 3;
}

impl FifoDataOutTag {
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

impl Default for FifoDataOutTag {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for FifoDataOutTag {}
impl Clone for FifoDataOutTag {
    fn clone(&self) -> Self {
        *self
    }
}
