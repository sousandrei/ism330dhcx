use embedded_hal::i2c::I2c;

use crate::Ism330Dhcx;
use crate::registers::{Int1Ctrl, Int2Ctrl, Md1Cfg, Md2Cfg, Register};

/// Interrupt routing configuration.
pub trait Interrupts {
    fn set_int1_drdy_xl<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int1_drdy_g<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int1_fifo_threshold<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int1_fifo_overrun<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int1_fifo_full<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int1_counter_bdr<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int1_den_drdy<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int2_drdy_xl<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int2_drdy_g<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int2_drdy_temp<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int2_fifo_threshold<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int2_fifo_overrun<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int2_fifo_full<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int2_counter_bdr<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int1_embedded_function<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
    fn set_int2_embedded_function<I2C>(
        &self,
        i2c: &mut I2C,
        enable: bool,
    ) -> Result<(), I2C::Error>
    where
        I2C: I2c;
}

macro_rules! route_setters {
    ($(fn $method:ident, $register:ident, $config:ident, $setter:ident;)*) => {
        $(
            fn $method<I2C>(&self, i2c: &mut I2C, enable: bool) -> Result<(), I2C::Error>
            where
                I2C: I2c,
            {
                self.modify_reg(i2c, Register::$register, |value| {
                    let mut reg = $config::from_bytes([value]);
                    reg.$setter(enable);
                    reg.into_bytes()[0]
                })
            }
        )*
    };
}

impl Interrupts for Ism330Dhcx {
    route_setters! {
        fn set_int1_drdy_xl, Int1Ctrl, Int1Ctrl, set_int1_drdy_xl;
        fn set_int1_drdy_g, Int1Ctrl, Int1Ctrl, set_int1_drdy_g;
        fn set_int1_fifo_threshold, Int1Ctrl, Int1Ctrl, set_int1_fifo_th;
        fn set_int1_fifo_overrun, Int1Ctrl, Int1Ctrl, set_int1_fifo_ovr;
        fn set_int1_fifo_full, Int1Ctrl, Int1Ctrl, set_int1_fifo_full;
        fn set_int1_counter_bdr, Int1Ctrl, Int1Ctrl, set_int1_cnt_bdr;
        fn set_int1_den_drdy, Int1Ctrl, Int1Ctrl, set_den_drdy_flag;
        fn set_int2_drdy_xl, Int2Ctrl, Int2Ctrl, set_int2_drdy_xl;
        fn set_int2_drdy_g, Int2Ctrl, Int2Ctrl, set_int2_drdy_g;
        fn set_int2_drdy_temp, Int2Ctrl, Int2Ctrl, set_int2_drdy_temp;
        fn set_int2_fifo_threshold, Int2Ctrl, Int2Ctrl, set_int2_fifo_th;
        fn set_int2_fifo_overrun, Int2Ctrl, Int2Ctrl, set_int2_fifo_ovr;
        fn set_int2_fifo_full, Int2Ctrl, Int2Ctrl, set_int2_fifo_full;
        fn set_int2_counter_bdr, Int2Ctrl, Int2Ctrl, set_int2_cnt_bdr;
        fn set_int1_embedded_function, Md1Cfg, Md1Cfg, set_int1_emb_func;
        fn set_int2_embedded_function, Md2Cfg, Md2Cfg, set_int2_emb_func;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DEFAULT_I2C_ADDRESS;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    #[test]
    fn routes_fifo_threshold_to_int1() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Int1Ctrl.addr()],
                vec![0x40],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Int1Ctrl.addr(), 0x48]),
        ]);

        sensor.set_int1_fifo_threshold(&mut i2c, true).unwrap();
        i2c.done();
    }

    #[test]
    fn routes_embedded_function_to_int2() {
        let sensor = Ism330Dhcx {
            address: DEFAULT_I2C_ADDRESS,
        };
        let mut i2c = Mock::new(&[
            Transaction::write_read(
                DEFAULT_I2C_ADDRESS,
                vec![Register::Md2Cfg.addr()],
                vec![0x01],
            ),
            Transaction::write(DEFAULT_I2C_ADDRESS, vec![Register::Md2Cfg.addr(), 0x41]),
        ]);

        sensor.set_int2_embedded_function(&mut i2c, true).unwrap();
        i2c.done();
    }
}
