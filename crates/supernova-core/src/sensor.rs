//! Optical sensor DPI encoding, decoding, and stepping rules.
//!
//! Reverse-engineered from `HATOR.exe` (`0x40e120..0x40ee72`) and `driver_sensor.h`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorKind {
    Paw3950,
    Paw3395,
    Paw3311 { max_dpi: u32, ext_flag: u8 },
    Paw3335,
    Paw3325,
    Paw3370,
    GenericLinear,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SensorSpec {
    pub kind: SensorKind,
    pub name: &'static str,
    pub min_dpi: u32,
    pub max_dpi: u32,
    pub base_step: u32,
    pub supports_independent_xy: bool,
    pub supports_lod_07mm: bool,
}

/// PAW3311 register values for `50..=10000` DPI in 50-DPI steps (200 entries).
/// Extracted verbatim from `HATOR MouseHUB/Sys32/driver_sensor.h`.
#[rustfmt::skip]
const TABLE_3311: [u8; 200] = [
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x08, 0x09, 0x0A, 0x0B,
    0x0C, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20, 0x22, 0x23,
    0x24, 0x25, 0x26, 0x27, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2F,
    0x30, 0x31, 0x32, 0x33, 0x34, 0x36, 0x37, 0x38, 0x39, 0x3A,
    0x3B, 0x3D, 0x3E, 0x3F, 0x40, 0x41, 0x43, 0x44, 0x45, 0x46,
    0x47, 0x48, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x4F, 0x51, 0x52,
    0x53, 0x54, 0x55, 0x57, 0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5E,
    0x5F, 0x60, 0x61, 0x62, 0x63, 0x65, 0x66, 0x67, 0x68, 0x69,
    0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x72, 0x73, 0x74, 0x75,
    0x76, 0x77, 0x79, 0x7A, 0x7B, 0x7C, 0x7D, 0x7F, 0x80, 0x81,
    0x82, 0x83, 0x84, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B, 0x8D,
    0x8E, 0x8F, 0x90, 0x91, 0x93, 0x94, 0x95, 0x96, 0x97, 0x98,
    0x9A, 0x9B, 0x9C, 0x9D, 0x9E, 0x9F, 0xA1, 0xA2, 0xA3, 0xA4,
    0xA5, 0xA7, 0xA8, 0xA9, 0xAA, 0xAB, 0xAC, 0xAE, 0xAF, 0xB0,
    0xB1, 0xB2, 0xB3, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBB, 0xBC,
    0xBD, 0xBE, 0xBF, 0xC0, 0xC2, 0xC3, 0xC4, 0xC5, 0xC6, 0xC7,
    0xC9, 0xCA, 0xCB, 0xCC, 0xCD, 0xCF, 0xD0, 0xD1, 0xD2, 0xD3,
    0xD4, 0xD6, 0xD7, 0xD8, 0xD9, 0xDA, 0xDB, 0xDD, 0xDE, 0xDF,
    0xE0, 0xE1, 0xE3, 0xE4, 0xE5, 0xE6, 0xE7, 0xE8, 0xEA, 0xEB,
];

/// PAW3335 register values for `100..=16000` DPI in 100-DPI steps (160 entries).
#[rustfmt::skip]
const TABLE_3335: [u8; 160] = [
    0x00, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x09, 0x0A, 0x0B,
    0x0C, 0x0D, 0x0E, 0x10, 0x11, 0x12, 0x13, 0x14, 0x16, 0x17,
    0x18, 0x19, 0x1A, 0x1B, 0x1D, 0x1E, 0x1F, 0x20, 0x21, 0x23,
    0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x2C, 0x2D, 0x2E, 0x2F,
    0x30, 0x32, 0x33, 0x34, 0x35, 0x36, 0x38, 0x39, 0x3A, 0x3B,
    0x3C, 0x3E, 0x3F, 0x40, 0x41, 0x42, 0x44, 0x45, 0x46, 0x47,
    0x48, 0x4A, 0x4B, 0x4C, 0x4D, 0x4E, 0x50, 0x51, 0x52, 0x53,
    0x54, 0x56, 0x57, 0x58, 0x59, 0x5A, 0x5C, 0x5D, 0x5E, 0x5F,
    0x60, 0x62, 0x63, 0x64, 0x65, 0x66, 0x68, 0x69, 0x6A, 0x6B,
    0x6C, 0x6E, 0x6F, 0x70, 0x71, 0x72, 0x74, 0x75, 0x76, 0x77,
    0x78, 0x7A, 0x7B, 0x7C, 0x7D, 0x7E, 0x80, 0x81, 0x82, 0x83,
    0x84, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8C, 0x8D, 0x8E, 0x8F,
    0x90, 0x92, 0x93, 0x94, 0x95, 0x96, 0x98, 0x99, 0x9A, 0x9B,
    0x9C, 0x9E, 0x9F, 0xA0, 0xA1, 0xA2, 0xA4, 0xA5, 0xA6, 0xA7,
    0xA8, 0xAA, 0xAB, 0xAC, 0xAD, 0xAE, 0xB0, 0xB1, 0xB2, 0xB3,
    0xB4, 0xB5, 0xB6, 0xB7, 0xB8, 0xB9, 0xBA, 0xBB, 0xBC, 0xBD,
];

