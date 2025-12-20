use bitfield::bitfield;

bitfield! {
    /// SDO, OCS_AUX, SDO_AUX pins pull-up enable/disable register (02h)
    pub struct PinCtrl(u8);
    impl Debug;
    /// Enable pull-up on SDO pin.
    pub sdo_pu_en, set_sdo_pu_en: 6;
    /// Disable pull-up on both OCS_Aux and SDO_Aux pins.
    pub ois_pu_dis, set_ois_pu_dis: 7;
}

impl PinCtrl {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
    // Added for compatibility with original code's custom Default
    pub fn with_ois_pu_dis(mut self, val: bool) -> Self {
        self.set_ois_pu_dis(val);
        self
    }
    pub fn with_sdo_pu_en(mut self, val: bool) -> Self {
        self.set_sdo_pu_en(val);
        self
    }
}

impl Default for PinCtrl {
    fn default() -> Self {
        Self::new().with_ois_pu_dis(false).with_sdo_pu_en(false)
    }
}

impl Copy for PinCtrl {}
impl Clone for PinCtrl {
    fn clone(&self) -> Self {
        *self
    }
}
