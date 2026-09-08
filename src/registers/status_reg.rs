use modular_bitfield::{bitfield, specifiers::B5};
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct StatusReg {
    pub xl_da: bool,
    pub g_da: bool,
    pub t_da: bool,
    #[skip]
    pub reserved: B5,
}
impl Default for StatusReg {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(StatusReg::default().into_bytes(), [0]);
        let mut r = StatusReg::new();
        r.set_t_da(true);
        assert_eq!(r.into_bytes(), [4]);
    }
}
