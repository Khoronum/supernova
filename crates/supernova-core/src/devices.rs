//! Known HATOR / Compx USB VID/PIDs and MID device profiles from `Config.ini`.

use crate::sensor::SensorKind;
use serde::{Deserialize, Serialize};

/// USB Vendor ID shared by HATOR MouseHUB / Compx devices.
pub const HATOR_VID: u16 = 0x379A;

/// Preferred vendor HID interface for configuration commands (`Usage Page 0xFF02`, `Usage 0x02`).
pub const VENDOR_USAGE_PAGE: u16 = 0xFF02;
pub const VENDOR_USAGE: u16 = 0x02;

/// Fallback USB interface number when OS/hidraw doesn't report usage page.
pub const FALLBACK_INTERFACE_NUMBER: i32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionKind {
    Wired,
    Wireless24G,
    Wireless8K,
    Unknown,
}

impl ConnectionKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Wired => "Wired USB",
            Self::Wireless24G => "2.4 GHz Wireless",
            Self::Wireless8K => "8K Wireless Dongle",
            Self::Unknown => "USB HID",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceProfile {
    pub mid: u8,
    pub model_name: &'static str,
    pub sensor: SensorKind,
    pub max_dpi_stages: u8,
}

/// Default 8 DPI stage values from `Config.ini` (`DPIGrade`).
pub const DEFAULT_DPI_GRADES: [u32; 8] = [400, 800, 1000, 1200, 1600, 3200, 6000, 8000];

/// Default 8 DPI stage RGB colors from `Config.ini` (`DPIColor`).
pub const DEFAULT_DPI_COLORS: [[u8; 3]; 8] = [
    [255, 255, 0],   // Yellow
    [0, 0, 255],     // Blue
    [255, 0, 255],   // Purple / Magenta
    [0, 255, 0],     // Green
    [255, 0, 0],     // Red
    [0, 255, 255],   // Cyan / Light Blue
    [0, 255, 255],   // Cyan / Light Yellow slot
    [255, 255, 255], // White
];

/// Classify a known HATOR PID from `Config.ini` into its connection mode.
#[must_use]
pub const fn classify_pid(pid: u16) -> ConnectionKind {
    match pid {
        0x0CEF | 0x0CEB | 0x3607 | 0x3318 | 0x3319 | 0x3307 | 0x3321 | 0x3330 | 0x3327 => {
            ConnectionKind::Wired
        }
        0x0CF0 | 0x3608 | 0x3308 | 0x3322 | 0x3331 | 0x3328 => ConnectionKind::Wireless24G,
        0x0CEC => ConnectionKind::Wireless8K,
        _ => ConnectionKind::Unknown,
    }
}

/// Friendly fallback display name when the USB descriptor only says generic Compx or PID.
#[must_use]
pub fn friendly_product_name(pid: u16, raw_product: &str) -> String {
    let trimmed = raw_product.trim();
    if !trimmed.is_empty() {
        if trimmed.eq_ignore_ascii_case("Compx Quasar 3WL") {
            return "HATOR Quasar 3 Wireless".to_string();
        }
        return trimmed.to_string();
    }
    match pid {
        0x0CEF | 0x0CF0 | 0x0CEC => "HATOR Quasar 3 Series".to_string(),
        0x0CEB => "HATOR Quasar 3 Wired".to_string(),
        _ => format!("HATOR Mouse (0x{pid:04X})"),
    }
}

/// Resolve sensor & capability profile from the device's MID (`Config.ini` `[Drive1..10]`).
///
/// `Config.ini` stores `MID` values as base-10 decimal integers (`07`, `08`, `09`,
/// `11`, `12`, `16`, `17`, `18`, `22`, `23`) parsed via `QString::toInt(..., 10)` and
/// compared directly against hardware `info.mid`. Unknown MIDs fall back safely to PAW3950.
#[must_use]
pub const fn profile_for_mid(mid: u8) -> DeviceProfile {
    match mid {
        7 => DeviceProfile {
            mid,
            model_name: "HATOR Mouse (PAW3311 / 12K)",
            sensor: SensorKind::Paw3311 {
                max_dpi: 12_000,
                ext_flag: 0x22,
            },
            max_dpi_stages: 6,
        },
        11 | 12 => DeviceProfile {
            mid,
            model_name: "HATOR Mouse (PAW3311 / 24K)",
            sensor: SensorKind::Paw3311 {
                max_dpi: 24_000,
                ext_flag: 0x33,
            },
            max_dpi_stages: 6,
        },
        8 | 9 | 16 | 17 | 18 | 22 | 23 => DeviceProfile {
            mid,
            model_name: "HATOR Mouse (PAW3950)",
            sensor: SensorKind::Paw3950,
            max_dpi_stages: 6,
        },
        _ => DeviceProfile {
            mid,
            model_name: "HATOR Mouse (PAW3950 compatible)",
            sensor: SensorKind::Paw3950,
            max_dpi_stages: 6,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mid_profiles_match_config_ini() {
        assert_eq!(
            profile_for_mid(7).sensor,
            SensorKind::Paw3311 {
                max_dpi: 12_000,
                ext_flag: 0x22,
            }
        );
        for mid in [11, 12] {
            assert_eq!(
                profile_for_mid(mid).sensor,
                SensorKind::Paw3311 {
                    max_dpi: 24_000,
                    ext_flag: 0x33,
                }
            );
        }
        for mid in [8, 9, 16, 17, 18, 22, 23] {
            assert_eq!(profile_for_mid(mid).sensor, SensorKind::Paw3950);
        }
    }
}
