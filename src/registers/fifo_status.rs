#![allow(unused_parens)]
use modular_bitfield::{
    bitfield,
    specifiers::{B1, B3, B10},
};

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FifoStatus {
    pub diff_fifo: B10,
    #[skip]
    pub __: B1,
    pub fifo_ovr_ia: bool,
    pub fifo_full_ia: bool,
    #[skip]
    pub __: B3,
}
