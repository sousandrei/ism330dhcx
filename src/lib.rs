//! This is a simple driver for ST's `ism330dhcx` sensor.
//!
//! # Quick Start
//! To declare a sensor is pretty simple:
//!
//! ```rust,ignore
//! let sensor = Ism330Dhcx::new(&mut i2c).unwrap();
//! ```
//!
//! The driver borrows the register transport for each operation.
//!
//! For four-wire SPI, wrap an already-configured [`embedded_hal::spi::SpiDevice`]:
//! ```rust,ignore
//! let mut spi = SpiDeviceBus::new(spi_device);
//! let sensor = Ism330Dhcx::new_spi(&mut spi).unwrap();
//! ```
//! SPI mode, clock frequency, and electrical setup are selected by the caller
//! according to the datasheet and their HAL. Three-wire SPI is not supported.
//!
//! To configure the sensor, use the high-level methods:
//!
//! ```rust,ignore
//! sensor.set_accel_odr(OdrXl::Hz52).unwrap();
//! sensor.set_boot(true).unwrap();
//! ```
//!
//! Register layouts, typed fields, reset values, and address namespaces are
//! defined in [`registers`]. Feature modules expose the public I2C API through
//! [`Accelerometer`], [`Gyroscope`], [`Configuration`], [`Fifo`], [`Motion`],
//! [`InterruptStatus`], [`EmbeddedFunctions`], [`SensorHub`], [`Offsets`], and
//! [`Ois`].
//!
//! # Reference
//!
//! - [Sensor page](https://www.st.com/en/mems-and-sensors/ism330dhcx.html)
//! - [Datasheet](https://www.st.com/resource/en/datasheet/ism330dhcx.pdf)

#![cfg_attr(not(test), no_std)]

pub mod accelerometer;
pub mod configuration;
pub mod embedded;
pub mod fifo;
pub mod gyroscope;
pub mod interrupt;
pub mod motion;
pub mod offsets;
pub mod ois;
pub mod registers;
pub mod sensor_hub;
pub mod spi;

pub use accelerometer::{AccelValue, Accelerometer, SENSORS_GRAVITY_STANDARD};
pub use configuration::{Configuration, CoreField};
pub use embedded::EmbeddedFunctions;
pub use fifo::{Fifo, FifoEntry, FifoTag, SensorTag, Value};
pub use gyroscope::{GyroValue, Gyroscope, SENSORS_DPS_TO_RADS};
pub use interrupt::{InterruptStatus, Interrupts};
pub use motion::Motion;
pub use offsets::Offsets;
pub use ois::Ois;
pub use sensor_hub::{SensorHub, SensorHubReadConfig, SensorHubSlave};
pub use spi::SpiDeviceBus;

use embedded_hal::i2c::I2c;
use embedded_hal::spi::SpiDevice;
use registers::*;

/// Datasheet write address for the device. (D6h)
pub const DEFAULT_I2C_ADDRESS: u8 = 0x6bu8;

/// Errors for the ISM330DHCX driver.
#[derive(Debug, Copy, Clone, defmt::Format)]
pub enum Error<E> {
    /// I2C bus error.
    I2c(E),
    /// SPI bus error.
    Spi(E),
    /// Invalid device found (WHO_AM_I mismatch).
    InvalidDevice(u8),
}

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Self::I2c(error)
    }
}

/// Internal register transport implemented by I2C and SPI buses.
pub trait RegisterBus {
    type Error;

    fn read_register(
        &mut self,
        address: u8,
        register: u8,
        data: &mut [u8],
    ) -> Result<(), Self::Error>;

    fn write_register(&mut self, address: u8, register: u8, data: &[u8])
    -> Result<(), Self::Error>;
}

impl<I2C> RegisterBus for I2C
where
    I2C: I2c,
{
    type Error = I2C::Error;

    fn read_register(
        &mut self,
        address: u8,
        register: u8,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write_read(address, &[register], data)
    }

    fn write_register(
        &mut self,
        address: u8,
        register: u8,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        assert!(data.len() <= 32, "register write exceeds transport buffer");
        let mut buffer = [0u8; 33];
        buffer[0] = register;
        buffer[1..data.len() + 1].copy_from_slice(data);
        self.write(address, &buffer[..data.len() + 1])
    }
}

/// Driver for the ISM330DHCX sensor.
pub struct Ism330Dhcx {
    /// I2C address.
    pub address: u8,
}