/// PAW3325 register values for `200..=5000` DPI in 100-DPI steps (49 entries).
#[rustfmt::skip]
const TABLE_3325: [u8; 49] = [
    0x04, 0x06, 0x08, 0x0B, 0x0D, 0x0F, 0x12, 0x14, 0x16, 0x19,
    0x1B, 0x1D, 0x20, 0x22, 0x24, 0x27, 0x29, 0x2B, 0x2E, 0x30,
    0x32, 0x34, 0x37, 0x39, 0x3B, 0x3E, 0x40, 0x42, 0x45, 0x47,
    0x49, 0x4C, 0x4E, 0x50, 0x53, 0x55, 0x57, 0x5A, 0x5C, 0x5E,
    0x61, 0x63, 0x65, 0x68, 0x6A, 0x6C, 0x6F, 0x71, 0x73,
];

impl SensorKind {
    #[must_use]
    pub const fn spec(self) -> SensorSpec {
        match self {
            Self::Paw3950 => SensorSpec {
                kind: self,
                name: "PAW3950",
                min_dpi: 50,
                max_dpi: 30_000,
                base_step: 50,
                supports_independent_xy: true,
                supports_lod_07mm: true,
            },
            Self::Paw3395 => SensorSpec {
                kind: self,
                name: "PAW3395",
                min_dpi: 50,
                max_dpi: 26_000,
                base_step: 50,
                supports_independent_xy: true,
                supports_lod_07mm: true,
            },
            Self::Paw3311 { max_dpi, .. } => SensorSpec {
                kind: self,
                name: "PAW3311",
                min_dpi: 50,
                max_dpi,
                base_step: 50,
                supports_independent_xy: false,
                supports_lod_07mm: false,
            },
            Self::Paw3335 => SensorSpec {
                kind: self,
                name: "PAW3335",
                min_dpi: 100,
                max_dpi: 16_000,
                base_step: 100,
                supports_independent_xy: false,
                supports_lod_07mm: false,
            },
            Self::Paw3325 => SensorSpec {
                kind: self,
                name: "PAW3325",
                min_dpi: 200,
                max_dpi: 5_000,
                base_step: 100,
                supports_independent_xy: false,
                supports_lod_07mm: false,
            },
            Self::Paw3370 => SensorSpec {
                kind: self,
                name: "PAW3370",
                min_dpi: 50,
                max_dpi: 19_000,
                base_step: 50,
                supports_independent_xy: true,
                supports_lod_07mm: false,
            },
            Self::GenericLinear => SensorSpec {
                kind: self,
                name: "Generic 50-DPI",
                min_dpi: 50,
                max_dpi: 30_000,
                base_step: 50,
                supports_independent_xy: true,
                supports_lod_07mm: true,
            },
        }
    }

    /// Valid DPI step size at a given DPI level.
    #[must_use]
    pub const fn step_for(self, dpi: u32) -> u32 {
        match self {
            Self::Paw3311 { .. } => {
                if dpi > 20_000 {
                    200
                } else if dpi > 10_000 {
                    100
                } else {
                    50
                }
            }
            Self::Paw3335 | Self::Paw3325 => 100,
            Self::Paw3950 | Self::Paw3395 | Self::Paw3370 | Self::GenericLinear => 50,
        }
    }

