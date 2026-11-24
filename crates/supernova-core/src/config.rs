//! Onboard mouse flash configuration layout, parsing, and minimal-diff generation.
//!
//! Reverse-engineered from `HIDUsb.dll` (`MouseConfigParser`, `ProtocolDataParser`,
//! `ProtocolDataCompareUpdate`).

use crate::devices::{DEFAULT_DPI_COLORS, DEFAULT_DPI_GRADES, DeviceCapabilities};
use crate::protocol::{FLASH_TOTAL_LEN, compx_checksum};
use crate::sensor::SensorKind;
use serde::{Deserialize, Serialize};

pub const MIN_DEBOUNCE_MS: u8 = 0;
pub const MAX_DEBOUNCE_MS: u8 = 30;
pub const MIN_SLEEP_MINUTES: u8 = 1;
pub const MAX_SLEEP_MINUTES: u8 = 40;
pub const MIN_LED_SPEED: u8 = 1;
pub const MAX_LED_SPEED: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PollingRate {
    Hz125 = 0x08,
    Hz250 = 0x04,
    Hz500 = 0x02,
    Hz1000 = 0x01,
    Hz2000 = 0x10,
    Hz4000 = 0x20,
    Hz8000 = 0x40,
}

impl PollingRate {
    pub const ALL: &'static [Self] = &[
        Self::Hz125,
        Self::Hz250,
        Self::Hz500,
        Self::Hz1000,
        Self::Hz2000,
        Self::Hz4000,
        Self::Hz8000,
    ];
    pub const RATES_1K: &'static [Self] = &[Self::Hz125, Self::Hz250, Self::Hz500, Self::Hz1000];
    pub const RATES_2K: &'static [Self] = &[
        Self::Hz125,
        Self::Hz250,
        Self::Hz500,
        Self::Hz1000,
        Self::Hz2000,
    ];
    pub const RATES_4K: &'static [Self] = &[
        Self::Hz125,
        Self::Hz250,
        Self::Hz500,
        Self::Hz1000,
        Self::Hz2000,
        Self::Hz4000,
    ];
    pub const RATES_8K: &'static [Self] = Self::ALL;

    #[must_use]
    pub const fn min_rate(self, other: Self) -> Self {
        if self.hz() <= other.hz() { self } else { other }
    }

    #[must_use]
    pub const fn rates_up_to(max: Self) -> &'static [Self] {
        match max {
            Self::Hz125 => &[Self::Hz125],
            Self::Hz250 => &[Self::Hz125, Self::Hz250],
            Self::Hz500 => &[Self::Hz125, Self::Hz250, Self::Hz500],
            Self::Hz1000 => Self::RATES_1K,
            Self::Hz2000 => Self::RATES_2K,
            Self::Hz4000 => Self::RATES_4K,
            Self::Hz8000 => Self::RATES_8K,
        }
    }

    #[must_use]
    pub const fn from_wire(b: u8) -> Self {
        match b {
            0x08 => Self::Hz125,
            0x04 => Self::Hz250,
            0x02 => Self::Hz500,
            0x01 => Self::Hz1000,
            0x10 => Self::Hz2000,
            0x20 => Self::Hz4000,
            0x40 => Self::Hz8000,
            _ => Self::Hz1000,
        }
    }

    #[must_use]
    pub const fn from_hz(hz: u32) -> Option<Self> {
        match hz {
            125 => Some(Self::Hz125),
            250 => Some(Self::Hz250),
            500 => Some(Self::Hz500),
            1000 => Some(Self::Hz1000),
            2000 => Some(Self::Hz2000),
            4000 => Some(Self::Hz4000),
            8000 => Some(Self::Hz8000),
            _ => None,
        }
    }

    #[must_use]
    pub const fn hz(self) -> u32 {
        match self {
            Self::Hz125 => 125,
            Self::Hz250 => 250,
            Self::Hz500 => 500,
            Self::Hz1000 => 1000,
            Self::Hz2000 => 2000,
            Self::Hz4000 => 4000,
            Self::Hz8000 => 8000,
        }
    }
}

impl PartialOrd for PollingRate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PollingRate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hz().cmp(&other.hz())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum LodLevel {
    Mm1 = 0x01,
    Mm2 = 0x02,
    Mm07 = 0x03,
}

