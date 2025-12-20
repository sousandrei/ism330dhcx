use crate::Ism330Dhcx;
use crate::registers::Register;
use bitfield::bitfield;
use embedded_hal::i2c::I2c;

bitfield! {
    /// Counter batch data rate register 1 (0Bh)
    pub struct CounterBdrReg1(u8);
    impl Debug;
    /// In conjunction with CNT_BDR_TH_[7:0] in COUNTER_BDR_REG2 (0Ch), sets the threshold for the internal counter of batch events.
    pub cnt_bdr_th_msb, set_cnt_bdr_th_msb: 2, 0;
    /// Selects the trigger for the internal counter of batch events between XL and gyro.
    pub trig_counter_bdr, set_trig_counter_bdr: 5;
    /// Resets the internal counter of batch events for a single sensor.
    pub rst_counter_bdr, set_rst_counter_bdr: 6;
    /// Enables pulsed data-ready mode.
    pub dataready_pulsed, set_dataready_pulsed: 7;
}

impl CounterBdrReg1 {
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

impl Default for CounterBdrReg1 {
    fn default() -> Self {
        Self::new()
    }
}

bitfield! {
    /// Counter batch data rate register 2 (0Ch)
    pub struct CounterBdrReg2(u8);
    impl Debug;
    /// In conjunction with CNT_BDR_TH_[10:8] in COUNTER_BDR_REG1 (0Bh), sets the threshold for the internal counter of batch events.
    pub cnt_bdr_th_lsb, set_cnt_bdr_th_lsb: 7, 0;
}

impl CounterBdrReg2 {
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

impl Default for CounterBdrReg2 {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration methods for COUNTER_BDR_REG1 register.
pub trait CounterBdrReg1Config {
    /// In conjunction with CNT_BDR_TH_[7:0] in COUNTER_BDR_REG2 (0Ch), sets the threshold for the internal counter of batch events.
    fn set_cnt_bdr_th_msb<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Selects the trigger for the internal counter of batch events between XL and gyro.
    fn set_trig_counter_bdr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Resets the internal counter of batch events for a single sensor.
    fn set_rst_counter_bdr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    /// Enables pulsed data-ready mode.
    fn set_dataready_pulsed<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl CounterBdrReg1Config for Ism330Dhcx {
    fn set_cnt_bdr_th_msb<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |v| {
            let mut reg = CounterBdrReg1::from_bytes([v]);
            reg.set_cnt_bdr_th_msb(val);
            reg.into_bytes()[0]
        })
    }
    fn set_trig_counter_bdr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |v| {
            let mut reg = CounterBdrReg1::from_bytes([v]);
            reg.set_trig_counter_bdr(val);
            reg.into_bytes()[0]
        })
    }
    fn set_rst_counter_bdr<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |v| {
            let mut reg = CounterBdrReg1::from_bytes([v]);
            reg.set_rst_counter_bdr(val);
            reg.into_bytes()[0]
        })
    }
    fn set_dataready_pulsed<I2C>(&self, i2c: &mut I2C, val: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |v| {
            let mut reg = CounterBdrReg1::from_bytes([v]);
            reg.set_dataready_pulsed(val);
            reg.into_bytes()[0]
        })
    }
}

/// Configuration methods for COUNTER_BDR_REG2 register.
pub trait CounterBdrReg2Config {
    /// In conjunction with CNT_BDR_TH_[10:8] in COUNTER_BDR_REG1 (0Bh), sets the threshold for the internal counter of batch events.
    fn set_cnt_bdr_th_lsb<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

impl CounterBdrReg2Config for Ism330Dhcx {
    fn set_cnt_bdr_th_lsb<I2C>(&self, i2c: &mut I2C, val: u8) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CounterBdrReg2, |v| {
            let mut reg = CounterBdrReg2::from_bytes([v]);
            reg.set_cnt_bdr_th_lsb(val);
            reg.into_bytes()[0]
        })
    }
}
