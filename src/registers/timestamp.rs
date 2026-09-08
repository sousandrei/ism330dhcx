use modular_bitfield::bitfield;

macro_rules! timestamp_register {
    ($name:ident, $doc:literal) => {
        #[bitfield]
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        #[doc = $doc]
        pub struct $name {
            pub value: u8,
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
timestamp_register!(Timestamp0, "Timestamp register 0.");
timestamp_register!(Timestamp1, "Timestamp register 1.");
timestamp_register!(Timestamp2, "Timestamp register 2.");
timestamp_register!(Timestamp3, "Timestamp register 3.");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn layout_and_default() {
        assert_eq!(Timestamp0::default().into_bytes(), [0]);
        let mut r = Timestamp3::new();
        r.set_value(0xa5);
        assert_eq!(r.into_bytes(), [0xa5]);
    }
}
