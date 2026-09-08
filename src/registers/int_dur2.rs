use modular_bitfield::{
    bitfield,
    specifiers::{B2, B4},
};
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IntDur2 {
    pub shock: B2,
    pub quiet: B2,
    pub dur: B4,
}
impl Default for IntDur2 {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(IntDur2::default().into_bytes(), [0]);
        let mut r = IntDur2::new();
        r.set_dur(3);
        assert_eq!(r.into_bytes(), [0x30]);
    }
}
