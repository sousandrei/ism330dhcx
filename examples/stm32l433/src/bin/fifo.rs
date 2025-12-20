#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_time::Timer;

use ism330dhcx::registers::fifo::{BdrGy, BdrXl, FifoMode};
use ism330dhcx::registers::{FsG, FsXl, OdrG, OdrXl};
use ism330dhcx::{Accelerometer, Ctrl3CConfig, Fifo, Gyroscope, Ism330Dhcx};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    //==========================================
    // Initilizing board

    let p = embassy_stm32::init(Default::default());

    //==========================================
    // Declaring I2C1

    let mut i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB7, Default::default());

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
    boot_sensor(&mut i2c, &mut sensor);

    // =======================================
    // FIFO configuration

    sensor
        .set_fifo_mode(&mut i2c, FifoMode::Continuous)
        .unwrap();
    sensor
        .set_fifo_accel_batch_rate(&mut i2c, BdrXl::Hz52)
        .unwrap();
    sensor
        .set_fifo_gyro_batch_rate(&mut i2c, BdrGy::Hz52)
        .unwrap();

    defmt::info!("FIFO initialized");

    loop {
        let status = sensor.get_fifo_status(&mut i2c).unwrap();
        let diff = status.diff_fifo();

        if diff > 0 {
            defmt::info!("FIFO has {} samples", diff);

            for _ in 0..diff {
                match sensor.fifo_pop(&mut i2c).unwrap() {
                    ism330dhcx::Value::Accel(accel) => {
                        defmt::info!("Accel: {:?}", accel.as_m_ss());
                    }
                    ism330dhcx::Value::Gyro(gyro) => {
                        defmt::info!("Gyro: {:?}", gyro.as_dps());
                    }
                    ism330dhcx::Value::Empty => {}
                    _ => {}
                }
            }
        }

        Timer::after_millis(500).await;
    }
}

// Booting the sensor accoring to Adafruit's driver
fn boot_sensor<I2C>(i2c: &mut I2C, sensor: &mut Ism330Dhcx)
where
    I2C: embedded_hal::i2c::I2c,
{
    // =======================================
    // CTRL3_C

    sensor.set_boot(i2c, true).unwrap();
    sensor.set_bdu(i2c, true).unwrap();
    sensor.set_if_inc(i2c, true).unwrap();

    // =======================================
    // CTRL9_XL

    sensor.set_den_x(i2c, true).unwrap();
    sensor.set_den_y(i2c, true).unwrap();
    sensor.set_den_z(i2c, true).unwrap();
    sensor.set_den_device_conf(i2c, true).unwrap();

    // =======================================
    // CTRL1_XL

    sensor.set_accel_odr(i2c, OdrXl::Hz52).unwrap();
    sensor.set_accel_scale(i2c, FsXl::G4).unwrap();
    sensor.set_lpf2_xl_en(i2c, true).unwrap();

    // =======================================
    // CTRL2_G

    sensor.set_gyro_odr(i2c, OdrG::Hz52).unwrap();
    sensor.set_gyro_scale(i2c, FsG::Dps500).unwrap();

    // =======================================
    // CTRL7_G

    sensor.set_g_hm_mode(i2c, true).unwrap();
}
