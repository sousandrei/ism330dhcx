use bitfield::bitfield;

/// Gyroscope high-pass filter cutoff selection.
#[derive(Debug, Copy, Clone, Eq, PartialEq, defmt::Format)]
pub enum HpmG {
    /// 16 mHz
    Hpmg16 = 0b00,
    /// 65 mHz
    Hpmg65 = 0b01,
    /// 260 mHz
    Hpmg260 = 0b10,
    /// 1.04 Hz
    Hpmg104 = 0b11,
}

impl From<u8> for HpmG {
    fn from(val: u8) -> Self {
        match val {
            0b00 => HpmG::Hpmg16,
            0b01 => HpmG::Hpmg65,
            0b10 => HpmG::Hpmg260,
            0b11 => HpmG::Hpmg104,
            _ => HpmG::Hpmg16,
        }
    }
}

impl From<HpmG> for u8 {
    fn from(val: HpmG) -> u8 {
        val as u8
    }
}

bitfield! {
    /// Control register 7 (Gyroscope).
    pub struct Ctrl7G(u8);
    impl Debug;
    /// OIS enable.
    pub ois_on, set_ois_on: 0;
    /// User off on out enable.
    pub usr_off_on_out, set_usr_off_on_out: 1;
    /// OIS enable.
    pub ois_on_en, set_ois_on_en: 2;
    /// High-pass filter cutoff freq.
    pub from into HpmG, hpm_g, set_hpm_g: 4, 3;
    /// High-pass filter enable.
    pub hp_en_g, set_hp_en_g: 6;
    /// High-performance operating mode disable (0 = enabled).
    pub g_hm_mode, set_g_hm_mode: 7;
}

// Correction for bit 1: modular-bitfield was:
// pub usr_off_on_out: bool, (bit 1)
// pub ois_on_en: bool, (bit 2)

impl Ctrl7G {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn from_bytes(bytes: [u8; 1]) -> Self {
        Self(bytes[0])
    }
    pub fn into_bytes(self) -> [u8; 1] {
        [self.0]
    }
}

impl Default for Ctrl7G {
    fn default() -> Self {
        Self::new()
    }
}

impl Copy for Ctrl7G {}
impl Clone for Ctrl7G {
    fn clone(&self) -> Self {
        *self
    }
}
