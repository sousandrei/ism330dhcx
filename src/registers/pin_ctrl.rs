use modular_bitfield::prelude::*;

/// SDO, OCS_AUX, SDO_AUX pins pull-up enable/disable register (02h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct PinCtrl {
    #[skip]
    pub __: B6,
    /// Enable pull-up on SDO pin.
    pub sdo_pu_en: bool,
    /// Disable pull-up on both OCS_Aux and SDO_Aux pins.
    pub ois_pu_dis: bool,
}

impl Default for PinCtrl {
    fn default() -> Self {
        Self::new().with_ois_pu_dis(false).with_sdo_pu_en(false)
    }
}
