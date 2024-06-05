use embedded_hal::i2c::I2c;

use crate::Register;

/// The FIFO_STATUS registers.
pub struct FifoStatus {
    pub address: u8,
}

pub const ADDR: u8 = 0x3a_u8;

impl Register for FifoStatus {}

impl FifoStatus {
    pub fn new(address: u8) -> Self {
        FifoStatus { address }
    }

    /// Is the FIFO full
    pub fn full<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1)?;

        Ok(v & (1 << 5) != 0)
    }

    /// Is the FIFO overrun
    pub fn overrun<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1)?;

        Ok(v & (1 << 6) != 0)
    }

    /// Is the FIFO watermark reached.
    pub fn watermark_reached<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1)?;

        Ok(v & (1 << 7) != 0)
    }

    /// Latched FIFO overrun status.
    pub fn overrun_latched<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1)?;

        Ok(v & (1 << 3) != 0)
    }

    /// Counter BDR reached.
    pub fn count_bdr_reached<I2C>(&mut self, i2c: &mut I2C) -> Result<bool, I2C::Error>
    where
        I2C: embedded_hal::i2c::I2c,
    {
        let v = self.read(i2c, self.address, ADDR + 1)?;

        Ok(v & (1 << 4) != 0)
    }

    /// Number of unread sensor data in FIFO.
    pub fn diff_fifo<I2C>(&mut self, i2c: &mut I2C) -> Result<u16, I2C::Error>
    where
        I2C: I2c,
    {
        let mut v = [0u8; 2];
        i2c.write_read(self.address, &[ADDR], &mut v)?;
        v[1] &= 0b11;

        Ok(u16::from_le_bytes(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn test_full() {
        let mut i2c = Mock::new(&[Transaction::write_read(0x6b, vec![0x3b], vec![0b100000])]);

        let mut f = FifoStatus::new(crate::DEFAULT_I2C_ADDRESS);
        assert!(f.full(&mut i2c).unwrap());
    }

    #[test]
    fn test_diff_1b() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            0x6b,
            vec![0x3a],
            vec![0b00100000_u8, 0b00000000_u8],
        )]);

        let mut f = FifoStatus::new(crate::DEFAULT_I2C_ADDRESS);
        let diff = f.diff_fifo(&mut i2c).unwrap();

        assert_eq!(diff, 0b000100000);
    }

    #[test]
    fn test_diff_2b() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            0x6b,
            vec![0x3a],
            vec![0b00100000_u8, 0b00000001_u8],
        )]);

        let mut f = FifoStatus::new(crate::DEFAULT_I2C_ADDRESS);
        let diff = f.diff_fifo(&mut i2c).unwrap();

        assert_eq!(diff, 0b100100000);
    }
}
