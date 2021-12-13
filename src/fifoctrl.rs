use core::fmt;
use embedded_hal::blocking::i2c::Write;

use crate::Register;

/// The FIFO_CTRL1 to FIFO_CTRL4 registers
///
/// The four registers are handled as one because values and functionality is split across several
/// registers.
pub struct FifoCtrl {
    pub address: u8,
    value: [u8; 4],
}

impl fmt::Display for FifoCtrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.value.iter() {
            write!(f, "{}", r)?;
        }

        Ok(())
    }
}

impl fmt::Binary for FifoCtrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.value.iter() {
            write!(f, "{:b}", r)?;
        }

        Ok(())
    }
}

impl fmt::LowerHex for FifoCtrl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.value.iter() {
            fmt::LowerHex::fmt(&r, f)?;
        }

        Ok(())
    }
}

pub const ADDR: u8 = 0x07_u8;

impl Register for FifoCtrl {}

/// FIFO mode
///
/// Default value is `Bypass` (off).
#[repr(u8)]
pub enum FifoMode {
    Bypass = 0b000,
    FifoMode = 0b001,
    ContinuousToFifo = 0b011,
    BypassToContinuous = 0b100,
    Conitnous = 0b110,
    BypassToFifo = 0b111,
}

impl FifoCtrl {
    pub fn new(value: [u8; 4], address: u8) -> Self {
        FifoCtrl { address, value }
    }

    /// Enable compression of values in FIFO, increasing FIFO size from 3kB to maximum 9kB.
    pub fn compression<I2C>(&mut self, i2c: &mut I2C, value: bool) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        self.value[1] &= !(1 << 6);
        self.value[1] |= (value as u8) << 6;
        self.write(i2c, self.address, ADDR + 1, self.value[1])
    }

    /// Set the FIFO mode (or disable FIFO)
    pub fn mode<I2C>(&mut self, i2c: &mut I2C, mode: FifoMode) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        const RESET: u8 = 0b111;

        self.value[3] &= !RESET;
        self.value[3] |= mode as u8;
        self.write(i2c, self.address, ADDR + 3, self.value[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::i2c::{Mock, Transaction};

    #[test]
    fn test_compression() {
        let mut i2c = Mock::new(&[Transaction::write(0x6b, vec![0x08, 0b1000000])]);

        let mut f = FifoCtrl::new([0; 4], crate::DEFAULT_I2C_ADDRESS);

        f.compression(&mut i2c, true).unwrap();
        assert_eq!(f.value[1], 0b1000000);
    }

    #[test]
    fn test_sub_addresses() {
        let r2 = ADDR + 1;
        let r3 = ADDR + 2;
        let r4 = ADDR + 3;

        assert_eq!(ADDR, 0x07);
        assert_eq!(r2, 0x08);
        assert_eq!(r3, 0x09);
        assert_eq!(r4, 0x0a);
    }

    #[test]
    fn set_mode() {
        let mut i2c = Mock::new(&[
            Transaction::write(0x6b, vec![0x0a, 0b0000000]),
            Transaction::write(0x6b, vec![0x0a, 0b0000001]),
        ]);
        let mut f = FifoCtrl {
            address: 0x6b,
            value: [0; 4],
        };

        f.mode(&mut i2c, FifoMode::Bypass).unwrap();
        assert_eq!(f.value[3], 0b0000000);

        f.mode(&mut i2c, FifoMode::FifoMode).unwrap();
        assert_eq!(f.value[3], 0b0000001);
    }
}
