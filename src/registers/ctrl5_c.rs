#![allow(non_snake_case)]
use modular_bitfield::{Specifier, bitfield, specifiers::B1, specifiers::B2};

#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum StXl {
    Normal = 0b00,
    Positive = 0b01,
    Negative = 0b10,
}

#[derive(Specifier, Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
#[bits = 2]
pub enum StG {
    Normal = 0b00,
    Positive = 0b01,
    Negative = 0b11,
}

/// Control register 5.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl5C {
    pub st_xl: StXl,
    pub st_g: StG,
    pub __: B1,
    pub rounding: B2,
    pub rounding_status: bool,
}
impl Default for Ctrl5C {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Ctrl5C::default().into_bytes(), [0]);
        let mut r = Ctrl5C::new();
        r.set_rounding_status(true);
        assert_eq!(r.into_bytes(), [0x80]);
    }

    #[test]
    fn self_test_selectors_use_typed_values() {
        let mut reg = Ctrl5C::new();
        reg.set_st_xl(StXl::Negative);
        reg.set_st_g(StG::Negative);
        assert_eq!(reg.into_bytes()[0] & 0x0f, 0x0e);
    }
}
