use modular_bitfield::{
    bitfield,
    specifiers::{B3, B5},
};
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct FreeFall {
    pub ff_ths: B3,
    pub ff_dur: B5,
}
impl Default for FreeFall {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(FreeFall::default().into_bytes(), [0]);
        let mut r = FreeFall::new();
        r.set_ff_ths(7);
        assert_eq!(r.into_bytes(), [7]);
    }
}