impl LodLevel {
    #[must_use]
    pub const fn from_wire(b: u8) -> Self {
        match b {
            0x03 => Self::Mm07,
            0x02 => Self::Mm2,
            _ => Self::Mm1,
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Mm07 => "0.7 mm",
            Self::Mm1 => "1.0 mm",
            Self::Mm2 => "2.0 mm",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum DpiLedMode {
    Off = 0,
    Static = 1,
    Breathing = 2,
}

impl DpiLedMode {
    #[must_use]
    pub const fn from_wire(mode: u8, enabled: u8) -> Self {
        if enabled == 0 || mode == 0 {
            Self::Off
        } else if mode == 1 {
            Self::Static
        } else {
            Self::Breathing
        }
    }

    #[must_use]
    pub const fn wire_pair(self) -> (u8, u8) {
        match self {
            Self::Off => (0, 0),
            Self::Static => (1, 1),
            Self::Breathing => (2, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum SensorMode {
    LowPower = 0,
    HighPerformance = 1,
    Corded = 2,
}

impl SensorMode {
    #[must_use]
    pub const fn from_wire(b: u8) -> Self {
        match b {
            1 => Self::HighPerformance,
            2 => Self::Corded,
            _ => Self::LowPower,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpiStage {
    pub dpi_x: u32,
    pub dpi_y: u32,
    pub color: [u8; 3],
    pub raw_dpi: [u8; 3],
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ConfigValidationError {
    #[error(
        "Polling rate {requested_hz} Hz is not supported on {context} (max {max_hz} Hz; supported: {})",
        format_hz_list(.supported_hz)
    )]
    UnsupportedPollingRate {
        requested_hz: u32,
        max_hz: u32,
        supported_hz: Vec<u32>,
        context: String,
    },
    #[error(
        "DPI stage count {count} exceeds supported maximum of {max_stages} (valid: 1..={max_stages})"
    )]
    InvalidStageCount { count: u8, max_stages: u8 },
    #[error("Active DPI stage {} exceeds enabled stage count {enabled_count}", .stage + 1)]
    InvalidActiveStage { stage: u8, enabled_count: u8 },
    #[error("DPI stage {stage} is out of range (valid: 1..={max_stages})")]
    StageOutOfRange { stage: u8, max_stages: u8 },
    #[error(
        "DPI {dpi} for stage {stage} is outside {sensor_name} supported range ({min_dpi}..={max_dpi})"
    )]
    DpiOutOfRange {
        stage: u8,
        dpi: u32,
        min_dpi: u32,
        max_dpi: u32,
        sensor_name: &'static str,
    },
    #[error("Independent X/Y DPI on stage {stage} is not supported by {sensor_name}")]
    IndependentXyUnsupported {
        stage: u8,
        sensor_name: &'static str,
    },
    #[error("Lift-off distance {} is not supported by {sensor_name}", .lod.label())]
    LodUnsupported {
        lod: LodLevel,
        sensor_name: &'static str,
    },
    #[error("{feature} is not supported by {sensor_name}")]
    FeatureUnsupported {
        feature: &'static str,
        sensor_name: &'static str,
    },
    #[error("Debounce time {ms} ms is out of range (valid: {min_ms}..={max_ms} ms)")]
    DebounceOutOfRange { ms: u8, min_ms: u8, max_ms: u8 },
    #[error(
        "Sleep timeout {} min is out of range (valid: {}..={} min)",
        .units_10s / 6,
        .min_units / 6,
        .max_units / 6
    )]
    SleepTimeoutOutOfRange {
        units_10s: u8,
        min_units: u8,
        max_units: u8,
    },
    #[error("LED speed {speed} is out of range (valid: {min_speed}..={max_speed})")]
    LedSpeedOutOfRange {
        speed: u8,
        min_speed: u8,
        max_speed: u8,
    },
}

fn format_hz_list(rates: &[u32]) -> String {
    rates
        .iter()
        .map(|hz| format!("{hz} Hz"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MouseConfig {
    pub polling_rate: PollingRate,
    pub dpi_stage_count: u8,
    pub active_dpi_stage: u8,
    pub lod: LodLevel,
    pub stages: [DpiStage; 8],
    pub dpi_led_mode: DpiLedMode,
    pub dpi_led_brightness: u8,
    pub dpi_led_speed: u8,
    pub debounce_ms: u8,
    pub motion_sync: bool,
    pub sleep_units_10s: u8,
    pub angle_snapping: bool,
    pub ripple_control: bool,
    pub moving_light_off: bool,
    pub high_performance: bool,
    pub high_perf_timeout_10s: u8,
    pub sensor_mode: SensorMode,
}

impl MouseConfig {
    /// Default configuration matching `HIDUsb.dll` `MouseConfigParser` & `Config.ini`.
    #[must_use]
    pub fn demo_default(sensor: SensorKind) -> Self {
        let stages = std::array::from_fn(|i| {
            let dpi = sensor.snap_dpi(DEFAULT_DPI_GRADES[i]);
            DpiStage {
                dpi_x: dpi,
                dpi_y: dpi,
                color: DEFAULT_DPI_COLORS[i],
                raw_dpi: sensor.encode_dpi(dpi, dpi),
            }
        });

        Self {
            polling_rate: PollingRate::Hz1000,
            dpi_stage_count: 6,
            active_dpi_stage: 2, // 1000 DPI default
            lod: LodLevel::Mm1,
            stages,
            dpi_led_mode: DpiLedMode::Static,
            dpi_led_brightness: 0x80,
            dpi_led_speed: 3,
            debounce_ms: 8,
            motion_sync: sensor.spec().supports_motion_sync,
            sleep_units_10s: 6,
            angle_snapping: false,
            ripple_control: false,
            moving_light_off: false,
            high_performance: false,
            high_perf_timeout_10s: 6,
            sensor_mode: SensorMode::LowPower,
        }
    }

    /// Parse 192-byte onboard flash image (`0x00..0xC0`), verifying each field's Compx checksum
    /// and falling back to HATOR defaults when a field checksum fails (matching `HIDUsb.dll`).
    #[must_use]
    pub fn from_flash(flash: &[u8; FLASH_TOTAL_LEN], sensor: SensorKind) -> Self {
        let mut cfg = Self::demo_default(sensor);

        if let Some(v) = read_u8_checked(flash, 0x00) {
            cfg.polling_rate = PollingRate::from_wire(v);
        }
        if let Some(v) = read_u8_checked(flash, 0x02)
            && (1..=8).contains(&v)
        {
            cfg.dpi_stage_count = v;
        }
        if let Some(v) = read_u8_checked(flash, 0x04) {
            if v < cfg.dpi_stage_count {
                cfg.active_dpi_stage = v;
            } else {
                cfg.active_dpi_stage = 0;
            }
        }
        if let Some(v) = read_u8_checked(flash, 0x0A)
            && v != 0
        {
            cfg.lod = LodLevel::from_wire(v);
        }

        for i in 0..8 {
            let dpi_off = 0x0C + i * 4;
            if let Some(raw) = read_3_checked(flash, dpi_off) {
                let (x, y) = sensor.decode_dpi(raw);
                cfg.stages[i].raw_dpi = raw;
                cfg.stages[i].dpi_x = x;
                cfg.stages[i].dpi_y = y;
            }

            let color_off = 0x2C + i * 4;
            if let Some(rgb) = read_3_checked(flash, color_off) {
                cfg.stages[i].color = rgb;
            }
        }

        let led_mode = read_u8_checked(flash, 0x4C).unwrap_or(2);
        let led_en = read_u8_checked(flash, 0x52).unwrap_or(1);
        cfg.dpi_led_mode = DpiLedMode::from_wire(led_mode, led_en);

        if let Some(v) = read_u8_checked(flash, 0x4E) {
            cfg.dpi_led_brightness = v;
        }
        if let Some(v) = read_u8_checked(flash, 0x50) {
            cfg.dpi_led_speed = v;
        }
        if let Some(v) = read_u8_checked(flash, 0xA9) {
            cfg.debounce_ms = v;
        }
        if let Some(v) = read_u8_checked(flash, 0xAB) {
            cfg.motion_sync = v != 0;
        }
        if let Some(v) = read_u8_checked(flash, 0xAD) {
            cfg.sleep_units_10s = v;
        }
        if let Some(v) = read_u8_checked(flash, 0xAF) {
            cfg.angle_snapping = v != 0;
        }
        if let Some(v) = read_u8_checked(flash, 0xB1) {
            cfg.ripple_control = v != 0;
        }
        if let Some(v) = read_u8_checked(flash, 0xB3) {
            cfg.moving_light_off = v != 0;
        }
        if let Some(v) = read_u8_checked(flash, 0xB5) {
            cfg.high_performance = v != 0;
        }
        if let Some(v) = read_u8_checked(flash, 0xB7) {
            cfg.high_perf_timeout_10s = v;
        }
        if let Some(v) = read_u8_checked(flash, 0xB9) {
            cfg.sensor_mode = SensorMode::from_wire(v);
        }

        cfg
    }

    /// Compute minimal `(flash_addr, payload_with_checksum)` writes required to transition `self` -> `next`,
    /// mirroring `HIDUsb.dll`'s `ProtocolDataCompareUpdate`.
    #[must_use]
    pub fn diff_writes(&self, next: &Self, sensor: SensorKind) -> Vec<(u16, Vec<u8>)> {
        let mut writes = Vec::new();

        push_if_changed_u8(
            &mut writes,
            0x00,
            self.polling_rate as u8,
            next.polling_rate as u8,
        );
        push_if_changed_u8(
            &mut writes,
            0x02,
            self.dpi_stage_count,
            next.dpi_stage_count,
        );
        push_if_changed_u8(
            &mut writes,
            0x04,
            self.active_dpi_stage,
            next.active_dpi_stage,
        );
        push_if_changed_u8(&mut writes, 0x0A, self.lod as u8, next.lod as u8);

        for i in 0..8 {
            let old_raw = self.stages[i].raw_dpi;
            let new_raw = sensor.encode_dpi(next.stages[i].dpi_x, next.stages[i].dpi_y);
            if old_raw != new_raw {
                let addr = 0x0C + (i as u16) * 4;
                writes.push((addr, with_checksum(&new_raw)));
            }

            if self.stages[i].color != next.stages[i].color {
                let addr = 0x2C + (i as u16) * 4;
                writes.push((addr, with_checksum(&next.stages[i].color)));
            }
        }

        let (old_mode, old_en) = self.dpi_led_mode.wire_pair();
        let (new_mode, new_en) = next.dpi_led_mode.wire_pair();
        push_if_changed_u8(&mut writes, 0x4C, old_mode, new_mode);
        push_if_changed_u8(
            &mut writes,
            0x4E,
            self.dpi_led_brightness,
            next.dpi_led_brightness,
        );
        push_if_changed_u8(&mut writes, 0x50, self.dpi_led_speed, next.dpi_led_speed);
        push_if_changed_u8(&mut writes, 0x52, old_en, new_en);

        push_if_changed_u8(&mut writes, 0xA9, self.debounce_ms, next.debounce_ms);
        push_if_changed_u8(
            &mut writes,
            0xAB,
            u8::from(self.motion_sync),
            u8::from(next.motion_sync),
        );
        push_if_changed_u8(
            &mut writes,
            0xAD,
            self.sleep_units_10s,
            next.sleep_units_10s,
        );
        push_if_changed_u8(
            &mut writes,
            0xAF,
            u8::from(self.angle_snapping),
            u8::from(next.angle_snapping),
        );
        push_if_changed_u8(
            &mut writes,
            0xB1,
            u8::from(self.ripple_control),
            u8::from(next.ripple_control),
        );
        push_if_changed_u8(
            &mut writes,
            0xB3,
            u8::from(self.moving_light_off),
            u8::from(next.moving_light_off),
        );
        push_if_changed_u8(
            &mut writes,
            0xB5,
            u8::from(self.high_performance),
            u8::from(next.high_performance),
        );
        push_if_changed_u8(
            &mut writes,
            0xB7,
            self.high_perf_timeout_10s,
            next.high_perf_timeout_10s,
        );
        push_if_changed_u8(
            &mut writes,
            0xB9,
            self.sensor_mode as u8,
            next.sensor_mode as u8,
        );

        writes
    }

    /// Re-encode `stages[idx].raw_dpi` after changing `dpi_x` or `dpi_y`.
    pub fn set_stage_dpi(&mut self, index: usize, dpi_x: u32, dpi_y: u32, sensor: SensorKind) {
        if index >= 8 {
            return;
        }
        let spec = sensor.spec();
        let x = sensor.snap_dpi(dpi_x);
        let y = if spec.supports_independent_xy {
            sensor.snap_dpi(dpi_y)
        } else {
            x
        };
        let raw = sensor.encode_dpi(x, y);
        let (dx, dy) = sensor.decode_dpi(raw);
        self.stages[index].dpi_x = dx;
        self.stages[index].dpi_y = dy;
        self.stages[index].raw_dpi = raw;
    }

    /// Validate all fields of this configuration against resolved device capabilities.
    pub fn validate(&self, caps: &DeviceCapabilities) -> Result<(), ConfigValidationError> {
        let spec = caps.sensor_spec();
        validate_polling_rate(self.polling_rate, caps)?;

        if !(1..=caps.max_dpi_stages).contains(&self.dpi_stage_count) {
            return Err(ConfigValidationError::InvalidStageCount {
                count: self.dpi_stage_count,
                max_stages: caps.max_dpi_stages,
            });
        }
        if self.active_dpi_stage >= self.dpi_stage_count
            || self.active_dpi_stage >= caps.max_dpi_stages
        {
            return Err(ConfigValidationError::InvalidActiveStage {
                stage: self.active_dpi_stage,
                enabled_count: self.dpi_stage_count,
            });
        }
        if !caps.supports_lod_level(self.lod) && (caps.supports_lod() || self.lod != LodLevel::Mm1)
        {
            return Err(ConfigValidationError::LodUnsupported {
                lod: self.lod,
                sensor_name: spec.name,
            });
        }

        for i in 0..caps.max_dpi_stages.min(8) as usize {
            validate_stage_dpi(i, &self.stages[i], caps)?;
        }

        validate_bounds(self.dpi_led_speed, self.debounce_ms, self.sleep_units_10s)?;

        if self.motion_sync && !caps.supports_motion_sync() {
            return Err(ConfigValidationError::FeatureUnsupported {
                feature: "Motion Sync",
                sensor_name: spec.name,
            });
        }
        if self.angle_snapping && !caps.supports_angle_snapping() {
            return Err(ConfigValidationError::FeatureUnsupported {
                feature: "Angle Snapping",
                sensor_name: spec.name,
            });
        }
        if self.ripple_control && !caps.supports_ripple_control() {
            return Err(ConfigValidationError::FeatureUnsupported {
                feature: "Ripple Control",
                sensor_name: spec.name,
            });
        }
        if (self.high_performance || self.sensor_mode != SensorMode::LowPower)
            && !caps.supports_sensor_mode()
        {
            return Err(ConfigValidationError::FeatureUnsupported {
                feature: "High Performance / Sensor Mode",
                sensor_name: spec.name,
            });
        }

        Ok(())
    }

    /// Validate only the fields that changed between `self` and `next` against `caps`.
    ///
    /// Pre-existing out-of-spec values in unrelated flash fields do not block valid edits or
    /// downgrades back into the supported range.
    pub fn validate_diff(
        &self,
        next: &Self,
        caps: &DeviceCapabilities,
    ) -> Result<(), ConfigValidationError> {
        let spec = caps.sensor_spec();

        if self.polling_rate != next.polling_rate {
            validate_polling_rate(next.polling_rate, caps)?;
        }

        if self.dpi_stage_count != next.dpi_stage_count
            && !(1..=caps.max_dpi_stages).contains(&next.dpi_stage_count)
        {
            return Err(ConfigValidationError::InvalidStageCount {
                count: next.dpi_stage_count,
                max_stages: caps.max_dpi_stages,
            });
        }

        if (self.active_dpi_stage != next.active_dpi_stage
            || self.dpi_stage_count != next.dpi_stage_count)
            && (next.active_dpi_stage >= next.dpi_stage_count
                || next.active_dpi_stage >= caps.max_dpi_stages)
        {
            return Err(ConfigValidationError::InvalidActiveStage {
                stage: next.active_dpi_stage,
                enabled_count: next.dpi_stage_count,
            });
        }

        if self.lod != next.lod && !caps.supports_lod_level(next.lod) {
            return Err(ConfigValidationError::LodUnsupported {
                lod: next.lod,
                sensor_name: spec.name,
            });
        }

        for i in 0..8 {
            let stage_dpi_changed = self.stages[i].dpi_x != next.stages[i].dpi_x
                || self.stages[i].dpi_y != next.stages[i].dpi_y
                || self.stages[i].raw_dpi != next.stages[i].raw_dpi;
            if stage_dpi_changed {
                validate_stage_dpi(i, &next.stages[i], caps)?;
            }
            if self.stages[i].color != next.stages[i].color && (i as u8) >= caps.max_dpi_stages {
                return Err(ConfigValidationError::StageOutOfRange {
                    stage: (i + 1) as u8,
                    max_stages: caps.max_dpi_stages,
                });
            }
        }

        if self.dpi_led_speed != next.dpi_led_speed
            && !(MIN_LED_SPEED..=MAX_LED_SPEED).contains(&next.dpi_led_speed)
        {
            return Err(ConfigValidationError::LedSpeedOutOfRange {
                speed: next.dpi_led_speed,
                min_speed: MIN_LED_SPEED,
                max_speed: MAX_LED_SPEED,
            });
        }

        if self.debounce_ms != next.debounce_ms
            && !(MIN_DEBOUNCE_MS..=MAX_DEBOUNCE_MS).contains(&next.debounce_ms)
        {
            return Err(ConfigValidationError::DebounceOutOfRange {
                ms: next.debounce_ms,
                min_ms: MIN_DEBOUNCE_MS,
                max_ms: MAX_DEBOUNCE_MS,
            });
        }

        if self.sleep_units_10s != next.sleep_units_10s
            && !(MIN_SLEEP_MINUTES * 6..=MAX_SLEEP_MINUTES * 6).contains(&next.sleep_units_10s)
        {
            return Err(ConfigValidationError::SleepTimeoutOutOfRange {
                units_10s: next.sleep_units_10s,
                min_units: MIN_SLEEP_MINUTES * 6,
                max_units: MAX_SLEEP_MINUTES * 6,
            });
        }

        if self.motion_sync != next.motion_sync && next.motion_sync && !caps.supports_motion_sync()
        {
            return Err(ConfigValidationError::FeatureUnsupported {
                feature: "Motion Sync",
                sensor_name: spec.name,
            });
        }
        if self.angle_snapping != next.angle_snapping
            && next.angle_snapping
            && !caps.supports_angle_snapping()
        {
            return Err(ConfigValidationError::FeatureUnsupported {
                feature: "Angle Snapping",
                sensor_name: spec.name,
            });
        }
        if self.ripple_control != next.ripple_control
            && next.ripple_control
            && !caps.supports_ripple_control()
        {
            return Err(ConfigValidationError::FeatureUnsupported {
                feature: "Ripple Control",
                sensor_name: spec.name,
            });
        }
        if self.high_performance != next.high_performance
            && next.high_performance
            && !caps.supports_sensor_mode()
        {
            return Err(ConfigValidationError::FeatureUnsupported {
                feature: "High Performance Mode",
                sensor_name: spec.name,
            });
        }
        if self.sensor_mode != next.sensor_mode && !caps.supports_sensor_mode() {
            return Err(ConfigValidationError::FeatureUnsupported {
                feature: "Sensor Mode",
                sensor_name: spec.name,
            });
        }

        Ok(())
    }
}

fn validate_polling_rate(
    rate: PollingRate,
    caps: &DeviceCapabilities,
) -> Result<(), ConfigValidationError> {
    if caps.supports_polling_rate(rate) {
        return Ok(());
    }
    let supported_hz = caps
        .supported_polling_rates()
        .iter()
        .map(|r| r.hz())
        .collect();
    Err(ConfigValidationError::UnsupportedPollingRate {
        requested_hz: rate.hz(),
        max_hz: caps.max_polling_rate.hz(),
        supported_hz,
        context: caps.connection_label().to_string(),
    })
}

fn validate_stage_dpi(
    index: usize,
    stage: &DpiStage,
    caps: &DeviceCapabilities,
) -> Result<(), ConfigValidationError> {
    let stage_num = (index + 1) as u8;
    if stage_num > caps.max_dpi_stages {
        return Err(ConfigValidationError::StageOutOfRange {
            stage: stage_num,
            max_stages: caps.max_dpi_stages,
        });
    }
    let spec = caps.sensor_spec();
    if stage.dpi_x < spec.min_dpi || stage.dpi_x > caps.max_dpi {
        return Err(ConfigValidationError::DpiOutOfRange {
            stage: stage_num,
            dpi: stage.dpi_x,
            min_dpi: spec.min_dpi,
            max_dpi: caps.max_dpi,
            sensor_name: spec.name,
        });
    }
    if stage.dpi_y < spec.min_dpi || stage.dpi_y > caps.max_dpi {
        return Err(ConfigValidationError::DpiOutOfRange {
            stage: stage_num,
            dpi: stage.dpi_y,
            min_dpi: spec.min_dpi,
            max_dpi: caps.max_dpi,
            sensor_name: spec.name,
        });
    }
    if !caps.supports_independent_xy() && stage.dpi_x != stage.dpi_y {
        return Err(ConfigValidationError::IndependentXyUnsupported {
            stage: stage_num,
            sensor_name: spec.name,
        });
    }
    Ok(())
}

fn validate_bounds(
    dpi_led_speed: u8,
    debounce_ms: u8,
    sleep_units_10s: u8,
) -> Result<(), ConfigValidationError> {
    if !(MIN_LED_SPEED..=MAX_LED_SPEED).contains(&dpi_led_speed) {
        return Err(ConfigValidationError::LedSpeedOutOfRange {
            speed: dpi_led_speed,
            min_speed: MIN_LED_SPEED,
            max_speed: MAX_LED_SPEED,
        });
    }
    if !(MIN_DEBOUNCE_MS..=MAX_DEBOUNCE_MS).contains(&debounce_ms) {
        return Err(ConfigValidationError::DebounceOutOfRange {
            ms: debounce_ms,
            min_ms: MIN_DEBOUNCE_MS,
            max_ms: MAX_DEBOUNCE_MS,
        });
    }
    if !(MIN_SLEEP_MINUTES * 6..=MAX_SLEEP_MINUTES * 6).contains(&sleep_units_10s) {
        return Err(ConfigValidationError::SleepTimeoutOutOfRange {
            units_10s: sleep_units_10s,
            min_units: MIN_SLEEP_MINUTES * 6,
            max_units: MAX_SLEEP_MINUTES * 6,
        });
    }
    Ok(())
}

fn read_u8_checked(flash: &[u8; FLASH_TOTAL_LEN], offset: usize) -> Option<u8> {
    let val = flash[offset];
    let cksum = flash[offset + 1];
    (compx_checksum(&[val]) == cksum).then_some(val)
}

fn read_3_checked(flash: &[u8; FLASH_TOTAL_LEN], offset: usize) -> Option<[u8; 3]> {
    let slice = &flash[offset..offset + 3];
    let cksum = flash[offset + 3];
    (compx_checksum(slice) == cksum).then_some([slice[0], slice[1], slice[2]])
}

fn with_checksum(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len() + 1);
    out.extend_from_slice(bytes);
    out.push(compx_checksum(bytes));
    out
}

fn push_if_changed_u8(writes: &mut Vec<(u16, Vec<u8>)>, addr: u16, old: u8, new: u8) {
    if old != new {
        writes.push((addr, with_checksum(&[new])));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_and_corrupt_flash() {
        let sensor = SensorKind::Paw3950;
        let mut flash = [0xFFu8; FLASH_TOTAL_LEN];

        // Set polling rate 4000Hz (0x20), stage count 4, active stage 1
        flash[0x00] = 0x20;
        flash[0x01] = compx_checksum(&[0x20]);
        flash[0x02] = 4;
        flash[0x03] = compx_checksum(&[4]);
        flash[0x04] = 1;
        flash[0x05] = compx_checksum(&[1]);

        // Set stage 1 to 2400 DPI
        let raw = sensor.encode_dpi(2400, 2400);
        flash[0x10..0x13].copy_from_slice(&raw);
        flash[0x13] = compx_checksum(&raw);

        // Corrupt stage 2 checksum intentionally -> should fall back to default 1000 DPI
        flash[0x14..0x17].copy_from_slice(&[0xAA, 0xBB, 0xCC]);
        flash[0x17] = 0x00;

        let cfg = MouseConfig::from_flash(&flash, sensor);
        assert_eq!(cfg.polling_rate, PollingRate::Hz4000);
        assert_eq!(cfg.dpi_stage_count, 4);
        assert_eq!(cfg.active_dpi_stage, 1);
        assert_eq!(cfg.stages[1].dpi_x, 2400);
        assert_eq!(cfg.stages[2].dpi_x, DEFAULT_DPI_GRADES[2]);
    }

    #[test]
    fn minimal_diff_writes() {
        let sensor = SensorKind::Paw3950;
        let base = MouseConfig::demo_default(sensor);

        let mut next = base.clone();
        next.active_dpi_stage = 4;
        let diff = base.diff_writes(&next, sensor);
        assert_eq!(diff, vec![(0x04, vec![4, compx_checksum(&[4])])]);

        let mut next2 = base.clone();
        next2.set_stage_dpi(1, 1600, 1600, sensor);
        let raw = sensor.encode_dpi(1600, 1600);
        let diff2 = base.diff_writes(&next2, sensor);
        assert_eq!(
            diff2,
            vec![(0x10, vec![raw[0], raw[1], raw[2], compx_checksum(&raw)])]
        );

        let mut next3 = base.clone();
        next3.stages[2].color = [0x11, 0x22, 0x33];
        let diff3 = base.diff_writes(&next3, sensor);
        assert_eq!(
            diff3,
            vec![(
                0x34,
                vec![0x11, 0x22, 0x33, compx_checksum(&[0x11, 0x22, 0x33])]
            )]
        );
    }

    #[test]
    fn polling_rate_ord_uses_hz_not_wire_byte() {
        assert!(PollingRate::Hz125 < PollingRate::Hz250);
        assert!(PollingRate::Hz250 < PollingRate::Hz500);
        assert!(PollingRate::Hz500 < PollingRate::Hz1000);
        assert!(PollingRate::Hz1000 < PollingRate::Hz2000);
        assert!(PollingRate::Hz2000 < PollingRate::Hz4000);
        assert!(PollingRate::Hz4000 < PollingRate::Hz8000);
        assert_eq!(
            PollingRate::Hz8000.min_rate(PollingRate::Hz1000),
            PollingRate::Hz1000
        );
    }

    #[test]
    fn validation_enforces_polling_rate_and_sensor_limits() {
        use crate::devices::{ConnectionKind, profile_for_mid};

        // Quasar 3 Wireless: MID=7, PAW3311 12K, 1K max
        let q3_caps = DeviceCapabilities::resolve(
            profile_for_mid(7),
            profile_for_mid(7).sensor,
            0,
            ConnectionKind::Wireless24G,
            "HATOR Quasar 3 Wireless",
        );
        let base_q3 = MouseConfig::demo_default(q3_caps.sensor);
        assert!(base_q3.validate(&q3_caps).is_ok());

        for rate in [
            PollingRate::Hz2000,
            PollingRate::Hz4000,
            PollingRate::Hz8000,
        ] {
            let mut next = base_q3.clone();
            next.polling_rate = rate;
            assert!(matches!(
                base_q3.validate_diff(&next, &q3_caps),
                Err(ConfigValidationError::UnsupportedPollingRate { .. })
            ));
            assert!(matches!(
                next.validate(&q3_caps),
                Err(ConfigValidationError::UnsupportedPollingRate { .. })
            ));
        }

        // Downgrading from 8K back to 1K on a 1K connection is allowed
        let mut bad_8k = base_q3.clone();
        bad_8k.polling_rate = PollingRate::Hz8000;
        assert!(bad_8k.validate_diff(&base_q3, &q3_caps).is_ok());

        // Unrelated DPI edit succeeds even if pre-existing flash had 8K polling rate
        let mut bad_8k_dpi = bad_8k.clone();
        bad_8k_dpi.set_stage_dpi(0, 800, 800, q3_caps.sensor);
        assert!(bad_8k.validate_diff(&bad_8k_dpi, &q3_caps).is_ok());

        // Reject DPI > 12000 on Quasar 3 Wireless
        let mut high_dpi = base_q3.clone();
        high_dpi.stages[0].dpi_x = 16_000;
        high_dpi.stages[0].dpi_y = 16_000;
        assert!(matches!(
            base_q3.validate_diff(&high_dpi, &q3_caps),
            Err(ConfigValidationError::DpiOutOfRange { dpi: 16_000, .. })
        ));

        // Reject independent X/Y on PAW3311
        let mut xy_diff = base_q3.clone();
        xy_diff.stages[0].dpi_x = 800;
        xy_diff.stages[0].dpi_y = 1600;
        assert!(matches!(
            base_q3.validate_diff(&xy_diff, &q3_caps),
            Err(ConfigValidationError::IndependentXyUnsupported { .. })
        ));

        // Reject LOD change on PAW3311
        let mut lod_diff = base_q3.clone();
        lod_diff.lod = LodLevel::Mm2;
        assert!(matches!(
            base_q3.validate_diff(&lod_diff, &q3_caps),
            Err(ConfigValidationError::LodUnsupported { .. })
        ));

        // Reject Motion Sync on PAW3311
        let mut ms_diff = base_q3.clone();
        ms_diff.motion_sync = true;
        assert!(matches!(
            base_q3.validate_diff(&ms_diff, &q3_caps),
            Err(ConfigValidationError::FeatureUnsupported {
                feature: "Motion Sync",
                ..
            })
        ));

        // Reject stage count > 6
        let mut stage_cnt = base_q3.clone();
        stage_cnt.dpi_stage_count = 7;
        assert!(matches!(
            base_q3.validate_diff(&stage_cnt, &q3_caps),
            Err(ConfigValidationError::InvalidStageCount {
                count: 7,
                max_stages: 6
            })
        ));

        // Reject out-of-range debounce, sleep timeout, and LED speed
        let mut bad_debounce = base_q3.clone();
        bad_debounce.debounce_ms = 31;
        assert!(matches!(
            base_q3.validate_diff(&bad_debounce, &q3_caps),
            Err(ConfigValidationError::DebounceOutOfRange { ms: 31, .. })
        ));

        let mut bad_sleep = base_q3.clone();
        bad_sleep.sleep_units_10s = 250;
        assert!(matches!(
            base_q3.validate_diff(&bad_sleep, &q3_caps),
            Err(ConfigValidationError::SleepTimeoutOutOfRange { .. })
        ));

        let mut bad_speed = base_q3.clone();
        bad_speed.dpi_led_speed = 0;
        assert!(matches!(
            base_q3.validate_diff(&bad_speed, &q3_caps),
            Err(ConfigValidationError::LedSpeedOutOfRange { speed: 0, .. })
        ));

        // Reject 0.7 mm LOD on PAW3370 and PAW3335
        for sensor in [SensorKind::Paw3370, SensorKind::Paw3335] {
            let caps = DeviceCapabilities::resolve(
                profile_for_mid(8),
                sensor,
                2,
                ConnectionKind::Wired,
                "HATOR Mouse",
            );
            let base = MouseConfig::demo_default(sensor);
            let mut next = base.clone();
            next.lod = LodLevel::Mm07;
            assert!(matches!(
                base.validate_diff(&next, &caps),
                Err(ConfigValidationError::LodUnsupported {
                    lod: LodLevel::Mm07,
                    ..
                })
            ));
        }

        // Quasar 3 Ultima 8K wired caps at 1K; wireless 8K allows 8K
        let ultima_wired = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            2,
            ConnectionKind::Wired,
            "HATOR Quasar 3 Ultima 8K Wireless",
        );
        let base_ultima = MouseConfig::demo_default(SensorKind::Paw3950);
        let mut next_8k = base_ultima.clone();
        next_8k.polling_rate = PollingRate::Hz8000;
        assert!(matches!(
            base_ultima.validate_diff(&next_8k, &ultima_wired),
            Err(ConfigValidationError::UnsupportedPollingRate { .. })
        ));

        let ultima_wl = DeviceCapabilities::resolve(
            profile_for_mid(8),
            SensorKind::Paw3950,
            5,
            ConnectionKind::Wireless8K,
            "HATOR Quasar 3 Ultima 8K Wireless",
        );
        assert!(base_ultima.validate_diff(&next_8k, &ultima_wl).is_ok());
    }
}
