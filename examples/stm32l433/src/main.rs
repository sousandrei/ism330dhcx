#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_time::Timer;

use ism330dhcx::registers::{FsG, FsXl, OdrG, OdrXl};
use ism330dhcx::Ism330Dhcx;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    //==========================================
    // Initilizing board

    let p = embassy_stm32::init(Default::default());

    //==========================================
    // Declaring I2C1

    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB7, Default::default());

    //==============================================
    // Declaring sensor

    let mut sensor = match Ism330Dhcx::new(i2c) {
        Ok(sensor) => sensor,
        Err(error) => {
            defmt::error!("{:?}", error);
            panic!("failed to create sensor")
        }
    };

    // Initializing sensor
    boot_sensor(&mut sensor);

    // =======================================

    loop {
        defmt::info!("Temperature: {}", sensor.get_temperature().unwrap());
        defmt::info!(
            "Gyroscope: {:?}",
            sensor.get_gyroscope().unwrap().as_dps()
        );
        defmt::info!(
            "Accelerometer: {:?}",
            sensor.get_accelerometer().unwrap().as_m_ss()
        );

        Timer::after_millis(500).await;
    }
}

// Booting the sensor accoring to Adafruit's driver
fn boot_sensor<I2C, E>(sensor: &mut Ism330Dhcx<I2C>)
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
    E: core::fmt::Debug,
{
    // =======================================
    // CTRL3_C

    sensor.set_boot(true).unwrap();
    sensor.set_bdu(true).unwrap();
    sensor.set_if_inc(true).unwrap();

    // =======================================
    // CTRL9_XL

    sensor.set_den_x(true).unwrap();
    sensor.set_den_y(true).unwrap();
    sensor.set_den_z(true).unwrap();
    sensor.set_device_conf(true).unwrap();

    // =======================================
    // CTRL1_XL

    sensor.set_accel_odr(OdrXl::Hz52).unwrap();
    sensor.set_accel_scale(FsXl::G4).unwrap();
    sensor.set_lpf2_xl_en(true).unwrap();

    // =======================================
    // CTRL2_G

    sensor.set_gyro_odr(OdrG::Hz52).unwrap();
    sensor.set_gyro_scale(FsG::Dps500).unwrap();

    // =======================================
    // CTRL7_G

    sensor.set_g_hm_mode(true).unwrap();
}
