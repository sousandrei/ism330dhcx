#![allow(unused_parens)]
use modular_bitfield::{bitfield, specifiers::B1};

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl9Xl {
    #[skip]
    pub __: B1,
    pub device_conf: bool,
    pub den_lh: bool,
    pub den_xl_en: bool,
    pub den_xl_g: bool,
    pub den_x: bool,
    pub den_y: bool,
    pub den_z: bool,
}

impl Default for Ctrl9Xl {
    fn default() -> Self {
        Self::new()
    }
}