impl Ism330Dhcx {
    /// Create a new driver instance with the default I2C address (0x6B).
    pub fn new<I2C>(i2c: &mut I2C) -> Result<Self, Error<I2C::Error>>
    where
        I2C: I2c,
    {
        Self::new_with_address(i2c, DEFAULT_I2C_ADDRESS)
    }

    /// Create a new driver instance with a specific I2C address.
    pub fn new_with_address<I2C>(i2c: &mut I2C, address: u8) -> Result<Self, Error<I2C::Error>>
    where
        I2C: I2c,
    {
        let mut buffer = [0u8];
        i2c.write_read(address, &[Register::WhoAmI.addr()], &mut buffer)?;

        if buffer[0] != 0x6b {
            return Err(Error::InvalidDevice(buffer[0]));
        }

        let sensor = Self { address };

        // Set sane defaults: BDU and IF_INC
        sensor.set_bdu(i2c, true)?;
        sensor.set_if_inc(i2c, true)?;

        Ok(sensor)
    }

    /// Create a new driver using an already-configured four-wire SPI device.
    pub fn new_spi<SPI>(spi: &mut SpiDeviceBus<SPI>) -> Result<Self, Error<SPI::Error>>
    where
        SPI: SpiDevice<u8>,
    {
        let mut buffer = [0u8];
        spi.read_register(0, Register::WhoAmI.addr(), &mut buffer)
            .map_err(Error::Spi)?;

        if buffer[0] != 0x6b {
            return Err(Error::InvalidDevice(buffer[0]));
        }

        let sensor = Self {
            address: DEFAULT_I2C_ADDRESS,
        };
        sensor.set_bdu(spi, true).map_err(Error::Spi)?;
        sensor.set_if_inc(spi, true).map_err(Error::Spi)?;

        Ok(sensor)
    }

    pub(crate) fn read_register<B: RegisterBus>(
        &self,
        bus: &mut B,
        register: u8,
        data: &mut [u8],
    ) -> Result<(), B::Error> {
        bus.read_register(self.address, register, data)
    }

    pub(crate) fn write_register<B: RegisterBus>(
        &self,
        bus: &mut B,
        register: u8,
        data: &[u8],
    ) -> Result<(), B::Error> {
        bus.write_register(self.address, register, data)
    }

    pub(crate) fn read_reg<B: RegisterBus>(
        &self,
        bus: &mut B,
        reg: Register,
    ) -> Result<u8, B::Error> {
        let mut buffer = [0u8];
        self.read_register(bus, reg.addr(), &mut buffer)?;
        Ok(buffer[0])
    }

    pub(crate) fn write_reg<B: RegisterBus>(
        &self,
        bus: &mut B,
        reg: Register,
        value: u8,
    ) -> Result<(), B::Error> {
        self.write_register(bus, reg.addr(), &[value])
    }

    pub(crate) fn modify_reg<B: RegisterBus, F>(
        &self,
        bus: &mut B,
        reg: Register,
        f: F,
    ) -> Result<(), B::Error>
    where
        F: FnOnce(u8) -> u8,
    {
        let value = self.read_reg(bus, reg)?;
        let new_value = f(value);
        self.write_reg(bus, reg, new_value)
    }

    /// Set the I2C address.
    pub fn set_address(&mut self, address: u8) {
        self.address = address;
    }

    // ===========================================
    // Configuration
    // ===========================================

