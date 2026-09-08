use embedded_hal::i2c::I2c;

use crate::Ism330Dhcx;
use crate::registers::{
    MasterConfig, Register, SensorHubRegister, Slv0Add, Slv0Config, StatusMaster,
};

/// Sensor-hub scheduling and external-sensor access.
pub trait SensorHub {
    /// Enable or disable sensor-hub master operation.
    fn set_sensor_hub_enabled<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
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
        I2C: I2c;
    /// Configure a one-shot external-sensor register write.
    fn write_sensor_hub<I2C>(
        &self,
        i2c: &mut I2C,
        address: u8,
        register: u8,
        value: u8,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Read the most recently scheduled external-sensor data.
    fn read_sensor_hub<I2C>(&self, i2c: &mut I2C, buffer: &mut [u8]) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Read the sensor-hub operation status.
    fn sensor_hub_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusMaster, I2C::Error>
    where
        I2C: I2c;
}

impl Ism330Dhcx {
    fn with_sensor_hub_access<I2C, T, F>(
        &self,
        i2c: &mut I2C,
        operation: F,
    ) -> Result<T, I2C::Error>
    where
        I2C: I2c,
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
        I2C: I2c,
    {
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            let mut raw = [0u8];
            bus.write_read(
                sensor.address,
                &[SensorHubRegister::MasterConfig.addr()],
                &mut raw,
            )?;
            let mut config = MasterConfig::from_bytes(raw);
            config.set_master_on(enable);
            bus.write(
                sensor.address,
                &[
                    SensorHubRegister::MasterConfig.addr(),
                    config.into_bytes()[0],
                ],
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
        I2C: I2c,
    {
        assert!(address < 0x80, "sensor-hub address must be seven bits");
        assert!(
            (1..=8).contains(&length),
            "sensor-hub slave-0 read length must be 1 through 8"
        );
        assert!(odr < 4, "sensor-hub ODR must fit in two bits");
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            let mut address_reg = Slv0Add::new();
            address_reg.set_rw(true);
            address_reg.set_address(address);
            let mut config = Slv0Config::new();
            config.set_num_op(length - 1);
            config.set_batch_ext_sens_en(batch);
            config.set_shub_odr(odr);
            bus.write(
                sensor.address,
                &[
                    SensorHubRegister::Slv0Add.addr(),
                    address_reg.into_bytes()[0],
                ],
            )?;
            bus.write(
                sensor.address,
                &[SensorHubRegister::Slv0Subadd.addr(), register],
            )?;
            bus.write(
                sensor.address,
                &[SensorHubRegister::Slv0Config.addr(), config.into_bytes()[0]],
            )?;
            let mut master = MasterConfig::new();
            master.set_master_on(true);
            master.set_start_config(true);
            bus.write(
                sensor.address,
                &[
                    SensorHubRegister::MasterConfig.addr(),
                    master.into_bytes()[0],
                ],
            )
        })
    }

    fn write_sensor_hub<I2C>(
        &self,
        i2c: &mut I2C,
        address: u8,
        register: u8,
        value: u8,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        assert!(address < 0x80, "sensor-hub address must be seven bits");
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            let mut address_reg = Slv0Add::new();
            address_reg.set_address(address);
            let mut master = MasterConfig::new();
            master.set_master_on(true);
            master.set_start_config(true);
            master.set_write_once(true);
            bus.write(
                sensor.address,
                &[
                    SensorHubRegister::Slv0Add.addr(),
                    address_reg.into_bytes()[0],
                ],
            )?;
            bus.write(
                sensor.address,
                &[SensorHubRegister::Slv0Subadd.addr(), register],
            )?;
            bus.write(
                sensor.address,
                &[SensorHubRegister::DatawriteSlv0.addr(), value],
            )?;
            bus.write(
                sensor.address,
                &[
                    SensorHubRegister::MasterConfig.addr(),
                    master.into_bytes()[0],
                ],
            )
        })
    }

    fn read_sensor_hub<I2C>(&self, i2c: &mut I2C, buffer: &mut [u8]) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        assert!(
            (1..=18).contains(&buffer.len()),
            "sensor-hub data length must be 1 through 18"
        );
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            bus.write_read(
                sensor.address,
                &[SensorHubRegister::SensorHub1.addr()],
                buffer,
            )
        })
    }

    fn sensor_hub_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusMaster, I2C::Error>
    where
        I2C: I2c,
    {
        let mut raw = [0u8];
        self.with_sensor_hub_access(i2c, |sensor, bus| {
            bus.write_read(
                sensor.address,
                &[SensorHubRegister::StatusMaster.addr()],
                &mut raw,
            )
        })?;
        Ok(StatusMaster::from_bytes(raw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

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
                vec![SensorHubRegister::DatawriteSlv0.addr(), 0x55],
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
}
