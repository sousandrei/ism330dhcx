use modular_bitfield::{bitfield, specifiers::B6};

/// SDO, OCS_AUX, SDO_AUX pins pull-up enable/disable register. (02h)
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PinCtrl {
    /// Reserved bits, must be set to 1 for correct operation.
    pub reserved: B6,
    /// Enable pull-up on SDO pin.
    pub sdo_pu_en: bool,
    /// Disable pull-up on both OCS_Aux and SDO_Aux pins.
    pub ois_pu_dis: bool,
}

impl Default for PinCtrl {
    fn default() -> Self {
        // Datasheet says bits 5:0 must be set to 1.
        Self::from_bytes([0x3F])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_ctrl() {
        let mut reg = PinCtrl::default();
        assert_eq!(reg.into_bytes()[0], 0x3F);

        reg.set_sdo_pu_en(true);
        assert_eq!(reg.into_bytes()[0], 0x7F);

        reg.set_ois_pu_dis(true);
        assert_eq!(reg.into_bytes()[0], 0xFF);
    }
}
