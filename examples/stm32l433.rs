#![no_std]
#![no_main]

use cortex_m_rt as rt;
use panic_semihosting as _;

use core::fmt::{Debug, Write};
use cortex_m_semihosting::hio::{self};
use rt::entry;
use stm32l4xx_hal::{
    delay::Delay,
    i2c::{self, I2c},
    prelude::*,
    stm32,
};

use ism330dhcx::{ctrl1xl, ctrl2g, Ism330Dhcx};

#[entry]
fn main() -> ! {
    //==========================================
    // Initilizing board

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc
        .cfgr
        .sysclk(80.MHz())
        .pclk1(8.MHz())
        .pclk2(80.MHz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut delay = Delay::new(cp.SYST, clocks);

    //==========================================
    // Declaring I2C1

    let scl =
        gpiob
            .pb8
            .into_alternate_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrh);

    let sda =
        gpiob
            .pb7
            .into_alternate_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    let mut i2c = I2c::i2c1(
        dp.I2C1,
        (scl, sda),
        i2c::Config::new(100.kHz(), clocks),
        &mut rcc.apb1r1,
    );

    //==============================================

    let mut stdout = hio::hstdout().unwrap();

    let mut sensor = match Ism330Dhcx::new(&mut i2c) {
        Ok(sensor) => sensor,
        Err(error) => {
            writeln!(stdout, "{:?}", error).unwrap();
            panic!("failed to create sensor")
        }
    };

    boot_sensor(&mut sensor, &mut i2c);

    loop {
        writeln!(stdout, "{}", sensor.get_temperature(&mut i2c).unwrap()).unwrap();
        writeln!(stdout, "{:?}", sensor.get_gyroscope(&mut i2c).unwrap()).unwrap();
        writeln!(stdout, "{:?}", sensor.get_accelerometer(&mut i2c).unwrap()).unwrap();

        delay.delay_ms(500u32);
    }
}

// Booting the sensor accoring to Adafruit's driver
fn boot_sensor<I2C, E>(sensor: &mut Ism330Dhcx, i2c: &mut I2C)
where
    I2C: embedded_hal::i2c::I2c,
    E: Debug,
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
