use modular_bitfield::{bitfield, specifiers::B1};

/// Interrupt routing control for INT1.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Int1Ctrl {
    pub int1_drdy_xl: bool,
    pub int1_drdy_g: bool,
    pub int1_boot: bool,
    pub int1_fifo_th: bool,
    pub int1_fifo_ovr: bool,
    pub int1_fifo_full: bool,
    pub int1_cnt_bdr: bool,
    pub den_drdy_flag: bool,
}
impl Default for Int1Ctrl {
    fn default() -> Self {
        Self::new()
    }
}

/// Interrupt routing control for INT2.
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Int2Ctrl {
    pub int2_drdy_xl: bool,
    pub int2_drdy_g: bool,
    pub int2_drdy_temp: bool,
    pub int2_fifo_th: bool,
    pub int2_fifo_ovr: bool,
    pub int2_fifo_full: bool,
    pub int2_cnt_bdr: bool,
    #[skip]
    pub __: B1,
}
impl Default for Int2Ctrl {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Int1Ctrl::default().into_bytes(), [0]);
        assert_eq!(Int2Ctrl::default().into_bytes(), [0]);
        let mut r = Int1Ctrl::new();
        r.set_den_drdy_flag(true);
        assert_eq!(r.into_bytes(), [0x80]);
    }
}
