//! Known HATOR / Compx USB VID/PIDs, MID profiles, and hardware capability limits.

use crate::config::{LodLevel, PollingRate};
use crate::sensor::{SensorKind, SensorSpec};
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

    /// Maximum polling rate permitted by the USB PID transport class alone.
    ///
    /// Standard 2.4 GHz dongles (`Wireless_PID`) cap at 1000 Hz; wired PIDs may be
    /// 1K or 8K depending on firmware `device_type` and mouse model.
    #[must_use]
    pub const fn max_polling_rate(self) -> PollingRate {
        match self {
            Self::Wireless24G => PollingRate::Hz1000,
            Self::Wired | Self::Wireless8K | Self::Unknown => PollingRate::Hz8000,
        }
    }
}

/// Authoritative link/receiver tier reported by firmware `ReadDeviceInfo` (`resp[12]`).
///
/// Reverse-engineered from `HATOR MouseHUB.exe` (`0x14000c656`, `0x1400122e7`):
/// `0` = Wireless 1K, `1` = Wireless 4K, `2` = Wired 1K, `3` = Wired 8K,
/// `4` = Wireless 2K, `5` = Wireless 8K.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkType {
    Wireless1K,
    Wireless4K,
    Wired1K,
    Wired8K,
    Wireless2K,
    Wireless8K,
    Unknown(u8),
}

impl LinkType {
    #[must_use]
    pub const fn from_device_type(device_type: u8) -> Self {
        match device_type {
            0 => Self::Wireless1K,
            1 => Self::Wireless4K,
            2 => Self::Wired1K,
            3 => Self::Wired8K,
            4 => Self::Wireless2K,
            5 => Self::Wireless8K,
            other => Self::Unknown(other),
        }
    }

    #[must_use]
    pub const fn max_polling_rate(self) -> PollingRate {
        match self {
            Self::Wireless1K | Self::Wired1K | Self::Unknown(_) => PollingRate::Hz1000,
            Self::Wireless2K => PollingRate::Hz2000,
            Self::Wireless4K => PollingRate::Hz4000,
            Self::Wired8K | Self::Wireless8K => PollingRate::Hz8000,
        }
    }

    #[must_use]
    pub const fn is_wired(self) -> bool {
        matches!(self, Self::Wired1K | Self::Wired8K)
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Wireless1K => "2.4 GHz Wireless (1K)",
            Self::Wireless2K => "2.4 GHz Wireless (2K)",
            Self::Wireless4K => "2.4 GHz Wireless (4K)",
            Self::Wireless8K => "2.4 GHz Wireless (8K)",
            Self::Wired1K => "Wired USB (1K)",
            Self::Wired8K => "Wired USB (8K)",
            Self::Unknown(_) => "USB HID (1K)",
        }
    }
}

/// Official HATOR mouse specifications cross-referenced from `hator.com`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelSpec {
    pub name: &'static str,
    pub skus: &'static [&'static str],
    pub sensor_name: &'static str,
    pub max_dpi: u32,
    pub max_dpi_stages: u8,
    pub wired_max_polling_rate: PollingRate,
    pub wireless_max_polling_rate: PollingRate,
}