    /// Reboot memory content.
    pub fn set_boot<B: RegisterBus>(&mut self, bus: &mut B, boot: bool) -> Result<(), B::Error> {
        self.modify_reg(bus, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_boot(boot);
            reg.into_bytes()[0]
        })
    }

    /// Block Data Update.
    ///
    /// If true, output registers are not updated until MSB and LSB have been read.
    pub fn set_bdu<B: RegisterBus>(&self, bus: &mut B, bdu: bool) -> Result<(), B::Error> {
        self.modify_reg(bus, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_bdu(bdu);
            reg.into_bytes()[0]
        })
    }

    /// Register address automatically incremented during a multiple byte access with a serial interface.
    pub fn set_if_inc<B: RegisterBus>(&self, bus: &mut B, if_inc: bool) -> Result<(), B::Error> {
        self.modify_reg(bus, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_if_inc(if_inc);
            reg.into_bytes()[0]
        })
    }

    /// Reset the device software.
    pub fn set_sw_reset<B: RegisterBus>(&self, bus: &mut B, enable: bool) -> Result<(), B::Error> {
        self.modify_reg(bus, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_sw_reset(enable);
            reg.into_bytes()[0]
        })
    }

    /// Select the serial interface mode (`true` selects 3-wire SPI).
    pub fn set_sim<B: RegisterBus>(&self, bus: &mut B, enable: bool) -> Result<(), B::Error> {
        self.modify_reg(bus, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_sim(enable);
            reg.into_bytes()[0]
        })
    }

    /// Select push-pull (`false`) or open-drain (`true`) interrupt pins.
    pub fn set_pp_od<B: RegisterBus>(&self, bus: &mut B, open_drain: bool) -> Result<(), B::Error> {
        self.modify_reg(bus, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_pp_od(open_drain);
            reg.into_bytes()[0]
        })
    }

    /// Select active-high (`false`) or active-low (`true`) interrupt pins.
    pub fn set_h_lactive<B: RegisterBus>(
        &self,
        bus: &mut B,
        active_low: bool,
    ) -> Result<(), B::Error> {
        self.modify_reg(bus, Register::Ctrl3C, |v| {
            let mut reg = Ctrl3C::from_bytes([v]);
            reg.set_h_lactive(active_low);
            reg.into_bytes()[0]
        })
    }

    // ===========================================
    // Sensors
    // ===========================================

    /// Get temperature in Celsius.
    pub fn get_temperature<B: RegisterBus>(&self, bus: &mut B) -> Result<f32, B::Error> {
        let mut measurements = [0u8; 2];
        self.read_register(bus, Register::OutTempL.addr(), &mut measurements)?;

        let raw_temp = (measurements[1] as i16) << 8 | measurements[0] as i16;
        let temp: f32 = (raw_temp as f32 / 256.0) + 25.0;

        Ok(temp)
    }

    /// Set chain full scale.
    pub fn set_chain_full_scale<B: RegisterBus>(
        &mut self,
        bus: &mut B,
        scale: FsG,
    ) -> Result<&mut Self, B::Error> {
        self.set_gyro_scale(bus, scale)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn parse_acceleromtere_2g() {
        use registers::FsXl;

        // Table 19 in AN5398
        assert_eq!(
            AccelValue::from_msr(FsXl::G2, &[0x0, 0x0, 0x0, 0x0, 0x0, 0x0]).as_m_ss(),
            [0., 0., 0.]
        );

        let a = AccelValue::from_msr(FsXl::G2, &[0x69, 0x16, 0x0, 0x0, 0x0, 0x0]).as_m_ss();
        assert_abs_diff_eq!(a[0], 0.350 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);

        let a = AccelValue::from_msr(FsXl::G2, &[0x09, 0x40, 0x0, 0x0, 0x0, 0x0]).as_m_ss();
        assert_abs_diff_eq!(a[0], 1.0 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);

        let a = AccelValue::from_msr(FsXl::G2, &[0x97, 0xe9, 0x0, 0x0, 0x0, 0x0]).as_m_ss();
        assert_abs_diff_eq!(a[0], -0.350 * SENSORS_GRAVITY_STANDARD, epsilon = 0.01);

        let a = AccelValue::from_msr(FsXl::G2, &[0xf7, 0xbf, 0x0, 0x0, 0x0, 0x0]).as_m_ss();
        assert_abs_diff_eq!(a[0], -SENSORS_GRAVITY_STANDARD, epsilon = 0.01);
    }

    #[test]
    fn parse_gyro_250dps() {
        use registers::FsG;

        // Table 19 in AN5398
        assert_eq!(
            GyroValue::from_msr(FsG::Dps250, &[0x0, 0x0, 0x0, 0x0, 0x0, 0x0]).as_rad(),
            [0., 0., 0.]
        );

        let a = GyroValue::from_msr(FsG::Dps250, &[0xa4, 0x2c, 0x0, 0x0, 0x0, 0x0]).as_rad();
        assert_abs_diff_eq!(a[0], 100. * SENSORS_DPS_TO_RADS, epsilon = 0.01);

        let a = GyroValue::from_msr(FsG::Dps250, &[0x49, 0x59, 0x0, 0x0, 0x0, 0x0]).as_rad();
        assert_abs_diff_eq!(a[0], 200. * SENSORS_DPS_TO_RADS, epsilon = 0.01);

        let a = GyroValue::from_msr(FsG::Dps250, &[0x5c, 0xd3, 0x0, 0x0, 0x0, 0x0]).as_rad();
        assert_abs_diff_eq!(a[0], -100. * SENSORS_DPS_TO_RADS, epsilon = 0.01);

        let a = GyroValue::from_msr(FsG::Dps250, &[0xb7, 0xa6, 0x0, 0x0, 0x0, 0x0]).as_rad();
        assert_abs_diff_eq!(a[0], -200. * SENSORS_DPS_TO_RADS, epsilon = 0.01);
    }

    #[test]
    fn test_set_fifo_mode() {
        // Read WhoAmI, then BDU and IF_INC modification (read Ctrl3C, write Ctrl3C)
        // Note: set_bdu(true) and set_if_inc(true) will each read and write Ctrl3C.
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::WhoAmI.addr()],
                vec![0x6b],
            ),
            // set_bdu(true)
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr()],
                vec![0b00000000],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr(), 0b01000000],
            ),
            // set_if_inc(true)
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr()],
                vec![0b01000000],
            ),
            Transaction::write(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl3C.addr(), 0b01000100],
            ),
            // set_fifo_mode
            Transaction::write_read(DEFAULT_I2C_ADDRESS, vec![0x0A], vec![0b00000000]),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![0x0A, 0b00000001]),
        ]);

        // Pass the mock by reference to the sensor
        let mut sensor = Ism330Dhcx::new(&mut i2c).unwrap();
        sensor.set_fifo_mode(&mut i2c, FifoMode::FifoMode).unwrap();

        // Verify expectations
        i2c.done();
    }

    #[test]
    fn test_new_invalid_device() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::WhoAmI.addr()],
            vec![0xFF],
        )]);

        let sensor = Ism330Dhcx::new(&mut i2c);
        let err = sensor.err().unwrap();
        assert!(matches!(err, Error::InvalidDevice(0xFF)));
        i2c.done();
    }

    macro_rules! ctrl3_modify_test {
        ($name:ident, $method:ident, $read:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut i2c = Mock::new(&[
                    Transaction::write_read(
                        DEFAULT_I2C_ADDRESS,
                        vec![Register::Ctrl3C.addr()],
                        vec![$read],
                    ),
                    Transaction::write(
                        DEFAULT_I2C_ADDRESS,
                        vec![Register::Ctrl3C.addr(), $expected],
                    ),
                ]);
                let sensor = Ism330Dhcx {
                    address: DEFAULT_I2C_ADDRESS,
                };

                sensor.$method(&mut i2c, true).unwrap();
                i2c.done();
            }
        };
    }

    ctrl3_modify_test!(test_set_sw_reset, set_sw_reset, 0xa4, 0xa5);
    ctrl3_modify_test!(test_set_sim, set_sim, 0xa4, 0xac);
    ctrl3_modify_test!(test_set_pp_od, set_pp_od, 0xa4, 0xb4);
    ctrl3_modify_test!(test_set_h_lactive, set_h_lactive, 0x84, 0xa4);

    #[test]
    fn test_set_timestamp_en() {
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Ctrl10C.addr()],
                vec![0x80],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Ctrl10C.addr(), 0xa0]),
        ]);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        sensor.set_timestamp_en(&mut i2c, true).unwrap();
        i2c.done();
    }

    #[test]
    fn test_get_timestamp_reads_three_bytes() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::Timestamp0.addr()],
            vec![0x34, 0x12, 0xab],
        )]);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        assert_eq!(sensor.get_timestamp(&mut i2c).unwrap(), 0x00ab_1234);
        i2c.done();
    }

    #[test]
    fn test_get_status_reads_status_register() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::StatusReg.addr()],
            vec![0x07],
        )]);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        let status = sensor.get_status(&mut i2c).unwrap();
        assert!(status.xl_da());
        assert!(status.g_da());
        assert!(status.t_da());
        i2c.done();
    }

    #[test]
    fn test_get_all_interrupt_sources_reads_register() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::AllIntSrc.addr()],
            vec![0x81],
        )]);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        let source = sensor.get_interrupt_sources(&mut i2c).unwrap();
        assert!(source.ff_ia());
        assert!(!source.d6d_ia());
        i2c.done();
    }

    #[test]
    fn test_get_wake_up_source_reads_register() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::WakeUpSrc.addr()],
            vec![0x08],
        )]);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        assert!(sensor.get_wake_up_source(&mut i2c).unwrap().wu_ia());
        i2c.done();
    }

    #[test]
    fn test_get_tap_source_reads_register() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::TapSrc.addr()],
            vec![0x40],
        )]);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        assert!(sensor.get_tap_source(&mut i2c).unwrap().tap_ia());
        i2c.done();
    }

    #[test]
    fn test_get_6d_source_reads_register() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            DEFAULT_I2C_ADDRESS,
            vec![Register::D6dSrc.addr()],
            vec![0x40],
        )]);
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };

        assert!(sensor.get_6d_source(&mut i2c).unwrap().d6d_ia());
        i2c.done();
    }
}
