#![allow(unused_parens)]
use modular_bitfield::{bitfield, specifiers::B1};

/// Control register 9 (XL).
#[bitfield]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ctrl9Xl {
    #[skip]
    pub __: B1,
    /// Device configuration.
    pub device_conf: bool,
    /// DEN active level.
    pub den_lh: bool,
    /// DEN stamping on accelerometer axis.
    pub den_xl_en: bool,
    /// DEN stamping on gyroscope axis.
    pub den_xl_g: bool,
    /// DEN value stored in LSB of X-axis.
    pub den_x: bool,
    /// DEN value stored in LSB of Y-axis.
    pub den_y: bool,
    /// DEN value stored in LSB of Z-axis.
    pub den_z: bool,
}

impl Default for Ctrl9Xl {
    fn default() -> Self {
        Self::from_bytes([0xe0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_matches_reset_value() {
        assert_eq!(Ctrl9Xl::default().into_bytes(), [0xe0]);
    }
}
