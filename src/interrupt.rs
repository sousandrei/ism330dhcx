use crate::RegisterBus;

use crate::Ism330Dhcx;
use crate::registers::{
    AllIntSrc, Ctrl1Xl, Ctrl2G, D6dSrc, Int1Ctrl, Int2Ctrl, Md1Cfg, Md2Cfg, Register, StatusReg,
    TapSrc, WakeUpSrc,
};

/// Interrupt routing configuration.
pub trait Interrupts {
    fn set_int1_boot<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_drdy_xl<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_drdy_g<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_fifo_threshold<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_fifo_overrun<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_fifo_full<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_counter_bdr<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_den_drdy<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_drdy_xl<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_drdy_g<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_drdy_temp<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_fifo_threshold<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_fifo_overrun<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_fifo_full<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_counter_bdr<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int1_embedded_function<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_int2_embedded_function<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
}

/// Read interrupt routing and event-source registers.
pub trait InterruptStatus {
    /// Read the combined interrupt source register.
    fn get_interrupt_sources<I2C>(&self, i2c: &mut I2C) -> Result<AllIntSrc, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the wake-up source register.
    fn get_wake_up_source<I2C>(&self, i2c: &mut I2C) -> Result<WakeUpSrc, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the tap source register.
    fn get_tap_source<I2C>(&self, i2c: &mut I2C) -> Result<TapSrc, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the 6D source register.
    fn get_6d_source<I2C>(&self, i2c: &mut I2C) -> Result<D6dSrc, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read INT1 routing configuration.
    fn get_int1_routing<I2C>(&self, i2c: &mut I2C) -> Result<Int1Ctrl, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read INT2 routing configuration.
    fn get_int2_routing<I2C>(&self, i2c: &mut I2C) -> Result<Int2Ctrl, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read embedded-event routing for INT1.
    fn get_int1_event_routing<I2C>(&self, i2c: &mut I2C) -> Result<Md1Cfg, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read embedded-event routing for INT2.
    fn get_int2_event_routing<I2C>(&self, i2c: &mut I2C) -> Result<Md2Cfg, I2C::Error>
    where
        I2C: RegisterBus;
    fn get_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusReg, I2C::Error>
    where
        I2C: RegisterBus;
    fn get_timestamp<I2C>(&self, i2c: &mut I2C) -> Result<u32, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the signed internal frequency correction value.
    fn get_internal_frequency_fine<I2C>(&self, i2c: &mut I2C) -> Result<i8, I2C::Error>
    where
        I2C: RegisterBus;
    /// Return the configured accelerometer ODR corrected by the internal clock.
    fn get_actual_accel_odr<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: RegisterBus;
    /// Return the configured gyroscope ODR corrected by the internal clock.
    fn get_actual_gyro_odr<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: RegisterBus;
    /// Return the corrected timestamp tick period in microseconds.
    fn get_timestamp_resolution_us<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the timestamp and convert it to seconds using the corrected period.
    fn get_timestamp_seconds<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: RegisterBus;
}

/// Calculate the timestamp tick period from `INTERNAL_FREQ_FINE`.
pub fn timestamp_resolution_us(frequency_fine: i8) -> f32 {
    1_000_000.0 / (40_000.0 * (1.0 + 0.0015 * frequency_fine as f32))
}

impl InterruptStatus for Ism330Dhcx {
    fn get_interrupt_sources<I2C>(&self, i2c: &mut I2C) -> Result<AllIntSrc, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(AllIntSrc::from_bytes([
            self.read_reg(i2c, Register::AllIntSrc)?
        ]))
    }

    fn get_wake_up_source<I2C>(&self, i2c: &mut I2C) -> Result<WakeUpSrc, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(WakeUpSrc::from_bytes([
            self.read_reg(i2c, Register::WakeUpSrc)?
        ]))
    }

    fn get_tap_source<I2C>(&self, i2c: &mut I2C) -> Result<TapSrc, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(TapSrc::from_bytes([self.read_reg(i2c, Register::TapSrc)?]))
    }

    fn get_6d_source<I2C>(&self, i2c: &mut I2C) -> Result<D6dSrc, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(D6dSrc::from_bytes([self.read_reg(i2c, Register::D6dSrc)?]))
    }

    fn get_int1_routing<I2C>(&self, i2c: &mut I2C) -> Result<Int1Ctrl, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Int1Ctrl::from_bytes([
            self.read_reg(i2c, Register::Int1Ctrl)?
        ]))
    }

    fn get_int2_routing<I2C>(&self, i2c: &mut I2C) -> Result<Int2Ctrl, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Int2Ctrl::from_bytes([
            self.read_reg(i2c, Register::Int2Ctrl)?
        ]))
    }

    fn get_int1_event_routing<I2C>(&self, i2c: &mut I2C) -> Result<Md1Cfg, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Md1Cfg::from_bytes([self.read_reg(i2c, Register::Md1Cfg)?]))
    }

    fn get_int2_event_routing<I2C>(&self, i2c: &mut I2C) -> Result<Md2Cfg, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(Md2Cfg::from_bytes([self.read_reg(i2c, Register::Md2Cfg)?]))
    }

    fn get_status<I2C>(&self, i2c: &mut I2C) -> Result<StatusReg, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(StatusReg::from_bytes([
            self.read_reg(i2c, Register::StatusReg)?
        ]))
    }

    fn get_timestamp<I2C>(&self, i2c: &mut I2C) -> Result<u32, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut bytes = [0u8; 3];
        self.read_register(i2c, Register::Timestamp0.addr(), &mut bytes)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], 0]))
    }

    fn get_internal_frequency_fine<I2C>(&self, i2c: &mut I2C) -> Result<i8, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(self.read_reg(i2c, Register::InternalFreqFine)? as i8)
    }

    fn get_actual_accel_odr<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let fine = self.get_internal_frequency_fine(i2c)?;
        let ctrl = Ctrl1Xl::from_bytes([self.read_reg(i2c, Register::Ctrl1Xl)?]);
        Ok(ctrl.odr_xl().actual_hz(fine))
    }

    fn get_actual_gyro_odr<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let fine = self.get_internal_frequency_fine(i2c)?;
        let ctrl = Ctrl2G::from_bytes([self.read_reg(i2c, Register::Ctrl2G)?]);
        Ok(ctrl.odr_g().actual_hz(fine))
    }

    fn get_timestamp_resolution_us<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(timestamp_resolution_us(
            self.get_internal_frequency_fine(i2c)?,
        ))
    }

    fn get_timestamp_seconds<I2C>(&self, i2c: &mut I2C) -> Result<f32, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let timestamp = self.get_timestamp(i2c)?;
        let resolution_us = self.get_timestamp_resolution_us(i2c)?;
        Ok(timestamp as f32 * resolution_us / 1_000_000.0)
    }
}