/// Known HATOR mice from `hator.com/collections/mice`.
pub const KNOWN_HATOR_MODELS: &[ModelSpec] = &[
    ModelSpec {
        name: "HATOR Quasar 3 Wireless",
        skus: &["HTM730", "HTM731"],
        sensor_name: "PixArt PMW3311",
        max_dpi: 12_000,
        max_dpi_stages: 6,
        wired_max_polling_rate: PollingRate::Hz1000,
        wireless_max_polling_rate: PollingRate::Hz1000,
    },
    ModelSpec {
        name: "HATOR Quasar 3 Pro Wireless",
        skus: &["HTM740"],
        sensor_name: "PixArt PAW3950",
        max_dpi: 30_000,
        max_dpi_stages: 6,
        wired_max_polling_rate: PollingRate::Hz1000,
        wireless_max_polling_rate: PollingRate::Hz1000,
    },
    ModelSpec {
        name: "HATOR Quasar 3S Pro Wireless",
        skus: &["HTM745"],
        sensor_name: "PixArt PAW3950",
        max_dpi: 30_000,
        max_dpi_stages: 6,
        wired_max_polling_rate: PollingRate::Hz1000,
        wireless_max_polling_rate: PollingRate::Hz1000,
    },
    ModelSpec {
        name: "HATOR Quasar 3 Ultima 8K Wireless",
        skus: &["HTM760", "HTM761", "HTM762"],
        sensor_name: "PixArt PAW3950",
        max_dpi: 30_000,
        max_dpi_stages: 6,
        wired_max_polling_rate: PollingRate::Hz1000,
        wireless_max_polling_rate: PollingRate::Hz8000,
    },
    ModelSpec {
        name: "HATOR Quasar 3S Ultima 8K Wireless",
        skus: &["HTM660", "HTM661"],
        sensor_name: "PixArt PAW3950",
        max_dpi: 30_000,
        max_dpi_stages: 6,
        wired_max_polling_rate: PollingRate::Hz1000,
        wireless_max_polling_rate: PollingRate::Hz8000,
    },
    ModelSpec {
        name: "HATOR Quasar 3 Ultra 8K+8K Wireless",
        skus: &["HTM770", "HTM771"],
        sensor_name: "PixArt PAW3950",
        max_dpi: 30_000,
        max_dpi_stages: 6,
        wired_max_polling_rate: PollingRate::Hz8000,
        wireless_max_polling_rate: PollingRate::Hz8000,
    },
    ModelSpec {
        name: "HATOR Pulsar 4 Wireless",
        skus: &["HTM830"],
        sensor_name: "PixArt PAW3315",
        max_dpi: 16_000,
        max_dpi_stages: 6,
        wired_max_polling_rate: PollingRate::Hz1000,
        wireless_max_polling_rate: PollingRate::Hz1000,
    },
];

/// Match a connected device against known `hator.com` product specifications.
///
/// Care is taken not to match generic USB product strings (such as `"Compx Quasar 3WL"`
/// mapped to `"HATOR Quasar 3 Wireless"`) to the 1K PAW3311 Quasar 3 base model when the
/// hardware actually reports a PAW3950 sensor.
#[must_use]
pub fn model_spec_for(
    product_name: &str,
    mid: u8,
    sensor: SensorKind,
) -> Option<&'static ModelSpec> {
    let lower = product_name.to_ascii_lowercase();
    if lower.contains("quasar 3 ultra") {
        return Some(&KNOWN_HATOR_MODELS[5]);
    }
    if lower.contains("quasar 3s ultima") {
        return Some(&KNOWN_HATOR_MODELS[4]);
    }
    if lower.contains("quasar 3 ultima") {
        return Some(&KNOWN_HATOR_MODELS[3]);
    }
    if lower.contains("quasar 3s pro") {
        return Some(&KNOWN_HATOR_MODELS[2]);
    }
    if lower.contains("quasar 3 pro") {
        return Some(&KNOWN_HATOR_MODELS[1]);
    }
    if lower.contains("pulsar 4") {
        return Some(&KNOWN_HATOR_MODELS[6]);
    }
    if mid == 7
        || (matches!(
            sensor,
            SensorKind::Paw3311 {
                max_dpi: 12_000,
                ..
            }
        ) && lower.contains("quasar 3"))
    {
        return Some(&KNOWN_HATOR_MODELS[0]);
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceProfile {
    pub mid: u8,
    pub model_name: &'static str,
    pub sensor: SensorKind,
    pub max_dpi_stages: u8,
}

/// Effective hardware, firmware link, and sensor capabilities for the active device connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceCapabilities {
    pub profile: DeviceProfile,
    pub sensor: SensorKind,
    pub link: LinkType,
    pub connection: ConnectionKind,
    pub model: Option<&'static ModelSpec>,
    pub max_polling_rate: PollingRate,
    pub max_dpi_stages: u8,
    pub max_dpi: u32,
}

