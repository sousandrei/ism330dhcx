#![allow(unused_parens)]
use crate::Ism330Dhcx;
use crate::registers::Register;
use embedded_hal::i2c::I2c;
use modular_bitfield::{bitfield, specifiers::B3};

/// Counter batch data rate register 1 (0Bh)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct CounterBdrReg1 {
    /// In conjunction with CNT_BDR_TH_[7:0] in COUNTER_BDR_REG2 (0Ch), sets the threshold for the
    /// internal counter of batch events.
    pub cnt_bdr_th_msb: B3,
    #[skip]
    pub __: modular_bitfield::specifiers::B2,
    /// Selects the trigger for the internal counter of batch events between XL and gyro.
    /// 0: XL batch event; 1: GYRO batch event
    pub trig_counter_bdr: bool,
    /// Resets the internal counter of batch events for a single sensor.
    pub rst_counter_bdr: bool,
    /// Enables pulsed data-ready mode.
    /// 0: Data-ready latched mode; 1: Data-ready pulsed mode
    pub dataready_pulsed: bool,
}

impl Default for CounterBdrReg1 {
    fn default() -> Self {
        Self::new()
    }
}

/// Counter batch data rate register 2 (0Ch)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct CounterBdrReg2 {
    /// In conjunction with CNT_BDR_TH_[10:8] in COUNTER_BDR_REG1 (0Bh), sets the threshold for the
    /// internal counter of batch events.
    pub cnt_bdr_th_lsb: u8,
}

impl Default for CounterBdrReg2 {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CounterBdrExt {
    /// Enables pulsed data-ready mode.
    fn set_dataready_pulsed<I2C>(&self, i2c: &mut I2C, pulsed: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Resets the internal counter of batch events for a single sensor.
    fn reset_counter_bdr<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Selects the trigger for the internal counter of batch events between XL and gyro.
    /// false: XL batch event; true: GYRO batch event
    fn set_trig_counter_bdr<I2C>(&self, i2c: &mut I2C, gyro: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;

    /// Sets the threshold for the internal counter of batch events (11 bits).
    fn set_cnt_bdr_threshold<I2C>(&self, i2c: &mut I2C, threshold: u11) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

#[allow(non_camel_case_types)]
type u11 = u16;

impl CounterBdrExt for Ism330Dhcx {
    fn set_dataready_pulsed<I2C>(&self, i2c: &mut I2C, pulsed: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |v| {
            let mut reg = CounterBdrReg1::from_bytes([v]);
            reg.set_dataready_pulsed(pulsed);
            reg.into_bytes()[0]
        })
    }

    fn reset_counter_bdr<I2C>(&self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |v| {
            let mut reg = CounterBdrReg1::from_bytes([v]);
            reg.set_rst_counter_bdr(true);
            reg.into_bytes()[0]
        })
    }

    fn set_trig_counter_bdr<I2C>(&self, i2c: &mut I2C, gyro: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        self.modify_reg(i2c, Register::CounterBdrReg1, |v| {
            let mut reg = CounterBdrReg1::from_bytes([v]);
            reg.set_trig_counter_bdr(gyro);
            reg.into_bytes()[0]
        })
    }

    fn set_cnt_bdr_threshold<I2C>(&self, i2c: &mut I2C, threshold: u11) -> Result<(), I2C::Error>
    where
        I2C: I2c,
    {
        let threshold = threshold & 0x07FF; // 11 bits
        let msb = (threshold >> 8) as u8;
        let lsb = (threshold & 0xFF) as u8;

        self.modify_reg(i2c, Register::CounterBdrReg1, |v| {
            let mut reg = CounterBdrReg1::from_bytes([v]);
            reg.set_cnt_bdr_th_msb(msb);
            reg.into_bytes()[0]
        })?;

        self.write_reg(i2c, Register::CounterBdrReg2, lsb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use crate::Ism330Dhcx;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn test_counter_bdr() {
        let mut reg1 = CounterBdrReg1::new();
        reg1.set_cnt_bdr_th_msb(0x07);
        reg1.set_trig_counter_bdr(true);
        reg1.set_rst_counter_bdr(true);
        reg1.set_dataready_pulsed(true);
        // bits: 1 1 1 0 0 1 1 1 = 0xE7
        assert_eq!(reg1.into_bytes()[0], 0xE7);

        let mut reg2 = CounterBdrReg2::new();
        reg2.set_cnt_bdr_th_lsb(0xAA);
        assert_eq!(reg2.into_bytes()[0], 0xAA);
    }

    #[test]
    fn test_set_cnt_bdr_threshold() {
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::WhoAmI.addr()],
                vec![0x6b],
            ),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr()],
                vec![0x00],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl3C.addr(), 0x40]),
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr()],
                vec![0x40],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl3C.addr(), 0x44]),
            // Threshold = 0x5AA = 101 10101010 => MSB=101(5), LSB=10101010(AA)
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::CounterBdrReg1.addr()],
                vec![0x00],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::CounterBdrReg1.addr(), 0x05],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::CounterBdrReg2.addr(), 0xAA],
            ),
        ]);

        let sensor = Ism330Dhcx::new(&mut i2c).unwrap();
        sensor.set_cnt_bdr_threshold(&mut i2c, 0x5AA).unwrap();
        i2c.done();
    }
}
