use modular_bitfield::{bitfield, specifiers::B1};
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AllIntSrc {
    pub ff_ia: bool,
    pub wu_ia: bool,
    pub single_tap: bool,
    pub double_tap: bool,
    pub d6d_ia: bool,
    pub sleep_change_ia: bool,
    #[skip]
    pub __: B1,
    pub timestamp_endcount: bool,
}
impl Default for AllIntSrc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(AllIntSrc::default().into_bytes(), [0]);
        let mut r = AllIntSrc::new();
        r.set_timestamp_endcount(true);
        assert_eq!(r.into_bytes(), [0x80]);

        r.set_sleep_change_ia(true);
        r.set_d6d_ia(true);
        r.set_double_tap(true);
        r.set_single_tap(true);
        r.set_wu_ia(true);
        r.set_ff_ia(true);
        assert_eq!(r.into_bytes(), [0xbf]);
    }
}