    /// Clamp and snap an arbitrary DPI value to the nearest valid sensor setting.
    #[must_use]
    pub fn snap_dpi(self, dpi: u32) -> u32 {
        let spec = self.spec();
        let clamped = dpi.clamp(spec.min_dpi, spec.max_dpi);
        let step = self.step_for(clamped);
        let snapped = ((clamped + step / 2) / step) * step;
        snapped.clamp(spec.min_dpi, spec.max_dpi)
    }

    /// Encode `(dpi_x, dpi_y)` into the 3-byte flash representation `[b0, b1, b2]`.
    #[must_use]
    pub fn encode_dpi(self, dpi_x: u32, dpi_y: u32) -> [u8; 3] {
        let spec = self.spec();
        let x = self.snap_dpi(dpi_x);
        let y = if spec.supports_independent_xy {
            self.snap_dpi(dpi_y)
        } else {
            x
        };

        match self {
            Self::Paw3950 | Self::Paw3395 | Self::Paw3370 | Self::GenericLinear => {
                let rx = (x / 50).saturating_sub(1);
                let ry = (y / 50).saturating_sub(1);
                let b0 = (rx & 0xFF) as u8;
                let b1 = (ry & 0xFF) as u8;
                let hi_x = ((rx >> 8) & 0x03) as u8;
                let hi_y = ((ry >> 8) & 0x03) as u8;
                let b2 = (hi_x << 6) | (hi_y << 2);
                [b0, b1, b2]
            }
            Self::Paw3311 { ext_flag, .. } => {
                let (base_dpi, b2) = if x <= 10_000 {
                    (x, 0x00u8)
                } else if x <= 12_000 {
                    (x / 2, 0x22u8)
                } else if x <= 20_000 && ext_flag == 0x33 {
                    (x / 2, 0x11u8)
                } else if ext_flag == 0x33 {
                    (x / 4, 0x33u8)
                } else {
                    (x / 2, 0x22u8)
                };
                let idx = ((base_dpi / 50).saturating_sub(1) as usize).min(TABLE_3311.len() - 1);
                let reg = TABLE_3311[idx];
                [reg, reg, b2]
            }
            Self::Paw3335 => {
                let idx = ((x / 100).saturating_sub(1) as usize).min(TABLE_3335.len() - 1);
                let reg = TABLE_3335[idx];
                [reg, reg, 0x00]
            }
            Self::Paw3325 => {
                let idx = ((x / 100).saturating_sub(2) as usize).min(TABLE_3325.len() - 1);
                let reg = TABLE_3325[idx];
                [reg, reg, 0x00]
            }
        }
    }

    /// Decode the 3-byte flash DPI representation `[b0, b1, b2]` into `(dpi_x, dpi_y)`.
    #[must_use]
    pub fn decode_dpi(self, raw: [u8; 3]) -> (u32, u32) {
        let [b0, b1, b2] = raw;
        match self {
            Self::Paw3950 | Self::Paw3395 | Self::Paw3370 | Self::GenericLinear => {
                let rx = u32::from(b0) | (u32::from(b2 & 0xC0) << 2);
                let ry = u32::from(b1) | (u32::from(b2 & 0x0C) << 6);
                let mut x = (rx + 1) * 50;
                let mut y = (ry + 1) * 50;
                // Preserve exact 0x22 / 0x11 multiplier compatibility if encountered
                if (b2 & 0x22) != 0 && (b2 & 0xC0) == 0 && (b2 & 0x0C) == 0 {
                    x *= 2;
                    y *= 2;
                }
                if (b2 & 0x11) != 0 && (b2 & 0xC0) == 0 && (b2 & 0x0C) == 0 {
                    x *= 2;
                    y *= 2;
                }
                (x, y)
            }
            Self::Paw3311 { .. } => {
                let base_x = lookup_3311_dpi(b0);
                let base_y = lookup_3311_dpi(b1);
                let mult = match (b2 & 0x22 != 0, b2 & 0x11 != 0) {
                    (false, false) => 1,
                    (true, false) | (false, true) => 2,
                    (true, true) => 4,
                };
                (base_x * mult, base_y * mult)
            }
            Self::Paw3335 => {
                let x = lookup_table_dpi(&TABLE_3335, b0, 100, 100);
                let y = lookup_table_dpi(&TABLE_3335, b1, 100, 100);
                (x, y)
            }
            Self::Paw3325 => {
                let x = lookup_table_dpi(&TABLE_3325, b0, 200, 100);
                let y = lookup_table_dpi(&TABLE_3325, b1, 200, 100);
                (x, y)
            }
        }
    }
}

