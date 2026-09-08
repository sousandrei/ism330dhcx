//! Sensor-hub register definitions.

use modular_bitfield::{
    bitfield,
    specifiers::{B2, B3, B7},
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
    DatawriteSlv0,
    StatusMaster,
}

impl SensorHubRegister {
    /// Return the register address.
    pub const fn addr(self) -> u8 {
        self as u8
    }
}

/// Sensor-hub master configuration.
#[bitfield]
#[derive(Debug, Copy, Clone)]
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
#[derive(Debug, Copy, Clone)]
pub struct Slv0Add {
    pub rw: bool,
    pub address: B7,
}

/// Sensor-hub slave-0 configuration.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Slv0Config {
    pub num_op: B3,
    pub batch_ext_sens_en: bool,
    #[skip]
    pub __: B2,
    pub shub_odr: B2,
}

/// Sensor-hub master status.
#[bitfield]
#[derive(Debug, Copy, Clone)]
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
        assert_eq!(SensorHubRegister::SensorHub1.addr(), 0x02);
        assert_eq!(SensorHubRegister::SensorHub18.addr(), 0x13);
        assert_eq!(SensorHubRegister::StatusMaster.addr(), 0x22);

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