macro_rules! route_setters {
    ($(fn $method:ident, $register:ident, $config:ident, $setter:ident;)*) => {
        $(
            fn $method<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
            where
                I2C: RegisterBus,
            {
                self.modify_reg(i2c, Register::$register, |value| {
                    let mut reg = $config::from_bytes([value]);
                    reg.$setter(enable);
                    reg.into_bytes()[0]
                })
            }
        )*
    };
}

impl Interrupts for Ism330Dhcx {
    route_setters! {
        fn set_int1_boot, Int1Ctrl, Int1Ctrl, set_int1_boot;
        fn set_int1_drdy_xl, Int1Ctrl, Int1Ctrl, set_int1_drdy_xl;
        fn set_int1_drdy_g, Int1Ctrl, Int1Ctrl, set_int1_drdy_g;
        fn set_int1_fifo_threshold, Int1Ctrl, Int1Ctrl, set_int1_fifo_th;
        fn set_int1_fifo_overrun, Int1Ctrl, Int1Ctrl, set_int1_fifo_ovr;
        fn set_int1_fifo_full, Int1Ctrl, Int1Ctrl, set_int1_fifo_full;
        fn set_int1_counter_bdr, Int1Ctrl, Int1Ctrl, set_int1_cnt_bdr;
        fn set_int1_den_drdy, Int1Ctrl, Int1Ctrl, set_den_drdy_flag;
        fn set_int2_drdy_xl, Int2Ctrl, Int2Ctrl, set_int2_drdy_xl;
        fn set_int2_drdy_g, Int2Ctrl, Int2Ctrl, set_int2_drdy_g;
        fn set_int2_drdy_temp, Int2Ctrl, Int2Ctrl, set_int2_drdy_temp;
        fn set_int2_fifo_threshold, Int2Ctrl, Int2Ctrl, set_int2_fifo_th;
        fn set_int2_fifo_overrun, Int2Ctrl, Int2Ctrl, set_int2_fifo_ovr;
        fn set_int2_fifo_full, Int2Ctrl, Int2Ctrl, set_int2_fifo_full;
        fn set_int2_counter_bdr, Int2Ctrl, Int2Ctrl, set_int2_cnt_bdr;
        fn set_int1_embedded_function, Md1Cfg, Md1Cfg, set_int1_emb_func;
        fn set_int2_embedded_function, Md2Cfg, Md2Cfg, set_int2_emb_func;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn routes_fifo_threshold_to_int1() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Int1Ctrl.addr()],
                vec![0x40],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Int1Ctrl.addr(), 0x48]),
        ]);

        sensor.set_int1_fifo_threshold(&mut i2c, true).unwrap();
        i2c.done();
    }

    #[test]
    fn routes_embedded_function_to_int2() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Md2Cfg.addr()],
                vec![0x01],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Md2Cfg.addr(), 0x41]),
        ]);

        sensor.set_int2_embedded_function(&mut i2c, true).unwrap();
        i2c.done();
    }

    #[test]
    fn routes_boot_status_to_int1() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Int1Ctrl.addr()],
                vec![0x00],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Int1Ctrl.addr(), 0x04]),
        ]);

        sensor.set_int1_boot(&mut i2c, true).unwrap();
        i2c.done();
    }

    #[test]
    fn calculates_corrected_odr_and_timestamp_period() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::InternalFreqFine.addr()],
                vec![10],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl1Xl.addr()],
                vec![0x30],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Timestamp0.addr()],
                vec![0x34, 0x12, 0],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::InternalFreqFine.addr()],
                vec![10],
            ),
        ]);

        assert!((sensor.get_actual_accel_odr(&mut i2c).unwrap() - 52.87).abs() < 0.01);
        assert!((sensor.get_timestamp_seconds(&mut i2c).unwrap() - 0.1148).abs() < 0.001);
        i2c.done();
    }
}