fn lookup_3311_dpi(reg: u8) -> u32 {
    if let Some(pos) = TABLE_3311.iter().position(|&v| v == reg) {
        return ((pos as u32) + 1) * 50;
    }
    // Closest match fallback if firmware stored an intermediate register value
    let best_idx = TABLE_3311
        .iter()
        .enumerate()
        .min_by_key(|(_, v)| v.abs_diff(reg))
        .map_or(0, |(idx, _)| idx);
    ((best_idx as u32) + 1) * 50
}

fn lookup_table_dpi(table: &[u8], reg: u8, start_dpi: u32, step: u32) -> u32 {
    if let Some(pos) = table.iter().position(|&v| v == reg) {
        return start_dpi + (pos as u32) * step;
    }
    let best_idx = table
        .iter()
        .enumerate()
        .min_by_key(|(_, v)| v.abs_diff(reg))
        .map_or(0, |(idx, _)| idx);
    start_dpi + (best_idx as u32) * step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paw3950_round_trip_symmetric_and_xy() {
        let sensor = SensorKind::Paw3950;
        for dpi in [
            50, 400, 800, 1600, 3200, 12_800, 12_850, 16_000, 26_000, 30_000,
        ] {
            let raw = sensor.encode_dpi(dpi, dpi);
            assert_eq!(sensor.decode_dpi(raw), (dpi, dpi), "dpi={dpi}");
        }

        // Known encoding check: 12850 -> rx = 256 (0x100) -> b0=0, b1=0, b2=(1<<6)|(1<<2)=0x44
        assert_eq!(sensor.encode_dpi(12_850, 12_850), [0x00, 0x00, 0x44]);
        // Asymmetric X/Y across 8-bit boundary
        let raw_xy = sensor.encode_dpi(800, 26_000);
        assert_eq!(sensor.decode_dpi(raw_xy), (800, 26_000));
        let raw_xy2 = sensor.encode_dpi(30_000, 1_600);
        assert_eq!(sensor.decode_dpi(raw_xy2), (30_000, 1_600));
    }

    #[test]
    fn paw3311_round_trip_all_tiers() {
        let sensor = SensorKind::Paw3311 {
            max_dpi: 24_000,
            ext_flag: 0x33,
        };
        let cases = [
            (50, 0x00),
            (400, 0x00),
            (800, 0x00),
            (1600, 0x00),
            (10_000, 0x00),
            (10_100, 0x22),
            (12_000, 0x22),
            (12_200, 0x11),
            (20_000, 0x11),
            (20_200, 0x33),
            (24_000, 0x33),
        ];
        for (dpi, expected_b2) in cases {
            let raw = sensor.encode_dpi(dpi, dpi);
            assert_eq!(raw[2], expected_b2, "dpi={dpi}");
            assert_eq!(sensor.decode_dpi(raw), (dpi, dpi), "dpi={dpi}");
        }
    }

    #[test]
    fn paw3335_round_trip() {
        let sensor = SensorKind::Paw3335;
        for dpi in [100, 400, 800, 1600, 3200, 16_000] {
            let raw = sensor.encode_dpi(dpi, dpi);
            assert_eq!(sensor.decode_dpi(raw), (dpi, dpi));
        }
    }

    #[test]
    fn snap_dpi_clamps_and_rounds() {
        assert_eq!(SensorKind::Paw3950.snap_dpi(10), 50);
        assert_eq!(SensorKind::Paw3950.snap_dpi(826), 850);
        assert_eq!(SensorKind::Paw3950.snap_dpi(50_000), 30_000);
        assert_eq!(
            SensorKind::Paw3311 {
                max_dpi: 24_000,
                ext_flag: 0x33
            }
            .snap_dpi(20_150),
            20_200
        );
    }
}
