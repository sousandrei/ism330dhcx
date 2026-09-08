use embedded_hal::spi::SpiDevice;

use crate::RegisterBus;

const MAX_TRANSFER_SIZE: usize = 32;

/// SPI transport for an already-configured four-wire SPI device.
pub struct SpiDeviceBus<SPI> {
    device: SPI,
}

impl<SPI> SpiDeviceBus<SPI> {
    /// Wrap an SPI device whose mode, frequency, and chip-select handling are
    /// configured by the caller and its HAL.
    pub const fn new(device: SPI) -> Self {
        Self { device }
    }

    /// Return the underlying SPI device.
    pub fn into_inner(self) -> SPI {
        self.device
    }
}

impl<SPI> RegisterBus for SpiDeviceBus<SPI>
where
    SPI: SpiDevice<u8>,
{
    type Error = SPI::Error;

    fn read_register(
        &mut self,
        _address: u8,
        register: u8,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        assert!(
            data.len() <= MAX_TRANSFER_SIZE,
            "register read exceeds transport buffer"
        );
        let mut buffer = [0u8; MAX_TRANSFER_SIZE + 1];
        buffer[0] = register | 0x80;

        self.device
            .transfer_in_place(&mut buffer[..data.len() + 1])?;
        data.copy_from_slice(&buffer[1..data.len() + 1]);
        Ok(())
    }

    fn write_register(
        &mut self,
        _address: u8,
        register: u8,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        assert!(
            data.len() <= MAX_TRANSFER_SIZE,
            "register write exceeds transport buffer"
        );
        let mut buffer = [0u8; MAX_TRANSFER_SIZE + 1];
        buffer[0] = register & 0x7f;
        buffer[1..data.len() + 1].copy_from_slice(data);
        self.device.write(&buffer[..data.len() + 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::eh1::spi::{Mock, Transaction};
    use std::vec;

    #[test]
    fn writes_register_with_write_command_bit_cleared() {
        let mut spi = SpiDeviceBus::new(Mock::new(&[
            Transaction::transaction_start(),
            Transaction::write_vec(vec![0x60, 0xaa, 0x55]),
            Transaction::transaction_end(),
        ]));

        spi.write_register(0, 0xe0, &[0xaa, 0x55]).unwrap();
        spi.device.done();
    }

    #[test]
    fn reads_register_with_read_command_bit_set() {
        let mut spi = SpiDeviceBus::new(Mock::new(&[
            Transaction::transaction_start(),
            Transaction::transfer_in_place(vec![0x80, 0, 0], vec![0, 0x34, 0x12]),
            Transaction::transaction_end(),
        ]));
        let mut data = [0u8; 2];

        spi.read_register(0, 0, &mut data).unwrap();

        assert_eq!(data, [0x34, 0x12]);
        spi.device.done();
    }

    #[test]
    fn supports_embedded_bank_sized_transfers() {
        let mut spi = SpiDeviceBus::new(Mock::new(&[
            Transaction::transaction_start(),
            Transaction::write_vec(vec![0x20, 1, 2, 3, 4, 5, 6, 7, 8]),
            Transaction::transaction_end(),
        ]));

        spi.write_register(0, 0xa0, &[1, 2, 3, 4, 5, 6, 7, 8])
            .unwrap();
        spi.device.done();
    }

    #[test]
    fn initializes_sensor_and_applies_defaults() {
        let mut spi = SpiDeviceBus::new(Mock::new(&[
            Transaction::transaction_start(),
            Transaction::transfer_in_place(vec![0x8f, 0], vec![0, 0x6b]),
            Transaction::transaction_end(),
            Transaction::transaction_start(),
            Transaction::transfer_in_place(vec![0x92, 0], vec![0, 0]),
            Transaction::transaction_end(),
            Transaction::transaction_start(),
            Transaction::write_vec(vec![0x12, 0x40]),
            Transaction::transaction_end(),
            Transaction::transaction_start(),
            Transaction::transfer_in_place(vec![0x92, 0], vec![0, 0x40]),
            Transaction::transaction_end(),
            Transaction::transaction_start(),
            Transaction::write_vec(vec![0x12, 0x44]),
            Transaction::transaction_end(),
        ]));

        let sensor = crate::Ism330Dhcx::new_spi(&mut spi).unwrap();

        assert_eq!(sensor.address, crate::DEFAULT_I2C_ADDRESS);
        spi.device.done();
    }
}
