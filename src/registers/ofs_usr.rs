use modular_bitfield::bitfield;

macro_rules! offset_register {
    ($name:ident) => {
        #[bitfield]
        #[derive(Debug, Copy, Clone)]
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
offset_register!(XOfsUsr);
offset_register!(YOfsUsr);
offset_register!(ZOfsUsr);

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
