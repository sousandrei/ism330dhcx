use embedded_hal::i2c::I2c;

use crate::Ism330Dhcx;
use crate::registers::{
    EmbFuncEnA, EmbFuncEnB, EmbFuncFifoCfg, EmbFuncInitA, EmbFuncInitB, EmbFuncInt, EmbFuncOdrCfgB,
    EmbFuncOdrCfgC, EmbFuncSrc, EmbFuncStatus, EmbeddedRegister, PageSel, Register,
};

/// Embedded-function page and register access.
pub trait EmbeddedFunctions {
    /// Select an embedded-function page (0 through 15).
    fn set_embedded_page<I2C>(&self, i2c: &mut I2C, page: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Read an embedded-function register while preserving the access mode.
    fn read_embedded_register<I2C>(
        &self,
        i2c: &mut I2C,
        register: EmbeddedRegister,
        buffer: &mut [u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Write an embedded-function register while preserving the access mode.
    fn write_embedded_register<I2C>(
        &self,
        i2c: &mut I2C,
        register: EmbeddedRegister,
        value: &[u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_pedometer<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_tilt<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_significant_motion<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_fsm<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_mlc<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_fifo_compression<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_pedometer_fifo<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_fsm_odr<I2C>(&self, i2c: &mut I2C, odr: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_mlc_odr<I2C>(&self, i2c: &mut I2C, odr: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn read_embedded_status<I2C>(&self, i2c: &mut I2C) -> Result<EmbFuncStatus, I2C::Error>
    where
        I2C: I2c;
    fn read_embedded_source<I2C>(&self, i2c: &mut I2C) -> Result<EmbFuncSrc, I2C::Error>
    where
        I2C: I2c;
    fn read_step_counter<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c;
    fn read_fsm_outputs<I2C>(&self, i2c: &mut I2C) -> Result<[u8; 16], I2C::Error>
    where
        I2C: I2c;
    fn read_mlc_sources<I2C>(&self, i2c: &mut I2C) -> Result<[u8; 8], I2C::Error>
    where
        I2C: I2c;
    fn set_fsm_enabled<I2C>(
        &self,
        i2c: &mut I2C,
        program: u8,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn clear_fsm_long_counter<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_int1_step_detector<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_int1_tilt<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_int1_significant_motion<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_int1_fsm_long_counter<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_int2_step_detector<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_int2_tilt<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_int2_significant_motion<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_embedded_int2_fsm_long_counter<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn initialize_step_detector<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn initialize_tilt<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn initialize_significant_motion<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn initialize_fsm<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn initialize_mlc<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl Ism330Dhcx {
    fn with_embedded_access<I2C, T, F>(&self, i2c: &mut I2C, operation: F) -> Result<T, I2C::Error>
    where
        I2C: I2c,
        F: FnOnce(&Self, &mut I2C) -> Result<T, I2C::Error>,
    {
        let access = self.read_reg(i2c, Register::FuncCfgAccess)?;
        self.write_reg(i2c, Register::FuncCfgAccess, access | 0x80)?;
        let result = operation(self, i2c);
        let restore = self.write_reg(i2c, Register::FuncCfgAccess, access);
        match result {
            Err(error) => Err(error),
            Ok(value) => restore.map(|()| value),
        }
    }
}

macro_rules! embedded_enable_setter {
    ($method:ident, $config:ident, $setter:ident, $register:ident) => {
        fn $method<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
        where
            I2C: I2c,
        {
            self.with_embedded_access(i2c, |sensor, bus| {
                let mut raw = [0u8];
                bus.write_read(
                    sensor.address,
                    &[EmbeddedRegister::$register.addr()],
                    &mut raw,
                )?;
                let mut reg = $config::from_bytes(raw);
                reg.$setter(enable);
                bus.write(
                    sensor.address,
                    &[EmbeddedRegister::$register.addr(), reg.into_bytes()[0]],
                )
            })
        }
    };
}

macro_rules! embedded_initializer {
    ($method:ident, $config:ident, $setter:ident, $register:ident) => {
        fn $method<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
        where
            I2C: I2c,
        {
            self.with_embedded_access(i2c, |sensor, bus| {
                let mut raw = [0u8];
                bus.write_read(
                    sensor.address,
                    &[EmbeddedRegister::$register.addr()],
                    &mut raw,
                )?;
                let mut reg = $config::from_bytes(raw);
                reg.$setter(true);
                bus.write(
                    sensor.address,
                    &[EmbeddedRegister::$register.addr(), reg.into_bytes()[0]],
                )
            })
        }
    };
}

impl EmbeddedFunctions for Ism330Dhcx {
    fn set_embedded_page<I2C>(&self, i2c: &mut I2C, page: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        assert!(page < 16, "embedded page must fit in four bits");
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut reg = PageSel::new();
            reg.set_page_sel(page);
            bus.write(
                sensor.address,
                &[EmbeddedRegister::PageSel.addr(), reg.into_bytes()[0]],
            )
        })
    }

    fn read_embedded_register<I2C>(
        &self,
        i2c: &mut I2C,
        register: EmbeddedRegister,
        buffer: &mut [u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.with_embedded_access(i2c, |sensor, bus| {
            bus.write_read(sensor.address, &[register.addr()], buffer)
        })
    }

    fn write_embedded_register<I2C>(
        &self,
        i2c: &mut I2C,
        register: EmbeddedRegister,
        value: &[u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        assert!(!value.is_empty(), "embedded register writes need a value");
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut bytes = [0u8; 33];
            assert!(value.len() < bytes.len(), "embedded write is too long");
            bytes[0] = register.addr();
            bytes[1..value.len() + 1].copy_from_slice(value);
            bus.write(sensor.address, &bytes[..value.len() + 1])
        })
    }

    embedded_enable_setter!(set_pedometer, EmbFuncEnA, set_pedo_en, EmbFuncEnA);
    embedded_enable_setter!(set_tilt, EmbFuncEnA, set_tilt_en, EmbFuncEnA);
    embedded_enable_setter!(
        set_embedded_significant_motion,
        EmbFuncEnA,
        set_sign_motion_en,
        EmbFuncEnA
    );
    embedded_enable_setter!(set_fsm, EmbFuncEnB, set_fsm_en, EmbFuncEnB);
    embedded_enable_setter!(set_mlc, EmbFuncEnB, set_mlc_en, EmbFuncEnB);
    embedded_enable_setter!(
        set_embedded_fifo_compression,
        EmbFuncEnB,
        set_fifo_compr_en,
        EmbFuncEnB
    );
    embedded_enable_setter!(
        set_pedometer_fifo,
        EmbFuncFifoCfg,
        set_pedo_fifo_en,
        EmbFuncFifoCfg
    );

    fn set_fsm_odr<I2C>(&self, i2c: &mut I2C, odr: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        assert!(odr < 4, "FSM ODR must fit in two bits");
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut raw = [0u8];
            bus.write_read(
                sensor.address,
                &[EmbeddedRegister::EmbFuncOdrCfgB.addr()],
                &mut raw,
            )?;
            let mut reg = EmbFuncOdrCfgB::from_bytes(raw);
            reg.set_odr(odr);
            bus.write(
                sensor.address,
                &[EmbeddedRegister::EmbFuncOdrCfgB.addr(), reg.into_bytes()[0]],
            )
        })
    }

    fn set_mlc_odr<I2C>(&self, i2c: &mut I2C, odr: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        assert!(odr < 4, "MLC ODR must fit in two bits");
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut raw = [0u8];
            bus.write_read(
                sensor.address,
                &[EmbeddedRegister::EmbFuncOdrCfgC.addr()],
                &mut raw,
            )?;
            let mut reg = EmbFuncOdrCfgC::from_bytes(raw);
            reg.set_odr(odr);
            bus.write(
                sensor.address,
                &[EmbeddedRegister::EmbFuncOdrCfgC.addr(), reg.into_bytes()[0]],
            )
        })
    }

    fn read_embedded_status<I2C>(&self, i2c: &mut I2C) -> Result<EmbFuncStatus, I2C::Error>
    where
        I2C: I2c,
    {
        let mut raw = [0u8];
        self.read_embedded_register(i2c, EmbeddedRegister::EmbFuncStatus, &mut raw)?;
        Ok(EmbFuncStatus::from_bytes(raw))
    }

    fn read_embedded_source<I2C>(&self, i2c: &mut I2C) -> Result<EmbFuncSrc, I2C::Error>
    where
        I2C: I2c,
    {
        let mut raw = [0u8];
        self.read_embedded_register(i2c, EmbeddedRegister::EmbFuncSrc, &mut raw)?;
        Ok(EmbFuncSrc::from_bytes(raw))
    }

    fn read_step_counter<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c,
    {
        let mut raw = [0u8; 2];
        self.read_embedded_register(i2c, EmbeddedRegister::StepCounterL, &mut raw)?;
        Ok(u16::from_le_bytes(raw))
    }

    fn read_fsm_outputs<I2C>(&self, i2c: &mut I2C) -> Result<[u8; 16], I2C::Error>
    where
        I2C: I2c,
    {
        let mut raw = [0u8; 16];
        self.read_embedded_register(i2c, EmbeddedRegister::FsmOuts1, &mut raw)?;
        Ok(raw)
    }

    fn read_mlc_sources<I2C>(&self, i2c: &mut I2C) -> Result<[u8; 8], I2C::Error>
    where
        I2C: I2c,
    {
        let mut raw = [0u8; 8];
        self.read_embedded_register(i2c, EmbeddedRegister::Mlc0Src, &mut raw)?;
        Ok(raw)
    }

    fn set_fsm_enabled<I2C>(
        &self,
        i2c: &mut I2C,
        program: u8,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        assert!(
            (1..=16).contains(&program),
            "FSM program must be 1 through 16"
        );
        let (register, mask) = if program <= 8 {
            (EmbeddedRegister::FsmEnableA, 1 << (program - 1))
        } else {
            (EmbeddedRegister::FsmEnableB, 1 << (program - 9))
        };
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut raw = [0u8];
            bus.write_read(sensor.address, &[register.addr()], &mut raw)?;
            raw[0] = if enable {
                raw[0] | mask
            } else {
                raw[0] & !mask
            };
            bus.write(sensor.address, &[register.addr(), raw[0]])
        })
    }

    fn clear_fsm_long_counter<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.write_embedded_register(i2c, EmbeddedRegister::FsmLongCounterClear, &[0x01])
    }