impl DeviceCapabilities {
    #[must_use]
    pub fn resolve(
        profile: DeviceProfile,
        sensor: SensorKind,
        device_type: u8,
        connection: ConnectionKind,
        product_name: &str,
    ) -> Self {
        let link = LinkType::from_device_type(device_type);
        let model = model_spec_for(product_name, profile.mid, sensor);
        let sensor_spec = sensor.spec();

        let is_wired = link.is_wired()
            || (matches!(link, LinkType::Unknown(_)) && connection == ConnectionKind::Wired);

        let mut max_polling_rate = link
            .max_polling_rate()
            .min_rate(connection.max_polling_rate())
            .min_rate(sensor_spec.max_polling_rate);

        if let Some(m) = model {
            let model_rate = if is_wired {
                m.wired_max_polling_rate
            } else {
                m.wireless_max_polling_rate
            };
            max_polling_rate = max_polling_rate.min_rate(model_rate);
        }

        let mut max_dpi_stages = profile.max_dpi_stages.min(8);
        let mut max_dpi = sensor_spec.max_dpi;
        if let Some(m) = model {
            max_dpi_stages = max_dpi_stages.min(m.max_dpi_stages.min(8));
            max_dpi = max_dpi.min(m.max_dpi);
        }

        Self {
            profile,
            sensor,
            link,
            connection,
            model,
            max_polling_rate,
            max_dpi_stages,
            max_dpi,
        }
    }

    #[must_use]
    pub const fn sensor_spec(&self) -> SensorSpec {
        self.sensor.spec()
    }

