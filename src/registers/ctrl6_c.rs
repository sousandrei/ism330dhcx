use modular_bitfield::{Specifier, bitfield};

#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 3]
pub enum Ftype {
    Bw0 = 0b00,
    Bw1 = 0b01,
    Bw2 = 0b10,
    Bw3 = 0b11,
    Bw4 = 0b100,
    Bw5 = 0b101,
    Bw6 = 0b110,
    Bw7 = 0b111,
}

/// Control register 6.
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl6C {
    pub ftype: Ftype,
    pub usr_off_w: bool,
    pub xl_hm_mode: bool,
    pub lvl2_en: bool,
    pub lvl1_en: bool,
    pub trig_en: bool,
}
impl Default for Ctrl6C {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Ctrl6C::default().into_bytes(), [0]);
        let mut r = Ctrl6C::new();
        r.set_trig_en(true);
        assert_eq!(r.into_bytes(), [0x80]);
    }

    #[test]
    fn filter_type_is_two_bits() {
        let mut reg = Ctrl6C::new();
        reg.set_ftype(Ftype::Bw3);
        assert_eq!(reg.into_bytes()[0] & 0x03, 0x03);
    }
}