    embedded_enable_setter!(
        set_embedded_int1_step_detector,
        EmbFuncInt,
        set_step_detector,
        EmbFuncInt1
    );
    embedded_enable_setter!(set_embedded_int1_tilt, EmbFuncInt, set_tilt, EmbFuncInt1);
    embedded_enable_setter!(
        set_embedded_int1_significant_motion,
        EmbFuncInt,
        set_significant_motion,
        EmbFuncInt1
    );
    embedded_enable_setter!(
        set_embedded_int1_fsm_long_counter,
        EmbFuncInt,
        set_fsm_long_counter,
        EmbFuncInt1
    );
    embedded_enable_setter!(
        set_embedded_int2_step_detector,
        EmbFuncInt,
        set_step_detector,
        EmbFuncInt2
    );
    embedded_enable_setter!(set_embedded_int2_tilt, EmbFuncInt, set_tilt, EmbFuncInt2);
    embedded_enable_setter!(
        set_embedded_int2_significant_motion,
        EmbFuncInt,
        set_significant_motion,
        EmbFuncInt2
    );
    embedded_enable_setter!(
        set_embedded_int2_fsm_long_counter,
        EmbFuncInt,
        set_fsm_long_counter,
        EmbFuncInt2
    );
    embedded_initializer!(
        initialize_step_detector,
        EmbFuncInitA,
        set_step_detector,
        EmbFuncInitA
    );
    embedded_initializer!(initialize_tilt, EmbFuncInitA, set_tilt, EmbFuncInitA);
    embedded_initializer!(
        initialize_significant_motion,
        EmbFuncInitA,
        set_significant_motion,
        EmbFuncInitA
    );
    embedded_initializer!(initialize_fsm, EmbFuncInitB, set_fsm, EmbFuncInitB);
    embedded_initializer!(initialize_mlc, EmbFuncInitB, set_mlc, EmbFuncInitB);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn embedded_enable_preserves_access_mode() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr()],
                vec![0x40],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0xC0],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::EmbFuncEnA.addr()],
                vec![0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::EmbFuncEnA.addr(), 0x08],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x40],
            ),
        ]);

        sensor.set_pedometer(&mut i2c, true).unwrap();
        i2c.done();
    }

    #[test]
    fn reads_step_counter_from_embedded_bank() {
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
                vec![Register::FuncCfgAccess.addr(), 0x80],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::StepCounterL.addr()],
                vec![0x34, 0x12],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x00],
            ),
        ]);

        assert_eq!(sensor.read_step_counter(&mut i2c).unwrap(), 0x1234);
        i2c.done();
    }
}
