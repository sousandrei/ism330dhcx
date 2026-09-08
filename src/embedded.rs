use crate::RegisterBus;

use crate::Ism330Dhcx;
use crate::registers::{
    EmbFuncEnA, EmbFuncEnB, EmbFuncFifoCfg, EmbFuncInitA, EmbFuncInitB, EmbFuncInt, EmbFuncOdrCfgB,
    EmbFuncOdrCfgC, EmbFuncSrc, EmbFuncStatus, EmbeddedRegister, FsmBits, MlcStatus, PageRw,
    PageSel, Register,
};

/// FSM program selector.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FsmProgram {
    Program1,
    Program2,
    Program3,
    Program4,
    Program5,
    Program6,
    Program7,
    Program8,
    Program9,
    Program10,
    Program11,
    Program12,
    Program13,
    Program14,
    Program15,
    Program16,
}

impl FsmProgram {
    const fn index(self) -> u8 {
        match self {
            Self::Program1 => 0,
            Self::Program2 => 1,
            Self::Program3 => 2,
            Self::Program4 => 3,
            Self::Program5 => 4,
            Self::Program6 => 5,
            Self::Program7 => 6,
            Self::Program8 => 7,
            Self::Program9 => 8,
            Self::Program10 => 9,
            Self::Program11 => 10,
            Self::Program12 => 11,
            Self::Program13 => 12,
            Self::Program14 => 13,
            Self::Program15 => 14,
            Self::Program16 => 15,
        }
    }
}

/// MLC output selector.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MlcOutput {
    Output1,
    Output2,
    Output3,
    Output4,
    Output5,
    Output6,
    Output7,
    Output8,
}

impl MlcOutput {
    const fn index(self) -> u8 {
        match self {
            Self::Output1 => 0,
            Self::Output2 => 1,
            Self::Output3 => 2,
            Self::Output4 => 3,
            Self::Output5 => 4,
            Self::Output6 => 5,
            Self::Output7 => 6,
            Self::Output8 => 7,
        }
    }
}

