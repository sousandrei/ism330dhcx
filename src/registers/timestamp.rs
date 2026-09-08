use crate::registers::Register;
use crate::Ism330Dhcx;
use embedded_hal::i2c::I2c;
use modular_bitfield::bitfield;

macro_rules! timestamp_register {
    ($name:ident) => {
        #[bitfield]
        #[derive(Debug, Copy, Clone)]
        pub struct $name {
            pub value: u8,
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
timestamp_register!(Timestamp0);
timestamp_register!(Timestamp1);
timestamp_register!(Timestamp2);
timestamp_register!(Timestamp3);

pub trait TimestampConfig {
    fn get_timestamp<I2C>(&self, i2c: &mut I2C) -> Result<u32, I2C::Error>
    where
        I2C: I2c;
}

impl TimestampConfig for Ism330Dhcx {
    fn get_timestamp<I2C>(&self, i2c: &mut I2C) -> Result<u32, I2C::Error>
    where
        I2C: I2c,
    {
        let mut bytes = [0u8; 4];
        i2c.write_read(self.address, &[Register::Timestamp0.addr()], &mut bytes)?;
        Ok(u32::from_le_bytes(bytes) & 0x00ff_ffff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Timestamp0::default().into_bytes(), [0]);
        let mut r = Timestamp3::new();
        r.set_value(0xa5);
        assert_eq!(r.into_bytes(), [0xa5]);
    }
}
