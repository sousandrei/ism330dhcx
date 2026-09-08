use crate::registers::{
    MasterConfig, Register, SensorHubRegister, Slv0Add, Slv0Config, StatusMaster,
};
use crate::{Ism330Dhcx, RegisterBus};

/// Sensor-hub external slave selector.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SensorHubSlave {
    Slave0,
    Slave1,
    Slave2,
    Slave3,
}

/// Periodic read configuration for one sensor-hub slave.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SensorHubReadConfig {
    pub slave: SensorHubSlave,
    pub address: u8,
    pub register: u8,
    pub length: u8,
    pub odr: u8,
    pub batch: bool,
}

impl SensorHubSlave {
    const fn registers(self) -> (u8, u8, u8) {
        match self {
            Self::Slave0 => (
                SensorHubRegister::Slv0Add.addr(),
                SensorHubRegister::Slv0Subadd.addr(),
                SensorHubRegister::Slv0Config.addr(),
            ),
            Self::Slave1 => (
                SensorHubRegister::Slv1Add.addr(),
                SensorHubRegister::Slv1Subadd.addr(),
                SensorHubRegister::Slv1Config.addr(),
            ),
            Self::Slave2 => (
                SensorHubRegister::Slv2Add.addr(),
                SensorHubRegister::Slv2Subadd.addr(),
                SensorHubRegister::Slv2Config.addr(),
            ),
            Self::Slave3 => (
                SensorHubRegister::Slv3Add.addr(),
                SensorHubRegister::Slv3Subadd.addr(),
                SensorHubRegister::Slv3Config.addr(),
            ),
        }
    }
}

/// Sensor-hub scheduling and external-sensor access.
pub trait SensorHub {
    /// Enable or disable sensor-hub master operation.
    fn set_sensor_hub_enabled<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Configure any sensor-hub slave for periodic reads.
    fn configure_sensor_hub_slave<I2C>(
        &self,
        i2c: &mut I2C,
        config: SensorHubReadConfig,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Configure slave 0 for periodic reads into `SENSOR_HUB_1..18`.
    fn configure_sensor_hub_read<I2C>(
        &self,
        i2c: &mut I2C,
        address: u8,
        register: u8,
        length: u8,
        odr: u8,
        batch: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Configure a one-shot external-sensor register write.
    fn write_sensor_hub<I2C>(
        &self,
        i2c: &mut I2C,
        address: u8,
        register: u8,
        value: u8,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the most recently scheduled external-sensor data.
    fn read_sensor_hub<I2C>(&self, i2c: &mut I2C, buffer: &mut [u8]) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the sensor-hub operation status.
    fn sensor_hub_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusMaster, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read sensor-hub operation status from the main register page.
    fn sensor_hub_status_mainpage<I2C>(&self, i2c: &mut I2C) -> Result<StatusMaster, I2C::Error>
    where
        I2C: RegisterBus;
    /// Return the NACK state for slaves 0 through 3.
    fn sensor_hub_nack_status<I2C>(&self, i2c: &mut I2C) -> Result<[bool; 4], I2C::Error>
    where
        I2C: RegisterBus;
    /// Enable or disable the sensor-hub auxiliary-sensor mode.
    fn set_sensor_hub_auxiliary_sensor<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Enable or disable the sensor-hub internal pull-ups.
    fn set_sensor_hub_pull_up<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Enable or disable sensor-hub pass-through mode.
    fn set_sensor_hub_pass_through<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Select whether the next master operation starts from the configured sequence.
    fn set_sensor_hub_start_config<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Enable or disable the one-shot slave-0 write mode.
    fn set_sensor_hub_write_once<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Reset the sensor-hub register bank.
    fn reset_sensor_hub_registers<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
}

impl Ism330Dhcx {
    fn with_sensor_hub_access<I2C, T, F>(
        &self,
        i2c: &mut I2C,
        operation: F,
    ) -> Result<T, I2C::Error>
    where
        I2C: RegisterBus,
        F: FnOnce(&Self, &mut I2C) -> Result<T, I2C::Error>,
    {
        let access = self.read_reg(i2c, Register::FuncCfgAccess)?;
        self.write_reg(i2c, Register::FuncCfgAccess, access | 0x40)?;
        let result = operation(self, i2c);
        let restore = self.write_reg(i2c, Register::FuncCfgAccess, access);
        match result {
            Err(error) => Err(error),
            Ok(value) => restore.map(|()| value),
        }
    }
}

impl SensorHub for Ism330Dhcx {
    fn set_sensor_hub_enabled<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            let mut raw = [0u8];
            bus.read_register(
                sensor.address,
                SensorHubRegister::MasterConfig.addr(),
                &mut raw,
            )?;
            let mut config = MasterConfig::from_bytes(raw);
            config.set_master_on(enable);
            bus.write_register(
                sensor.address,
                SensorHubRegister::MasterConfig.addr(),
                &[config.into_bytes()[0]],
            )
        })
    }