/// Embedded-function page and register access.
pub trait EmbeddedFunctions {
    /// Select an embedded-function page (0 through 15).
    fn set_embedded_page<I2C>(&self, i2c: &mut I2C, page: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Read an embedded-function register while preserving the access mode.
    fn read_embedded_register<I2C>(
        &self,
        i2c: &mut I2C,
        register: EmbeddedRegister,
        buffer: &mut [u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Write an embedded-function register while preserving the access mode.
    fn write_embedded_register<I2C>(
        &self,
        i2c: &mut I2C,
        register: EmbeddedRegister,
        value: &[u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Read a bounded register range from an advanced embedded page.
    fn read_embedded_page<I2C>(
        &self,
        i2c: &mut I2C,
        page: u8,
        address: u8,
        buffer: &mut [u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Write a bounded register range to an advanced embedded page.
    fn write_embedded_page<I2C>(
        &self,
        i2c: &mut I2C,
        page: u8,
        address: u8,
        value: &[u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_pedometer<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_tilt<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_significant_motion<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_fsm<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_mlc<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_fifo_compression<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_pedometer_fifo<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_fsm_odr<I2C>(&self, i2c: &mut I2C, odr: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_mlc_odr<I2C>(&self, i2c: &mut I2C, odr: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn read_embedded_status<I2C>(&self, i2c: &mut I2C) -> Result<EmbFuncStatus, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read FSM status for programs 1 through 16 from the embedded bank.
    fn read_fsm_status<I2C>(&self, i2c: &mut I2C) -> Result<[FsmBits; 2], I2C::Error>
    where
        I2C: RegisterBus;
    /// Read MLC status for outputs 1 through 8 from the embedded bank.
    fn read_mlc_status<I2C>(&self, i2c: &mut I2C) -> Result<MlcStatus, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read embedded-function status from the main register page.
    fn read_embedded_status_mainpage<I2C>(
        &self,
        i2c: &mut I2C,
    ) -> Result<EmbFuncStatus, I2C::Error>
    where
        I2C: RegisterBus;
    /// Read FSM status from the main register page.
    fn read_fsm_status_mainpage<I2C>(&self, i2c: &mut I2C) -> Result<[FsmBits; 2], I2C::Error>
    where
        I2C: RegisterBus;
    /// Read MLC status from the main register page.
    fn read_mlc_status_mainpage<I2C>(&self, i2c: &mut I2C) -> Result<MlcStatus, I2C::Error>
    where
        I2C: RegisterBus;
    fn read_embedded_source<I2C>(&self, i2c: &mut I2C) -> Result<EmbFuncSrc, I2C::Error>
    where
        I2C: RegisterBus;
    fn read_step_counter<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: RegisterBus;
    fn read_fsm_outputs<I2C>(&self, i2c: &mut I2C) -> Result<[u8; 16], I2C::Error>
    where
        I2C: RegisterBus;
    fn read_mlc_sources<I2C>(&self, i2c: &mut I2C) -> Result<[u8; 8], I2C::Error>
    where
        I2C: RegisterBus;
    fn set_fsm_enabled<I2C>(
        &self,
        i2c: &mut I2C,
        program: u8,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Route one FSM program to INT1.
    fn set_fsm_int1<I2C>(
        &self,
        i2c: &mut I2C,
        program: FsmProgram,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Route one FSM program to INT2.
    fn set_fsm_int2<I2C>(
        &self,
        i2c: &mut I2C,
        program: FsmProgram,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Route one MLC output to INT1.
    fn set_mlc_int1<I2C>(
        &self,
        i2c: &mut I2C,
        output: MlcOutput,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Route one MLC output to INT2.
    fn set_mlc_int2<I2C>(
        &self,
        i2c: &mut I2C,
        output: MlcOutput,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn clear_fsm_long_counter<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    /// Read the 16-bit FSM long counter.
    fn read_fsm_long_counter<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_int1_step_detector<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_int1_tilt<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_int1_significant_motion<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_int1_fsm_long_counter<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_int2_step_detector<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_int2_tilt<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_int2_significant_motion<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn set_embedded_int2_fsm_long_counter<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn initialize_step_detector<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn initialize_tilt<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn initialize_significant_motion<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn initialize_fsm<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
    fn initialize_mlc<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus;
}

impl Ism330Dhcx {
    fn with_embedded_access<I2C, T, F>(&self, i2c: &mut I2C, operation: F) -> Result<T, I2C::Error>
    where
        I2C: RegisterBus,
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

    fn set_embedded_routing_bit<I2C>(
        &self,
        i2c: &mut I2C,
        register: EmbeddedRegister,
        bit: u8,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut raw = [0u8];
            bus.read_register(sensor.address, register.addr(), &mut raw)?;
            raw[0] = if enable {
                raw[0] | (1 << bit)
            } else {
                raw[0] & !(1 << bit)
            };
            bus.write_register(sensor.address, register.addr(), &raw)
        })
    }

    fn access_embedded_page<I2C, T, F>(
        &self,
        i2c: &mut I2C,
        page: u8,
        address: u8,
        write: bool,
        operation: F,
    ) -> Result<T, I2C::Error>
    where
        I2C: RegisterBus,
        F: FnOnce(&Self, &mut I2C) -> Result<T, I2C::Error>,
    {
        assert!(page < 16, "embedded page must fit in four bits");
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut rw = PageRw::new();
            if write {
                rw.set_page_write(true);
            } else {
                rw.set_page_read(true);
            }
            bus.write_register(
                sensor.address,
                EmbeddedRegister::PageRw.addr(),
                &rw.into_bytes(),
            )?;

            let mut page_sel = PageSel::default();
            page_sel.set_page_sel(page);
            bus.write_register(
                sensor.address,
                EmbeddedRegister::PageSel.addr(),
                &page_sel.into_bytes(),
            )?;
            bus.write_register(
                sensor.address,
                EmbeddedRegister::PageAddress.addr(),
                &[address],
            )?;

            let result = operation(sensor, bus);
            let disable = bus.write_register(sensor.address, EmbeddedRegister::PageRw.addr(), &[0]);
            match result {
                Err(error) => Err(error),
                Ok(value) => disable.map(|()| value),
            }
        })
    }
}

macro_rules! embedded_enable_setter {
    ($method:ident, $config:ident, $setter:ident, $register:ident) => {
        fn $method<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
        where
            I2C: RegisterBus,
        {
            self.with_embedded_access(i2c, |sensor, bus| {
                let mut raw = [0u8];
                bus.read_register(sensor.address, EmbeddedRegister::$register.addr(), &mut raw)?;
                let mut reg = $config::from_bytes(raw);
                reg.$setter(enable);
                bus.write_register(
                    sensor.address,
                    EmbeddedRegister::$register.addr(),
                    &[reg.into_bytes()[0]],
                )
            })
        }
    };
}

macro_rules! embedded_initializer {
    ($method:ident, $config:ident, $setter:ident, $register:ident) => {
        fn $method<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
        where
            I2C: RegisterBus,
        {
            self.with_embedded_access(i2c, |sensor, bus| {
                let mut raw = [0u8];
                bus.read_register(sensor.address, EmbeddedRegister::$register.addr(), &mut raw)?;
                let mut reg = $config::from_bytes(raw);
                reg.$setter(true);
                bus.write_register(
                    sensor.address,
                    EmbeddedRegister::$register.addr(),
                    &[reg.into_bytes()[0]],
                )
            })
        }
    };
}

impl EmbeddedFunctions for Ism330Dhcx {
    fn set_embedded_page<I2C>(&self, i2c: &mut I2C, page: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        assert!(page < 16, "embedded page must fit in four bits");
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut reg = PageSel::default();
            reg.set_page_sel(page);
            bus.write_register(
                sensor.address,
                EmbeddedRegister::PageSel.addr(),
                &[reg.into_bytes()[0]],
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
        I2C: RegisterBus,
    {
        self.with_embedded_access(i2c, |sensor, bus| {
            bus.read_register(sensor.address, register.addr(), buffer)
        })
    }

    fn write_embedded_register<I2C>(
        &self,
        i2c: &mut I2C,
        register: EmbeddedRegister,
        value: &[u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        assert!(!value.is_empty(), "embedded register writes need a value");
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut bytes = [0u8; 33];
            assert!(value.len() < bytes.len(), "embedded write is too long");
            bytes[0] = register.addr();
            bytes[1..value.len() + 1].copy_from_slice(value);
            bus.write_register(sensor.address, bytes[0], &bytes[1..value.len() + 1])
        })
    }

    fn read_embedded_page<I2C>(
        &self,
        i2c: &mut I2C,
        page: u8,
        address: u8,
        buffer: &mut [u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        assert!(!buffer.is_empty(), "embedded page reads need a buffer");
        assert!(buffer.len() <= 32, "embedded page read is too long");
        assert!(
            usize::from(address) + buffer.len() <= 256,
            "embedded page read wraps"
        );
        self.access_embedded_page(i2c, page, address, false, |sensor, bus| {
            bus.read_register(sensor.address, EmbeddedRegister::PageValue.addr(), buffer)
        })
    }

    fn write_embedded_page<I2C>(
        &self,
        i2c: &mut I2C,
        page: u8,
        address: u8,
        value: &[u8],
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        assert!(!value.is_empty(), "embedded page writes need a value");
        assert!(value.len() <= 32, "embedded page write is too long");
        assert!(
            usize::from(address) + value.len() <= 256,
            "embedded page write wraps"
        );
        self.access_embedded_page(i2c, page, address, true, |sensor, bus| {
            bus.write_register(sensor.address, EmbeddedRegister::PageValue.addr(), value)
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
        I2C: RegisterBus,
    {
        assert!(odr < 4, "FSM ODR must fit in two bits");
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut raw = [0u8];
            bus.read_register(
                sensor.address,
                EmbeddedRegister::EmbFuncOdrCfgB.addr(),
                &mut raw,
            )?;
            let mut reg = EmbFuncOdrCfgB::from_bytes(raw);
            reg.set_odr(odr);
            bus.write_register(
                sensor.address,
                EmbeddedRegister::EmbFuncOdrCfgB.addr(),
                &[reg.into_bytes()[0]],
            )
        })
    }

    fn set_mlc_odr<I2C>(&self, i2c: &mut I2C, odr: u8) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        assert!(odr < 4, "MLC ODR must fit in two bits");
        self.with_embedded_access(i2c, |sensor, bus| {
            let mut raw = [0u8];
            bus.read_register(
                sensor.address,
                EmbeddedRegister::EmbFuncOdrCfgC.addr(),
                &mut raw,
            )?;
            let mut reg = EmbFuncOdrCfgC::from_bytes(raw);
            reg.set_odr(odr);
            bus.write_register(
                sensor.address,
                EmbeddedRegister::EmbFuncOdrCfgC.addr(),
                &[reg.into_bytes()[0]],
            )
        })
    }

    fn read_embedded_status<I2C>(&self, i2c: &mut I2C) -> Result<EmbFuncStatus, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8];
        self.read_embedded_register(i2c, EmbeddedRegister::EmbFuncStatus, &mut raw)?;
        Ok(EmbFuncStatus::from_bytes(raw))
    }

    fn read_fsm_status<I2C>(&self, i2c: &mut I2C) -> Result<[FsmBits; 2], I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8; 2];
        self.read_embedded_register(i2c, EmbeddedRegister::FsmStatusA, &mut raw)?;
        Ok([FsmBits::from_bytes([raw[0]]), FsmBits::from_bytes([raw[1]])])
    }

    fn read_mlc_status<I2C>(&self, i2c: &mut I2C) -> Result<MlcStatus, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8];
        self.read_embedded_register(i2c, EmbeddedRegister::MlcStatus, &mut raw)?;
        Ok(MlcStatus::from_bytes(raw))
    }

    fn read_embedded_status_mainpage<I2C>(&self, i2c: &mut I2C) -> Result<EmbFuncStatus, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(EmbFuncStatus::from_bytes([
            self.read_reg(i2c, Register::EmbFuncStatusMainpage)?
        ]))
    }

    fn read_fsm_status_mainpage<I2C>(&self, i2c: &mut I2C) -> Result<[FsmBits; 2], I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8; 2];
        self.read_register(i2c, Register::FsmStatusAMainpage.addr(), &mut raw)?;
        Ok([FsmBits::from_bytes([raw[0]]), FsmBits::from_bytes([raw[1]])])
    }

    fn read_mlc_status_mainpage<I2C>(&self, i2c: &mut I2C) -> Result<MlcStatus, I2C::Error>
    where
        I2C: RegisterBus,
    {
        Ok(MlcStatus::from_bytes([
            self.read_reg(i2c, Register::MlcStatusMainpage)?
        ]))
    }

    fn read_embedded_source<I2C>(&self, i2c: &mut I2C) -> Result<EmbFuncSrc, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8];
        self.read_embedded_register(i2c, EmbeddedRegister::EmbFuncSrc, &mut raw)?;
        Ok(EmbFuncSrc::from_bytes(raw))
    }

    fn read_step_counter<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8; 2];
        self.read_embedded_register(i2c, EmbeddedRegister::StepCounterL, &mut raw)?;
        Ok(u16::from_le_bytes(raw))
    }

    fn read_fsm_outputs<I2C>(&self, i2c: &mut I2C) -> Result<[u8; 16], I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8; 16];
        self.read_embedded_register(i2c, EmbeddedRegister::FsmOuts1, &mut raw)?;
        Ok(raw)
    }

    fn read_mlc_sources<I2C>(&self, i2c: &mut I2C) -> Result<[u8; 8], I2C::Error>
    where
        I2C: RegisterBus,
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
        I2C: RegisterBus,
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
            bus.read_register(sensor.address, register.addr(), &mut raw)?;
            raw[0] = if enable {
                raw[0] | mask
            } else {
                raw[0] & !mask
            };
            bus.write_register(sensor.address, register.addr(), &[raw[0]])
        })
    }

    fn set_fsm_int1<I2C>(
        &self,
        i2c: &mut I2C,
        program: FsmProgram,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        let index = program.index();
        self.set_embedded_routing_bit(
            i2c,
            if index < 8 {
                EmbeddedRegister::FsmInt1A
            } else {
                EmbeddedRegister::FsmInt1B
            },
            index % 8,
            enable,
        )
    }

    fn set_fsm_int2<I2C>(
        &self,
        i2c: &mut I2C,
        program: FsmProgram,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        let index = program.index();
        self.set_embedded_routing_bit(
            i2c,
            if index < 8 {
                EmbeddedRegister::FsmInt2A
            } else {
                EmbeddedRegister::FsmInt2B
            },
            index % 8,
            enable,
        )
    }

    fn set_mlc_int1<I2C>(
        &self,
        i2c: &mut I2C,
        output: MlcOutput,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.set_embedded_routing_bit(i2c, EmbeddedRegister::MlcInt1, output.index(), enable)
    }

    fn set_mlc_int2<I2C>(
        &self,
        i2c: &mut I2C,
        output: MlcOutput,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.set_embedded_routing_bit(i2c, EmbeddedRegister::MlcInt2, output.index(), enable)
    }

    fn clear_fsm_long_counter<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: RegisterBus,
    {
        self.write_embedded_register(i2c, EmbeddedRegister::FsmLongCounterClear, &[0x01])
    }

    fn read_fsm_long_counter<I2C>(&self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: RegisterBus,
    {
        let mut raw = [0u8; 2];
        self.read_embedded_register(i2c, EmbeddedRegister::FsmLongCounterL, &mut raw)?;
        Ok(u16::from_le_bytes(raw))
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
    use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTransaction};
    use std::vec;

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

    #[test]
    fn routes_fsm_and_mlc_outputs_to_int2() {
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
                vec![EmbeddedRegister::FsmInt2B.addr()],
                vec![0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::FsmInt2B.addr(), 0x01],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x00],
            ),
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
                vec![EmbeddedRegister::MlcInt2.addr()],
                vec![0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::MlcInt2.addr(), 0x80],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x00],
            ),
        ]);

        sensor
            .set_fsm_int2(&mut i2c, FsmProgram::Program9, true)
            .unwrap();
        sensor
            .set_mlc_int2(&mut i2c, MlcOutput::Output8, true)
            .unwrap();
        i2c.done();
    }

    #[test]
    fn reads_mainpage_status_and_fsm_long_counter() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::EmbFuncStatusMainpage.addr()],
                vec![0x88],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FsmStatusAMainpage.addr()],
                vec![0x01, 0x80],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::MlcStatusMainpage.addr()],
                vec![0x81],
            ),
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
                vec![EmbeddedRegister::FsmLongCounterL.addr()],
                vec![0x34, 0x12],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x00],
            ),
        ]);

        assert!(
            sensor
                .read_embedded_status_mainpage(&mut i2c)
                .unwrap()
                .fsm_long_counter()
        );
        assert_eq!(
            sensor.read_fsm_status_mainpage(&mut i2c).unwrap()[1].bits(),
            0x80
        );
        assert_eq!(
            sensor.read_mlc_status_mainpage(&mut i2c).unwrap().results(),
            0x81
        );
        assert_eq!(sensor.read_fsm_long_counter(&mut i2c).unwrap(), 0x1234);
        i2c.done();
    }

    #[test]
    fn reads_bounded_advanced_page_registers() {
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
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::PageRw.addr(), 0x20],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::PageSel.addr(), 0x11],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::PageAddress.addr(), 0x84],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::PageValue.addr()],
                vec![0x06, 0x07],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![EmbeddedRegister::PageRw.addr(), 0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::FuncCfgAccess.addr(), 0x00],
            ),
        ]);
        let mut value = [0u8; 2];

        sensor
            .read_embedded_page(&mut i2c, 1, 0x84, &mut value)
            .unwrap();

        assert_eq!(value, [0x06, 0x07]);
        i2c.done();
    }

    #[test]
    fn routes_fsm_output_over_spi() {
        let spi = SpiMock::new(&[
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(vec![0x81, 0], vec![0, 0]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x01, 0x80]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::transfer_in_place(vec![0x8b, 0], vec![0, 0]),
            SpiTransaction::transaction_end(),
            SpiTransaction::transaction_start(),
            SpiTransaction::write_vec(vec![0x0b, 0x01]),
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
            .set_fsm_int1(&mut bus, FsmProgram::Program1, true)
            .unwrap();
        bus.into_inner().done();
    }
}