    #[must_use]
    pub const fn supported_polling_rates(&self) -> &'static [PollingRate] {
        PollingRate::rates_up_to(self.max_polling_rate)
    }

    #[must_use]
    pub const fn supports_polling_rate(&self, rate: PollingRate) -> bool {
        rate.hz() <= self.max_polling_rate.hz()
    }

    #[must_use]
    pub const fn supports_lod(&self) -> bool {
        self.sensor.spec().supports_lod
    }

    #[must_use]
    pub const fn supports_lod_level(&self, lod: LodLevel) -> bool {
        self.sensor.spec().supports_lod_level(lod)
    }

    #[must_use]
    pub const fn supports_independent_xy(&self) -> bool {
        self.sensor.spec().supports_independent_xy
    }

    #[must_use]
    pub const fn supports_motion_sync(&self) -> bool {
        self.sensor.spec().supports_motion_sync
    }

    #[must_use]
    pub const fn supports_ripple_control(&self) -> bool {
        self.sensor.spec().supports_ripple_control
    }

    #[must_use]
    pub const fn supports_angle_snapping(&self) -> bool {
        self.sensor.spec().supports_angle_snapping
    }

    #[must_use]
    pub const fn supports_sensor_mode(&self) -> bool {
        self.sensor.spec().supports_sensor_mode
    }

    #[must_use]
    pub const fn connection_label(&self) -> &'static str {
        match self.link {
            LinkType::Unknown(_) => self.connection.label(),
            link => link.label(),
        }
    }

    #[must_use]
    pub fn polling_limit_note(&self) -> String {
        format!(
            "Max {} Hz · {}",
            self.max_polling_rate.hz(),
            self.connection_label()
        )
    }
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
            model_name: "HATOR Quasar 3 Wireless (PAW3311 / 12K)",
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

    #[test]
    fn capabilities_match_hator_com_lineup_and_device_type() {
        // Quasar 3 Wireless (MID=7, PMW3311, 12K, 1K wired & wireless; even bogus dt=5 stays 1K)
        let q3_base = DeviceCapabilities::resolve(
            profile_for_mid(7),
            profile_for_mid(7).sensor,
            5,
            ConnectionKind::Wireless24G,
            "HATOR Quasar 3 Wireless",
        );
        assert_eq!(q3_base.max_polling_rate, PollingRate::Hz1000);
        assert_eq!(
            q3_base.supported_polling_rates(),
            &[
                PollingRate::Hz125,
                PollingRate::Hz250,
                PollingRate::Hz500,
                PollingRate::Hz1000,
            ]
        );
        assert_eq!(q3_base.max_dpi, 12_000);
        assert_eq!(q3_base.max_dpi_stages, 6);
        assert!(!q3_base.supports_lod());
        assert!(!q3_base.supports_motion_sync());
        assert!(!q3_base.supports_independent_xy());
        assert!(q3_base.supports_ripple_control());
        assert!(q3_base.supports_angle_snapping());
        assert!(q3_base.supports_sensor_mode());

        // Quasar 3 Pro / 3S Pro Wireless (PAW3950, 1K max wired & wireless)
        let q3_pro = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            0,
            ConnectionKind::Wireless24G,
            "HATOR Quasar 3 Pro Wireless",
        );
        assert_eq!(q3_pro.max_polling_rate, PollingRate::Hz1000);
        assert!(q3_pro.supports_lod());
        assert!(q3_pro.supports_lod_level(LodLevel::Mm07));
        assert!(q3_pro.supports_motion_sync());
        assert!(q3_pro.supports_independent_xy());

        let q3s_pro_wired = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            2,
            ConnectionKind::Wired,
            "HATOR Quasar 3S Pro Wireless",
        );
        assert_eq!(q3s_pro_wired.max_polling_rate, PollingRate::Hz1000);

        // Quasar 3 Ultima 8K / 3S Ultima 8K (Wired 1K, Wireless 8K)
        let ultima_wired = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            3,
            ConnectionKind::Wired,
            "HATOR Quasar 3 Ultima 8K Wireless",
        );
        assert_eq!(ultima_wired.max_polling_rate, PollingRate::Hz1000);

        let ultima_wl = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            5,
            ConnectionKind::Wireless8K,
            "HATOR Quasar 3S Ultima 8K Wireless",
        );
        assert_eq!(ultima_wl.max_polling_rate, PollingRate::Hz8000);

        // Quasar 3 Ultra 8K+8K (Wired 8K, Wireless 8K)
        let ultra_wired = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            3,
            ConnectionKind::Wired,
            "HATOR Quasar 3 Ultra 8K+8K Wireless",
        );
        assert_eq!(ultra_wired.max_polling_rate, PollingRate::Hz8000);

        let ultra_wl = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            5,
            ConnectionKind::Wireless8K,
            "HATOR Quasar 3 Ultra 8K+8K Wireless",
        );
        assert_eq!(ultra_wl.max_polling_rate, PollingRate::Hz8000);
    }

    #[test]
    fn generic_compx_name_with_paw3950_does_not_false_cap_to_1k() {
        let generic_name = friendly_product_name(0x0CEC, "Compx Quasar 3WL");
        assert_eq!(generic_name, "HATOR Quasar 3 Wireless");
        let caps = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            5,
            ConnectionKind::Wireless8K,
            &generic_name,
        );
        assert!(caps.model.is_none());
        assert_eq!(caps.max_polling_rate, PollingRate::Hz8000);
    }

    #[test]
    fn unknown_and_intermediate_link_types() {
        let link_2k = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            4,
            ConnectionKind::Unknown,
            "HATOR Mouse",
        );
        assert_eq!(link_2k.max_polling_rate, PollingRate::Hz2000);

        let link_4k = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            1,
            ConnectionKind::Unknown,
            "HATOR Mouse",
        );
        assert_eq!(link_4k.max_polling_rate, PollingRate::Hz4000);

        let link_unknown = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            0xFF,
            ConnectionKind::Unknown,
            "HATOR Mouse",
        );
        assert_eq!(link_unknown.max_polling_rate, PollingRate::Hz1000);
    }
}
