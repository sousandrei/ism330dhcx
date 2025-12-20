use modular_bitfield::prelude::*;

/// Linear acceleration sensor self-test mode selection.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 2]
pub enum StXl {
    /// Normal mode
    Normal = 0b00,
    /// Positive sign self-test
    Positive = 0b01,
    /// Negative sign self-test
    Negative = 0b10,
}

/// Angular rate sensor self-test mode selection.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 2]
pub enum StG {
    /// Normal mode
    Normal = 0b00,
    /// Positive sign self-test
    Positive = 0b01,
    /// Negative sign self-test
    Negative = 0b11,
}

/// Circular burst-mode (rounding) read selection.
#[derive(BitfieldSpecifier, Debug, Copy, Clone, Eq, PartialEq)]
#[bits = 2]
pub enum Rounding {
    /// No rounding
    None = 0b00,
    /// Accelerometer only
    AccelOnly = 0b01,
    /// Gyroscope only
    GyroOnly = 0b10,
    /// Gyroscope + Accelerometer
    GyroAccel = 0b11,
}

/// Control register 5 (14h)
#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct Ctrl5C {
    /// Linear acceleration sensor self-test enable.
    pub st_xl: StXl,
    /// Angular rate sensor self-test enable.
    pub st_g: StG,
    #[skip]
    pub __1: B1,
    /// Circular burst-mode (rounding) read of the output registers.
    pub rounding: Rounding,
    #[skip]
    pub __2: B1,
}

impl Default for Ctrl5C {
    fn default() -> Self {
        Self::new()
    }
}
