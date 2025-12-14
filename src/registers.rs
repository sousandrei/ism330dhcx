use defmt::Format;

/// Register addresses
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    FifoCtrl1 = 0x07,
    FifoCtrl2 = 0x08,
    FifoCtrl3 = 0x09,
    FifoCtrl4 = 0x0A,
    Ctrl1Xl = 0x10,
    Ctrl2G = 0x11,
    Ctrl3C = 0x12,
    Ctrl7G = 0x16,
    Ctrl9Xl = 0x18,
    FifoStatus1 = 0x3A,
    FifoStatus2 = 0x3B,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

// =============================================================================
// CTRL1_XL (0x10) Accelerometer Control
// =============================================================================

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Format)]
pub enum FsXl {
    G2 = 0b00,  // ±2  g
    G16 = 0b01, // ±16 g
    G4 = 0b10,  // ±4  g
    G8 = 0b11,  // ±8  g
}

impl FsXl {
    pub fn sensitivity(&self) -> f32 {
        match self {
            FsXl::G2 => 0.061,
            FsXl::G4 => 0.122,
            FsXl::G8 => 0.244,
            FsXl::G16 => 0.488,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Format)]
#[repr(u8)]
pub enum OdrXl {
    Off = 0b0000,
    Hz12_5 = 0b0001,
    Hz26 = 0b0010,
    Hz52 = 0b0011,
    Hz104 = 0b0100,
    Hz208 = 0b0101,
    Hz416 = 0b0110,
    Hz833 = 0b0111,
    Hz1667 = 0b1000,
    Hz3333 = 0b1001,
    Hz6667 = 0b1010,
}

pub const CTRL1_XL_ODR_MASK: u8 = 0b1111_0000;
pub const CTRL1_XL_ODR_SHIFT: u8 = 4;
pub const CTRL1_XL_FS_MASK: u8 = 0b0000_1100;
pub const CTRL1_XL_FS_SHIFT: u8 = 2;
pub const CTRL1_XL_LPF2_XL_EN: u8 = 1 << 1;

// =============================================================================
// CTRL2_G (0x11) Gyroscope Control
// =============================================================================

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Format)]
pub enum FsG {
    Dps125,
    Dps250,
    Dps500,
    Dps1000,
    Dps2000,
    Dps4000,
}

impl FsG {
    pub fn sensitivity(&self) -> f32 {
        match self {
            FsG::Dps125 => 4.375,
            FsG::Dps250 => 8.750,
            FsG::Dps500 => 17.50,
            FsG::Dps1000 => 35.,
            FsG::Dps2000 => 70.,
            FsG::Dps4000 => 140.,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Format)]
#[repr(u8)]
pub enum OdrG {
    Off = 0b0000,
    Hz12_5 = 0b0001,
    Hz26 = 0b0010,
    Hz52 = 0b0011,
    Hz104 = 0b0100,
    Hz208 = 0b0101,
    Hz416 = 0b0110,
    Hz833 = 0b0111,
    Hz1667 = 0b1000,
    Hz3333 = 0b1001,
    Hz6667 = 0b1010,
}

pub const CTRL2_G_ODR_MASK: u8 = 0b1111_0000;
pub const CTRL2_G_ODR_SHIFT: u8 = 4;
pub const CTRL2_G_FS_MASK: u8 = 0b0000_1100;
pub const CTRL2_G_FS_SHIFT: u8 = 2;
pub const CTRL2_G_FS_125: u8 = 1 << 1;
pub const CTRL2_G_FS_4000: u8 = 1;

// =============================================================================
// CTRL3_C (0x12)
// =============================================================================

pub const CTRL3_C_BOOT: u8 = 1 << 7;
pub const CTRL3_C_BDU: u8 = 1 << 6;
pub const CTRL3_C_H_LACTIVE: u8 = 1 << 5;
pub const CTRL3_C_PP_OD: u8 = 1 << 4;
pub const CTRL3_C_SIM: u8 = 1 << 3;
pub const CTRL3_C_IF_INC: u8 = 1 << 2;
pub const CTRL3_C_SW_RESET: u8 = 1;

// =============================================================================
// CTRL7_G (0x16)
// =============================================================================

pub const CTRL7_G_G_HM_MODE: u8 = 1 << 7;
pub const CTRL7_G_HP_EN_G: u8 = 1 << 6;
pub const CTRL7_G_HPM_G_MASK: u8 = 0b0001_1000;
pub const CTRL7_G_HPM_G_SHIFT: u8 = 3;
pub const CTRL7_G_OIS_ON_EN: u8 = 1 << 2;
pub const CTRL7_G_USR_OFF_ON_OUT: u8 = 1 << 1;
pub const CTRL7_G_OIS_ON: u8 = 1;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Format)]
#[repr(u8)]
pub enum HpmG {
    Hpmg16 = 0b00,  // ±250 mHz
    Hpmg65 = 0b01,  // ±500 mHz
    Hpmg260 = 0b10, // ±1000 mHz
    Hpmg104 = 0b11, // ±4000 Hz
}

