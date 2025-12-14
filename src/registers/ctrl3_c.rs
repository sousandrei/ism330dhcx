#![allow(unused_parens)]
use modular_bitfield::{bitfield, specifiers::B1};

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl3C {
    pub sw_reset: bool,
    #[skip]
    pub __: B1,
    pub if_inc: bool,
    pub sim: bool,
    pub pp_od: bool,
    pub h_lactive: bool,
    pub bdu: bool,
    pub boot: bool,
}