    fn configure_sensor_hub_slave<I2C>(
        &self,
        i2c: &mut I2C,
        config: SensorHubReadConfig,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        assert!(
            config.address < 0x80,
            "sensor-hub address must be seven bits"
        );
        assert!(
            (1..=8).contains(&config.length),
            "sensor-hub read length must be 1 through 8"
        );
        assert!(config.odr < 4, "sensor-hub ODR must fit in two bits");
        let (address_register, subaddress_register, config_register) = config.slave.registers();

        self.with_sensor_hub_access(i2c, |sensor, bus| {
            let mut address_reg = Slv0Add::new();
            address_reg.set_rw(true);
            address_reg.set_address(config.address);
            let mut slave_config = Slv0Config::new();
            slave_config.set_num_op(config.length - 1);
            slave_config.set_batch_ext_sens_en(config.batch);
            slave_config.set_shub_odr(config.odr);

            bus.write_register(
                sensor.address,
                address_register,
                &[address_reg.into_bytes()[0]],
            )?;
            bus.write_register(sensor.address, subaddress_register, &[config.register])?;
            bus.write_register(
                sensor.address,
                config_register,
                &[slave_config.into_bytes()[0]],
            )?;
            let mut raw = [0u8];
            bus.read_register(
                sensor.address,
                SensorHubRegister::MasterConfig.addr(),
                &mut raw,
            )?;
            let mut master = MasterConfig::from_bytes(raw);
            master.set_master_on(true);
            master.set_start_config(true);
            bus.write_register(
                sensor.address,
                SensorHubRegister::MasterConfig.addr(),
                &[master.into_bytes()[0]],
            )
        })
    }

    fn configure_sensor_hub_read<I2C>(
        &self,
        i2c: &mut I2C,
        address: u8,
        register: u8,
        length: u8,
        odr: u8,
        batch: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.configure_sensor_hub_slave(
            i2c,
            SensorHubReadConfig {
                slave: SensorHubSlave::Slave0,
                address,
                register,
                length,
                odr,
                batch,
            },
        )
    }

    fn write_sensor_hub<I2C>(
        &self,
        i2c: &mut I2C,
        address: u8,
        register: u8,
        value: u8,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        assert!(address < 0x80, "sensor-hub address must be seven bits");
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            let mut address_reg = Slv0Add::new();
            address_reg.set_address(address);
            let mut master = MasterConfig::new();
            master.set_master_on(true);
            master.set_start_config(true);
            master.set_write_once(true);
            bus.write_register(
                sensor.address,
                SensorHubRegister::Slv0Add.addr(),
                &[address_reg.into_bytes()[0]],
            )?;
            bus.write_register(
                sensor.address,
                SensorHubRegister::Slv0Subadd.addr(),
                &[register],
            )?;
            bus.write_register(
                sensor.address,
                SensorHubRegister::DataWriteSlv0.addr(),
                &[value],
            )?;
            bus.write_register(
                sensor.address,
                SensorHubRegister::MasterConfig.addr(),
                &[master.into_bytes()[0]],
            )
        })
    }

    fn read_sensor_hub<I2C>(&self, i2c: &mut I2C, buffer: &mut [u8]) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        assert!(
            (1..=18).contains(&buffer.len()),
            "sensor-hub data length must be 1 through 18"
        );
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            bus.read_register(sensor.address, SensorHubRegister::SensorHub1.addr(), buffer)
        })
    }

    fn sensor_hub_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusMaster, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8];
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            bus.read_register(
                sensor.address,
                SensorHubRegister::StatusMaster.addr(),
                &mut raw,
            )
        })?;
        Ok(StatusMaster::from_bytes(raw))
    }

    fn sensor_hub_status_mainpage<I2C>(&self, i2c: &mut I2C) -> Result<StatusMaster, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8];
        self.read_register(i2c, Register::StatusMasterMainpage.addr(), &mut raw)?;
        Ok(StatusMaster::from_bytes(raw))
    }

    fn sensor_hub_nack_status<I2C>(&self, i2c: &mut I2C) -> Result<[bool; 4], I2C::Error>
    where
        I2C: RegisterBus,
    {
        let status = self.sensor_hub_status_mainpage(i2c)?;
        Ok([
            status.slave0_nack(),
            status.slave1_nack(),
            status.slave2_nack(),
            status.slave3_nack(),
        ])
    }

    fn set_sensor_hub_auxiliary_sensor<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_sensor_hub_config(i2c, |config| {
            config.set_aux_sens_on(if enable { 3 } else { 0 })
        })
    }

    fn set_sensor_hub_pull_up<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_sensor_hub_config(i2c, |config| config.set_shub_pu_en(enable))
    }

    fn set_sensor_hub_pass_through<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_sensor_hub_config(i2c, |config| config.set_pass_through_mode(enable))
    }

    fn set_sensor_hub_start_config<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_sensor_hub_config(i2c, |config| config.set_start_config(enable))
    }

    fn set_sensor_hub_write_once<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_sensor_hub_config(i2c, |config| config.set_write_once(enable))
    }

    fn reset_sensor_hub_registers<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.modify_sensor_hub_config(i2c, |config| config.set_rst_master_regs(true))
    }
}

