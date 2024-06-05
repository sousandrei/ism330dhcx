#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::Hertz;
use embassy_time::Timer;

use ism330dhcx::{ctrl1xl, ctrl2g, Ism330Dhcx};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    //==========================================
    // Initilizing board

    let p = embassy_stm32::init(Default::default());

    //==========================================
    // Declaring I2C1

    let mut i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB7, Hertz(100_000), Default::default());

    //==============================================
    // Declaring sensor

    let mut sensor = match Ism330Dhcx::new(&mut i2c) {
        Ok(sensor) => sensor,
        Err(error) => {
            defmt::error!("{:?}", error);
            panic!("failed to create sensor")
        }
    };

    // Initializing sensor
    boot_sensor(&mut sensor, &mut i2c);

    // =======================================

    loop {
        defmt::info!("Temperature: {}", sensor.get_temperature(&mut i2c).unwrap());
        defmt::info!(
            "Gyroscope: {:?}",
            sensor.get_gyroscope(&mut i2c).unwrap().as_dps()
        );
        defmt::info!(
            "Accelerometer: {:?}",
            sensor.get_accelerometer(&mut i2c).unwrap().as_m_ss()
        );

        Timer::after_millis(500).await;
    }
}

// Booting the sensor accoring to Adafruit's driver
fn boot_sensor<I2C>(sensor: &mut Ism330Dhcx, i2c: &mut I2C)
where
    I2C: embedded_hal::i2c::I2c,
{
    // =======================================
    // CTRL3_C

    sensor.ctrl3c.set_boot(i2c, true).unwrap();
    sensor.ctrl3c.set_bdu(i2c, true).unwrap();
    sensor.ctrl3c.set_if_inc(i2c, true).unwrap();

    // =======================================
    // CTRL9_XL

    sensor.ctrl9xl.set_den_x(i2c, true).unwrap();
    sensor.ctrl9xl.set_den_y(i2c, true).unwrap();
    sensor.ctrl9xl.set_den_z(i2c, true).unwrap();
    sensor.ctrl9xl.set_device_conf(i2c, true).unwrap();

    // =======================================
    // CTRL1_XL

    sensor
        .ctrl1xl
        .set_accelerometer_data_rate(i2c, ctrl1xl::Odr_Xl::Hz52)
        .unwrap();

    sensor
        .ctrl1xl
        .set_chain_full_scale(i2c, ctrl1xl::Fs_Xl::G4)
        .unwrap();
    sensor.ctrl1xl.set_lpf2_xl_en(i2c, true).unwrap();

    // =======================================
    // CTRL2_G

    sensor
        .ctrl2g
        .set_gyroscope_data_rate(i2c, ctrl2g::Odr::Hz52)
        .unwrap();

    sensor
        .ctrl2g
        .set_chain_full_scale(i2c, ctrl2g::Fs::Dps500)
        .unwrap();

    // =======================================
    // CTRL7_G

    sensor.ctrl7g.set_g_hm_mode(i2c, true).unwrap();
}
