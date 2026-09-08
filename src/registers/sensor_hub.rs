//! Sensor-hub register definitions.

use modular_bitfield::{
    bitfield,
    specifiers::{B2, B3, B7, B8},
};

/// Registers in the sensor-hub bank.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SensorHubRegister {
    SensorHub1 = 0x02,
    SensorHub2,
    SensorHub3,
    SensorHub4,
    SensorHub5,
    SensorHub6,
    SensorHub7,
    SensorHub8,
    SensorHub9,
    SensorHub10,
    SensorHub11,
    SensorHub12,
    SensorHub13,
    SensorHub14,
    SensorHub15,
    SensorHub16,
    SensorHub17,
    SensorHub18,
    MasterConfig = 0x14,
    Slv0Add,
    Slv0Subadd,
    Slv0Config,
    Slv1Add,
    Slv1Subadd,
    Slv1Config,
    Slv2Add,
    Slv2Subadd,
    Slv2Config,
    Slv3Add,
    Slv3Subadd,
    Slv3Config,
    DataWriteSlv0,
    StatusMaster,
}

/// Raw sensor-hub data register.
pub type SensorHub1 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub2 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub3 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub4 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub5 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub6 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub7 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub8 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub9 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub10 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub11 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub12 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub13 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub14 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub15 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub16 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub17 = SensorHubByte;
/// Raw sensor-hub data register.
pub type SensorHub18 = SensorHubByte;

impl SensorHubRegister {
    /// Return the register address.
    pub const fn addr(self) -> u8 {
        self as u8
    }
}

/// Sensor-hub master configuration.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct MasterConfig {
    pub aux_sens_on: B2,
    pub master_on: bool,
    pub shub_pu_en: bool,
    pub pass_through_mode: bool,
    pub start_config: bool,
    pub write_once: bool,
    pub rst_master_regs: bool,
}

/// Sensor-hub slave address and direction.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Slv0Add {
    pub rw: bool,
    pub address: B7,
}

/// Sensor-hub slave-0 configuration.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Slv0Config {
    pub num_op: B3,
    pub batch_ext_sens_en: bool,
    #[skip]
    pub __: B2,
    pub shub_odr: B2,
}

/// Sensor-hub slave address and direction register.
pub type Slv1Add = Slv0Add;
/// Sensor-hub slave subaddress register.
pub type Slv1Subadd = SensorHubByte;
/// Sensor-hub slave configuration register.
pub type Slv1Config = Slv0Config;
/// Sensor-hub slave subaddress register.
pub type Slv0Subadd = SensorHubByte;
/// Sensor-hub slave address and direction register.
pub type Slv2Add = Slv0Add;
/// Sensor-hub slave subaddress register.
pub type Slv2Subadd = SensorHubByte;
/// Sensor-hub slave configuration register.
pub type Slv2Config = Slv0Config;
/// Sensor-hub slave address and direction register.
pub type Slv3Add = Slv0Add;
/// Sensor-hub slave subaddress register.
pub type Slv3Subadd = SensorHubByte;
/// Sensor-hub slave configuration register.
pub type Slv3Config = Slv0Config;
/// Sensor-hub data-write register.
pub type DataWriteSlv0 = SensorHubByte;

/// Raw byte in the sensor-hub register bank.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SensorHubByte {
    pub value: B8,
}

impl Default for SensorHubByte {
    fn default() -> Self {
        Self::new()
    }
}

/// Sensor-hub master status.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct StatusMaster {
    pub sens_hub_endop: bool,
    #[skip]
    pub __: B2,
    pub slave0_nack: bool,
    pub slave1_nack: bool,
    pub slave2_nack: bool,
    pub slave3_nack: bool,
    pub wr_once_done: bool,
}

impl Default for MasterConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Slv0Add {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Slv0Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for StatusMaster {
    fn default() -> Self {
        Self::new()
    }
}

/// Sensor-hub page register access mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SensorHubDirection {
    Read,
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensor_hub_layouts() {
        assert_eq!(MasterConfig::default().into_bytes(), [0]);
        assert_eq!(Slv0Add::default().into_bytes(), [0]);
        assert_eq!(Slv0Config::default().into_bytes(), [0]);
        assert_eq!(StatusMaster::default().into_bytes(), [0]);

        let addresses = [
            (SensorHubRegister::SensorHub1, 0x02),
            (SensorHubRegister::SensorHub2, 0x03),
            (SensorHubRegister::SensorHub3, 0x04),
            (SensorHubRegister::SensorHub4, 0x05),
            (SensorHubRegister::SensorHub5, 0x06),
            (SensorHubRegister::SensorHub6, 0x07),
            (SensorHubRegister::SensorHub7, 0x08),
            (SensorHubRegister::SensorHub8, 0x09),
            (SensorHubRegister::SensorHub9, 0x0A),
            (SensorHubRegister::SensorHub10, 0x0B),
            (SensorHubRegister::SensorHub11, 0x0C),
            (SensorHubRegister::SensorHub12, 0x0D),
            (SensorHubRegister::SensorHub13, 0x0E),
            (SensorHubRegister::SensorHub14, 0x0F),
            (SensorHubRegister::SensorHub15, 0x10),
            (SensorHubRegister::SensorHub16, 0x11),
            (SensorHubRegister::SensorHub17, 0x12),
            (SensorHubRegister::SensorHub18, 0x13),
            (SensorHubRegister::MasterConfig, 0x14),
            (SensorHubRegister::Slv0Add, 0x15),
            (SensorHubRegister::Slv0Subadd, 0x16),
            (SensorHubRegister::Slv0Config, 0x17),
            (SensorHubRegister::Slv1Add, 0x18),
            (SensorHubRegister::Slv1Subadd, 0x19),
            (SensorHubRegister::Slv1Config, 0x1A),
            (SensorHubRegister::Slv2Add, 0x1B),
            (SensorHubRegister::Slv2Subadd, 0x1C),
            (SensorHubRegister::Slv2Config, 0x1D),
            (SensorHubRegister::Slv3Add, 0x1E),
            (SensorHubRegister::Slv3Subadd, 0x1F),
            (SensorHubRegister::Slv3Config, 0x20),
            (SensorHubRegister::DataWriteSlv0, 0x21),
            (SensorHubRegister::StatusMaster, 0x22),
        ];

        for (register, address) in addresses {
            assert_eq!(register.addr(), address);
        }

        let mut address = Slv0Add::new();
        address.set_rw(true);
        address.set_address(0x3c);
        assert_eq!(address.into_bytes(), [0x79]);

        let mut config = Slv0Config::new();
        config.set_num_op(6);
        config.set_batch_ext_sens_en(true);
        config.set_shub_odr(2);
        assert_eq!(config.into_bytes(), [0x8e]);
    }
}