// =============================================================================
// CTRL9_XL (0x18)
// =============================================================================

pub const CTRL9_XL_DEN_Z: u8 = 1 << 7;
pub const CTRL9_XL_DEN_Y: u8 = 1 << 6;
pub const CTRL9_XL_DEN_X: u8 = 1 << 5;
pub const CTRL9_XL_DEN_XL_G: u8 = 1 << 4;
pub const CTRL9_XL_DEN_XL_EN: u8 = 1 << 3;
pub const CTRL9_XL_DEN_LH: u8 = 1 << 2;
pub const CTRL9_XL_DEVICE_CONF: u8 = 1 << 1;

// =============================================================================
// FIFO_CTRL (0x07 - 0x0A)
// =============================================================================

// FIFO_CTRL2 (0x08)
pub const FIFO_CTRL2_WTM_MASK: u8 = 0b0000_0001; 
// Bits 7 always 0? No, register map says WTM[8] is in CTRL2. 
// Ah, `fifoctrl.rs` says bit 6 is compression.
pub const FIFO_CTRL2_STOP_ON_WTM: u8 = 1 << 7;
pub const FIFO_CTRL2_FIFO_COMPR_RT_EN: u8 = 1 << 6;

// FIFO_CTRL3 (0x09) - BDR
pub const FIFO_CTRL3_BDR_GY_MASK: u8 = 0b1111_0000;
pub const FIFO_CTRL3_BDR_GY_SHIFT: u8 = 4;
pub const FIFO_CTRL3_BDR_XL_MASK: u8 = 0b0000_1111;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Format)]
#[repr(u8)]
pub enum BdrGy {
    Off = 0b0000,
    Hz6_5 = 0b1011,
    Hz12_5 = 0b0001,
    Hz26 = 0b0010,
    Hz52 = 0b0011,
    Hz104 = 0b0100,
    Hz208 = 0b0101,
    Hz417 = 0b0110,
    Hz833 = 0b0111,
    Hz1667 = 0b1000,
    Hz3333 = 0b1001,
    Hz6667 = 0b1010,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Format)]
#[repr(u8)]
pub enum BdrXl {
    Off = 0b0000,
    Hz6_5 = 0b1011,
    Hz12_5 = 0b0001,
    Hz26 = 0b0010,
    Hz52 = 0b0011,
    Hz104 = 0b0100,
    Hz208 = 0b0101,
    Hz417 = 0b0110,
    Hz833 = 0b0111,
    Hz1667 = 0b1000,
    Hz3333 = 0b1001,
    Hz6667 = 0b1010,
}

// FIFO_CTRL4 (0x0A) - Mode
pub const FIFO_CTRL4_FIFO_MODE_MASK: u8 = 0b0000_0111;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Format)]
#[repr(u8)]
pub enum FifoMode {
    Bypass = 0b000,
    FifoMode = 0b001,
    ContinuousToFifo = 0b011,
    BypassToContinuous = 0b100,
    Continuous = 0b110,
    BypassToFifo = 0b111,
}
