use modular_bitfield::bitfield;
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Md1Cfg {
    pub int1_inact_state: bool,
    pub int1_single_tap: bool,
    pub int1_wu: bool,
    pub int1_ff: bool,
    pub int1_double_tap: bool,
    pub int1_6d: bool,
    pub int1_tilt: bool,
    pub int1_emb_func: bool,
}
impl Default for Md1Cfg {
    fn default() -> Self {
        Self::new()
    }
}
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Md2Cfg {
    pub int2_inact_state: bool,
    pub int2_single_tap: bool,
    pub int2_wu: bool,
    pub int2_ff: bool,
    pub int2_double_tap: bool,
    pub int2_6d: bool,
    pub int2_tilt: bool,
    pub int2_emb_func: bool,
}
impl Default for Md2Cfg {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Md1Cfg::default().into_bytes(), [0]);
        assert_eq!(Md2Cfg::default().into_bytes(), [0]);
        let mut r = Md1Cfg::new();
        r.set_int1_emb_func(true);
        assert_eq!(r.into_bytes(), [0x80]);
    }
}