impl Ism330Dhcx {
    fn modify_sensor_hub_config<I2C, F>(&self, i2c: &mut I2C, modify: F) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
        F: FnOnce(&mut MasterConfig),
    {
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            let mut raw = [0u8];
            bus.read_register(
                sensor.address,
                SensorHubRegister::MasterConfig.addr(),
                &mut raw,
            )?;
            let mut config = MasterConfig::from_bytes(raw);
            modify(&mut config);
            bus.write_register(
                sensor.address,
                SensorHubRegister::MasterConfig.addr(),
                &config.into_bytes(),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
    use std::vec;

    #[test]
    fn configures_external_read_and_restores_access_mode() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr()],
                vec![0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x40],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::Slv0Add.addr(), 0xD1],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::Slv0Subadd.addr(), 0x20],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::Slv0Config.addr(), 0x03],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::MasterConfig.addr()],
                vec![0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::MasterConfig.addr(), 0x24],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x00],
            ),
        ]);

        sensor
            .configure_sensor_hub_read(&mut i2c, 0x68, 0x20, 4, 0, false)
            .unwrap();
        i2c.done();
    }

    #[test]
    fn writes_external_register_through_sensor_hub() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr()],
                vec![0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x40],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::Slv0Add.addr(), 0x68],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::Slv0Subadd.addr(), 0x10],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::DataWriteSlv0.addr(), 0x55],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::MasterConfig.addr(), 0x64],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x00],
            ),
        ]);

        sensor.write_sensor_hub(&mut i2c, 0x34, 0x10, 0x55).unwrap();
        i2c.done();
    }

    #[test]
    fn configures_slave_one_independently() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr()],
                vec![0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x40],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::Slv1Add.addr(), 0xD1],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::Slv1Subadd.addr(), 0x30],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::Slv1Config.addr(), 0x8B],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::MasterConfig.addr()],
                vec![0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![SensorHubRegister::MasterConfig.addr(), 0x24],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x00],
            ),
        ]);

        sensor
            .configure_sensor_hub_slave(
                &mut i2c,
                SensorHubReadConfig {
                    slave: SensorHubSlave::Slave1,
                    address: 0x68,
                    register: 0x30,
                    length: 4,
                    odr: 2,
                    batch: true,
                },
            )
            .unwrap();
        i2c.done();
    }

    #[test]
    fn reads_mainpage_nack_status() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::StatusMasterMainpage.addr()],
            vec![0x50],
        )]);

        assert_eq!(
            sensor.sensor_hub_nack_status(&mut i2c).unwrap(),
            [false, true, false, true]
        );
        i2c.done();
    }

    #[test]
    fn configures_sensor_hub_slave_over_spi() {
        let spi = SpiMock::new(&[
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(vec![0x81, 0], vec![0, 0]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x01, 0x40]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x1b, 0xD1]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x1c, 0x20]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x1d, 0x41]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(vec![0x94, 0], vec![0, 0]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x14, 0x24]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x01, 0x00]),
            SpiTransaction::transaction_end(),
        ]);
        let mut bus = crate::SpiDeviceBus::new(spi);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        sensor
            .configure_sensor_hub_slave(
                &mut bus,
                SensorHubReadConfig {
                    slave: SensorHubSlave::Slave2,
                    address: 0x68,
                    register: 0x20,
                    length: 2,
                    odr: 1,
                    batch: false,
                },
            )
            .unwrap();
        bus.into_inner().done();
    }
}
