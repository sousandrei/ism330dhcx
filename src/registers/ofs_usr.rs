use modular_bitfield::bitfield;

macro_rules! offset_register {
    ($name:ident, $doc:literal) => {
        #[bitfield]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        #[doc = $doc]
        pub struct $name {
            pub offset: u8,
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
offset_register!(XOfsUsr, "User X-axis offset register.");
offset_register!(YOfsUsr, "User Y-axis offset register.");
offset_register!(ZOfsUsr, "User Z-axis offset register.");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(XOfsUsr::default().into_bytes(), [0]);
        let mut r = ZOfsUsr::new();
        r.set_offset(0x5a);
        assert_eq!(r.into_bytes(), [0x5a]);
    }
}
