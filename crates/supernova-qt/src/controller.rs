//! `cxx-qt` bridge exposing HATOR mouse DPI & sensor controls to Qt 6 QML.

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_cxx_name]
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(bool, connected)]
        #[qproperty(bool, mouse_online)]
        #[qproperty(bool, permission_denied)]
        #[qproperty(bool, demo_mode)]
        #[qproperty(QString, device_name)]
        #[qproperty(QString, connection_mode)]
        #[qproperty(QString, sensor_name)]
        #[qproperty(QString, firmware_version)]
        #[qproperty(QString, status_message)]
        #[qproperty(QString, error_message)]
        #[qproperty(QString, udev_help_command)]
        #[qproperty(i32, battery_percent)]
        #[qproperty(bool, battery_charging)]
        #[qproperty(i32, battery_mv)]
        #[qproperty(i32, dpi_min)]
        #[qproperty(i32, dpi_max)]
        #[qproperty(i32, dpi_step)]
        #[qproperty(bool, supports_xy)]
        #[qproperty(bool, supports_sub_mm_lod)]
        #[qproperty(i32, max_stages)]
        #[qproperty(i32, stage_count)]
        #[qproperty(i32, active_stage)]
        #[qproperty(i32, active_dpi_x)]
        #[qproperty(i32, active_dpi_y)]
        #[qproperty(QString, active_color)]
        #[qproperty(bool, xy_linked)]
        #[qproperty(QString, stages_json)]
        #[qproperty(i32, dpi_led_mode)]
        #[qproperty(i32, dpi_led_brightness)]
        #[qproperty(i32, dpi_led_speed)]
        #[qproperty(i32, polling_rate_hz)]
        #[qproperty(i32, max_polling_rate_hz)]
        #[qproperty(QString, supported_polling_rates_json)]
        #[qproperty(QString, polling_rate_limit_note)]
        #[qproperty(i32, lod_level)]
        #[qproperty(bool, supports_lod)]
        #[qproperty(i32, debounce_ms)]
        #[qproperty(i32, sleep_minutes)]
        #[qproperty(bool, motion_sync)]
        #[qproperty(bool, supports_motion_sync)]
        #[qproperty(bool, angle_snapping)]
        #[qproperty(bool, supports_angle_snapping)]
        #[qproperty(bool, ripple_control)]
        #[qproperty(bool, supports_ripple_control)]
        #[qproperty(bool, high_performance)]
        #[qproperty(bool, supports_high_performance)]
        type MouseController = super::MouseControllerRust;

        #[qinvokable]
        #[cxx_name = "refreshDevice"]
        fn refresh_device(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "applyDemoMode"]
        fn apply_demo_mode(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "selectStage"]
        fn select_stage(self: Pin<&mut Self>, stage_index: i32);

        #[qinvokable]
        #[cxx_name = "applyStageCount"]
        fn apply_stage_count(self: Pin<&mut Self>, count: i32);

        #[qinvokable]
        #[cxx_name = "setStageDpi"]
        fn set_stage_dpi(self: Pin<&mut Self>, stage_index: i32, dpi_x: i32, dpi_y: i32);

        #[qinvokable]
        #[cxx_name = "applyXyLinked"]
        fn apply_xy_linked(self: Pin<&mut Self>, linked: bool);

        #[qinvokable]
        #[cxx_name = "setStageColor"]
        fn set_stage_color(self: Pin<&mut Self>, stage_index: i32, hex_color: &QString);

        #[qinvokable]
        #[cxx_name = "setDpiLed"]
        fn set_dpi_led(self: Pin<&mut Self>, mode: i32, brightness: i32, speed: i32);

        #[qinvokable]
        #[cxx_name = "setPollingRate"]
        fn set_polling_rate(self: Pin<&mut Self>, hz: i32);

        #[qinvokable]
        #[cxx_name = "setLod"]
        fn set_lod(self: Pin<&mut Self>, level: i32);

        #[qinvokable]
        #[cxx_name = "applyDebounceMs"]
        fn apply_debounce_ms(self: Pin<&mut Self>, ms: i32);

        #[qinvokable]
        #[cxx_name = "applySleepMinutes"]
        fn apply_sleep_minutes(self: Pin<&mut Self>, minutes: i32);

        #[qinvokable]
        #[cxx_name = "applyMotionSync"]
        fn apply_motion_sync(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "applyAngleSnapping"]
        fn apply_angle_snapping(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "applyRippleControl"]
        fn apply_ripple_control(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "applyHighPerformance"]
        fn apply_high_performance(self: Pin<&mut Self>, enabled: bool);

        #[qinvokable]
        #[cxx_name = "factoryReset"]
        fn factory_reset(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "pollDevice"]
        fn poll_device(self: Pin<&mut Self>);
    }
}

use core::pin::Pin;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use serde::Serialize;
use supernova_core::devices::profile_for_mid;
use supernova_core::{
    discover_devices, BatteryStatus, ConnectionKind, DeviceCapabilities, DpiLedMode, HatorDevice,
    HidError, HidapiTransport, LodLevel, MouseConfig, PollingRate, SensorKind, MAX_DEBOUNCE_MS,
    MAX_LED_SPEED, MAX_SLEEP_MINUTES, MIN_DEBOUNCE_MS, MIN_LED_SPEED, MIN_SLEEP_MINUTES,
};

const UDEV_HELP_CMD: &str =
    "sudo cp udev/70-hator-mouse.rules /etc/udev/rules.d/ && sudo udevadm control --reload-rules && sudo udevadm trigger --subsystem-match=hidraw --subsystem-match=usb";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct QmlStageEntry {
    index: usize,
    stage: usize,
    dpi_x: u32,
    dpi_y: u32,
    color: String,
    active: bool,
    enabled: bool,
}

struct ActiveConnection {
    device: Option<HatorDevice<HidapiTransport>>,
    caps: DeviceCapabilities,
    config: MouseConfig,
    device_name: String,
    firmware_version: String,
    battery: Option<BatteryStatus>,
}

pub struct MouseControllerRust {
    connected: bool,
    mouse_online: bool,
    permission_denied: bool,
    demo_mode: bool,
    device_name: QString,
    connection_mode: QString,
    sensor_name: QString,
    firmware_version: QString,
    status_message: QString,
    error_message: QString,
    udev_help_command: QString,
    battery_percent: i32,
    battery_charging: bool,
    battery_mv: i32,
    dpi_min: i32,
    dpi_max: i32,
    dpi_step: i32,
    supports_xy: bool,
    supports_sub_mm_lod: bool,
    max_stages: i32,
    stage_count: i32,
    active_stage: i32,
    active_dpi_x: i32,
    active_dpi_y: i32,
    active_color: QString,
    xy_linked: bool,
    stages_json: QString,
    dpi_led_mode: i32,
    dpi_led_brightness: i32,
    dpi_led_speed: i32,
    polling_rate_hz: i32,
    max_polling_rate_hz: i32,
    supported_polling_rates_json: QString,
    polling_rate_limit_note: QString,
    lod_level: i32,
    supports_lod: bool,
    debounce_ms: i32,
    sleep_minutes: i32,
    motion_sync: bool,
    supports_motion_sync: bool,
    angle_snapping: bool,
    supports_angle_snapping: bool,
    ripple_control: bool,
    supports_ripple_control: bool,
    high_performance: bool,
    supports_high_performance: bool,

    active: Option<ActiveConnection>,
    poll_ticks: u32,
    startup_demo_checked: bool,
}

impl Default for MouseControllerRust {
    fn default() -> Self {
        let sensor = SensorKind::Paw3950;
        let spec = sensor.spec();
        let cfg = MouseConfig::demo_default(sensor);
        let stages_json = serialize_stages(&cfg, 6);
        let active_st = &cfg.stages[cfg.active_dpi_stage as usize];
        Self {
            connected: false,
            mouse_online: false,
            permission_denied: false,
            demo_mode: false,
            device_name: QString::from("No HATOR Mouse Connected"),
            connection_mode: QString::from("Disconnected"),
            sensor_name: QString::from(spec.name),
            firmware_version: QString::from("--"),
            status_message: QString::from("Scanning for HATOR USB HID devices..."),
            error_message: QString::from(""),
            udev_help_command: QString::from(UDEV_HELP_CMD),
            battery_percent: 0,
            battery_charging: false,
            battery_mv: 0,
            dpi_min: spec.min_dpi as i32,
            dpi_max: spec.max_dpi as i32,
            dpi_step: spec.base_step as i32,
            supports_xy: spec.supports_independent_xy,
            supports_sub_mm_lod: spec.supports_lod_07mm,
            max_stages: 6,
            stage_count: i32::from(cfg.dpi_stage_count),
            active_stage: i32::from(cfg.active_dpi_stage),
            active_dpi_x: active_st.dpi_x as i32,
            active_dpi_y: active_st.dpi_y as i32,
            active_color: QString::from(&format_hex(active_st.color)),
            xy_linked: true,
            stages_json: QString::from(&stages_json),
            dpi_led_mode: cfg.dpi_led_mode as i32,
            dpi_led_brightness: i32::from(cfg.dpi_led_brightness),
            dpi_led_speed: i32::from(cfg.dpi_led_speed),
            polling_rate_hz: cfg.polling_rate.hz() as i32,
            max_polling_rate_hz: 1000,
            supported_polling_rates_json: QString::from("[125,250,500,1000]"),
            polling_rate_limit_note: QString::from(""),
            lod_level: cfg.lod as i32,
            supports_lod: spec.supports_lod,
            debounce_ms: i32::from(cfg.debounce_ms),
            sleep_minutes: i32::from((cfg.sleep_units_10s / 6).max(1)),
            motion_sync: cfg.motion_sync,
            supports_motion_sync: spec.supports_motion_sync,
            angle_snapping: cfg.angle_snapping,
            supports_angle_snapping: spec.supports_angle_snapping,
            ripple_control: cfg.ripple_control,
            supports_ripple_control: spec.supports_ripple_control,
            high_performance: cfg.high_performance,
            supports_high_performance: spec.supports_sensor_mode,
            active: None,
            poll_ticks: 0,
            startup_demo_checked: false,
        }
    }
}

fn format_hex(rgb: [u8; 3]) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2])
}

fn parse_hex(hex: &str) -> Option<[u8; 3]> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&clean[0..2], 16).ok()?;
    let g = u8::from_str_radix(&clean[2..4], 16).ok()?;
    let b = u8::from_str_radix(&clean[4..6], 16).ok()?;
    Some([r, g, b])
}

fn serialize_stages(cfg: &MouseConfig, max_stages: u8) -> String {
    let entries: Vec<QmlStageEntry> = (0..max_stages.min(8) as usize)
        .map(|i| {
            let st = &cfg.stages[i];
            QmlStageEntry {
                index: i,
                stage: i + 1,
                dpi_x: st.dpi_x,
                dpi_y: st.dpi_y,
                color: format_hex(st.color),
                active: i as u8 == cfg.active_dpi_stage,
                enabled: (i as u8) < cfg.dpi_stage_count,
            }
        })
        .collect();
    serde_json::to_string(&entries).unwrap_or_else(|_| "[]".to_string())
}

fn serialize_polling_rates(caps: &DeviceCapabilities) -> String {
    let rates: Vec<u32> = caps
        .supported_polling_rates()
        .iter()
        .map(|r| r.hz())
        .collect();
    serde_json::to_string(&rates).unwrap_or_else(|_| "[125,250,500,1000]".to_string())
}

fn should_start_in_demo() -> bool {
    std::env::var_os("SUPERNOVA_DEMO").is_some_and(|v| !v.is_empty() && v != "0")
        || std::env::args().any(|arg| arg == "--demo")
}

impl qobject::MouseController {
    pub fn refresh_device(mut self: Pin<&mut Self>) {
        if !self.rust().startup_demo_checked {
            self.as_mut().rust_mut().startup_demo_checked = true;
            if should_start_in_demo() {
                self.apply_demo_mode(true);
                return;
            }
        }

        if *self.demo_mode() {
            self.as_mut().apply_demo_mode(false);
            return;
        }

        self.as_mut().rust_mut().active = None;
        self.as_mut().set_permission_denied(false);
        self.as_mut().set_error_message(QString::from(""));

        let devices = match discover_devices() {
            Ok(list) => list,
            Err(err) => {
                self.as_mut().set_connected(false);
                self.as_mut().set_mouse_online(false);
                self.as_mut()
                    .set_status_message(QString::from("USB HID scan failed"));
                self.as_mut()
                    .set_error_message(QString::from(&err.to_string()));
                return;
            }
        };

        let Some(meta) = devices.into_iter().next() else {
            self.as_mut().set_connected(false);
            self.as_mut().set_mouse_online(false);
            self.as_mut()
                .set_device_name(QString::from("No HATOR Mouse Connected"));
            self.as_mut()
                .set_connection_mode(QString::from("Disconnected"));
            self.as_mut().set_status_message(QString::from(
                "Plug in your HATOR mouse or 2.4 GHz receiver",
            ));
            return;
        };

        let transport = match HidapiTransport::open(&meta.path) {
            Ok(t) => t,
            Err(HidError::PermissionDenied { path }) => {
                self.as_mut().set_connected(false);
                self.as_mut().set_mouse_online(false);
                self.as_mut().set_permission_denied(true);
                self.as_mut()
                    .set_device_name(QString::from(&meta.product_name));
                self.as_mut()
                    .set_connection_mode(QString::from(meta.connection.label()));
                self.as_mut().set_status_message(QString::from(&format!(
                    "Detected {} on {path}, awaiting hidraw udev permission",
                    meta.product_name
                )));
                self.as_mut().set_error_message(QString::from(&format!(
                    "Permission denied opening {path}. Install udev/70-hator-mouse.rules and reload udev rules."
                )));
                return;
            }
            Err(err) => {
                self.as_mut().set_connected(false);
                self.as_mut().set_mouse_online(false);
                self.as_mut()
                    .set_device_name(QString::from(&meta.product_name));
                self.as_mut()
                    .set_connection_mode(QString::from(meta.connection.label()));
                self.as_mut()
                    .set_error_message(QString::from(&err.to_string()));
                return;
            }
        };

        let mut dev = HatorDevice::new(transport);
        let (_info, caps) = match dev.capabilities(&meta, None) {
            Ok(res) => res,
            Err(err) => {
                self.as_mut().set_connected(false);
                self.as_mut().set_mouse_online(false);
                self.as_mut()
                    .set_device_name(QString::from(&meta.product_name));
                self.as_mut()
                    .set_connection_mode(QString::from(meta.connection.label()));
                self.as_mut()
                    .set_error_message(QString::from(&err.to_string()));
                return;
            }
        };

        let online = dev.is_online().unwrap_or(false);
        let fw = dev
            .read_firmware_version()
            .map(|v| format!("v{}.{:02X}", v >> 8, v & 0xFF))
            .unwrap_or_else(|_| "--".to_string());

        if !online {
            self.as_mut().set_connected(true);
            self.as_mut().set_mouse_online(false);
            self.as_mut()
                .set_device_name(QString::from(&meta.product_name));
            self.as_mut()
                .set_connection_mode(QString::from(caps.connection_label()));
            self.as_mut()
                .set_sensor_name(QString::from(caps.sensor_spec().name));
            self.as_mut().set_firmware_version(QString::from(&fw));
            self.as_mut()
                .set_status_message(QString::from("Receiver detected, but mouse is asleep"));
            self.as_mut().set_error_message(QString::from(
                "Move or click your wireless mouse to wake it, then click Refresh.",
            ));
            return;
        }

        let _ = dev.set_pc_driver_status(true);
        let battery = dev.read_battery().ok();
        let cfg = match dev.read_config(caps.sensor) {
            Ok(c) => c,
            Err(err) => {
                self.as_mut().set_connected(false);
                self.as_mut().set_mouse_online(false);
                self.as_mut()
                    .set_error_message(QString::from(&err.to_string()));
                return;
            }
        };

        self.as_mut().rust_mut().active = Some(ActiveConnection {
            device: Some(dev),
            caps,
            config: cfg,
            device_name: meta.product_name.clone(),
            firmware_version: fw,
            battery,
        });
        self.as_mut().rust_mut().poll_ticks = 0;
        self.as_mut().set_connected(true);
        self.as_mut().set_mouse_online(true);
        self.sync_ui_from_active(&format!("Connected to {}", meta.product_name), "");
    }

    /// Deprecated from user-facing UI; retained for offline development/testing via `--demo` or `SUPERNOVA_DEMO=1`.
    pub fn apply_demo_mode(mut self: Pin<&mut Self>, enabled: bool) {
        if !enabled {
            self.as_mut().set_demo_mode(false);
            self.as_mut().rust_mut().active = None;
            self.refresh_device();
            return;
        }

        let sensor = SensorKind::Paw3950;
        let caps = DeviceCapabilities::resolve(
            profile_for_mid(8),
            sensor,
            5,
            ConnectionKind::Wireless8K,
            "HATOR Quasar 3 Ultra 8K+8K Wireless",
        );
        let cfg = MouseConfig::demo_default(sensor);
        self.as_mut().rust_mut().active = Some(ActiveConnection {
            device: None,
            caps,
            config: cfg,
            device_name: "HATOR Quasar 3 Ultra 8K+8K Wireless (Demo)".to_string(),
            firmware_version: "v1.05".to_string(),
            battery: Some(BatteryStatus {
                percent: 84,
                charging: false,
                voltage_mv: 4030,
            }),
        });
        self.as_mut().set_demo_mode(true);
        self.as_mut().set_connected(true);
        self.as_mut().set_mouse_online(true);
        self.as_mut().set_permission_denied(false);
        self.sync_ui_from_active("Demo preview active — changes apply locally", "");
    }

    pub fn select_stage(self: Pin<&mut Self>, stage_index: i32) {
        self.mutate_config(|conn| {
            if stage_index < 0 || stage_index >= i32::from(conn.config.dpi_stage_count) {
                return Err("Selected stage is outside enabled stage count".to_string());
            }
            conn.config.active_dpi_stage = stage_index as u8;
            let st = &conn.config.stages[stage_index as usize];
            Ok(format!(
                "Switched to Stage {} ({} DPI)",
                stage_index + 1,
                st.dpi_x
            ))
        });
    }

    pub fn apply_stage_count(self: Pin<&mut Self>, count: i32) {
        self.mutate_config(|conn| {
            let clamped = count.clamp(1, i32::from(conn.caps.max_dpi_stages)) as u8;
            conn.config.dpi_stage_count = clamped;
            if conn.config.active_dpi_stage >= clamped {
                conn.config.active_dpi_stage = clamped - 1;
            }
            Ok(format!("Enabled DPI stages set to {clamped}"))
        });
    }

    pub fn set_stage_dpi(mut self: Pin<&mut Self>, stage_index: i32, dpi_x: i32, dpi_y: i32) {
        let linked = *self.xy_linked();
        let target_y = if linked { dpi_x } else { dpi_y };
        self.as_mut().mutate_config(|conn| {
            let max_stages = i32::from(conn.caps.max_dpi_stages);
            if !(0..max_stages).contains(&stage_index) {
                return Err(format!(
                    "Stage index out of range (valid: 1..={max_stages})"
                ));
            }
            let idx = stage_index as usize;
            let spec = conn.caps.sensor_spec();
            let clamped_x = dpi_x.clamp(spec.min_dpi as i32, conn.caps.max_dpi as i32) as u32;
            let clamped_y = if conn.caps.supports_independent_xy() {
                target_y.clamp(spec.min_dpi as i32, conn.caps.max_dpi as i32) as u32
            } else {
                clamped_x
            };
            conn.config
                .set_stage_dpi(idx, clamped_x, clamped_y, conn.caps.sensor);
            let st = &conn.config.stages[idx];
            if st.dpi_x == st.dpi_y {
                Ok(format!("Stage {} set to {} DPI", idx + 1, st.dpi_x))
            } else {
                Ok(format!(
                    "Stage {} set to {} × {} DPI",
                    idx + 1,
                    st.dpi_x,
                    st.dpi_y
                ))
            }
        });
    }

    pub fn apply_xy_linked(mut self: Pin<&mut Self>, linked: bool) {
        if !linked && !*self.supports_xy() {
            self.as_mut().set_xy_linked(true);
            self.as_mut().set_error_message(QString::from(
                "Independent X/Y DPI is not supported by this mouse sensor",
            ));
            return;
        }
        self.as_mut().set_xy_linked(linked);
        if linked {
            let active_idx = *self.active_stage();
            let active_x = *self.active_dpi_x();
            let active_y = *self.active_dpi_y();
            if active_x != active_y {
                self.set_stage_dpi(active_idx, active_x, active_x);
            }
        }
    }

    pub fn set_stage_color(self: Pin<&mut Self>, stage_index: i32, hex_color: &QString) {
        let hex = hex_color.to_string();
        let Some(rgb) = parse_hex(&hex) else {
            return;
        };
        self.mutate_config(|conn| {
            let max_stages = i32::from(conn.caps.max_dpi_stages);
            if !(0..max_stages).contains(&stage_index) {
                return Err(format!(
                    "Stage index out of range (valid: 1..={max_stages})"
                ));
            }
            conn.config.stages[stage_index as usize].color = rgb;
            Ok(format!(
                "Stage {} indicator color set to {}",
                stage_index + 1,
                format_hex(rgb)
            ))
        });
    }

    pub fn set_dpi_led(self: Pin<&mut Self>, mode: i32, brightness: i32, speed: i32) {
        self.mutate_config(|conn| {
            conn.config.dpi_led_mode = match mode {
                0 => DpiLedMode::Off,
                1 => DpiLedMode::Static,
                _ => DpiLedMode::Breathing,
            };
            conn.config.dpi_led_brightness = brightness.clamp(0, 255) as u8;
            conn.config.dpi_led_speed =
                speed.clamp(i32::from(MIN_LED_SPEED), i32::from(MAX_LED_SPEED)) as u8;
            Ok("Updated DPI indicator LED settings".to_string())
        });
    }

    pub fn set_polling_rate(mut self: Pin<&mut Self>, hz: i32) {
        let Some(rate) = PollingRate::from_hz(hz.max(0) as u32) else {
            self.as_mut()
                .set_error_message(QString::from(&format!("Unsupported polling rate {hz} Hz")));
            return;
        };
        self.mutate_config(|conn| {
            if !conn.caps.supports_polling_rate(rate) {
                return Err(format!(
                    "Polling rate {} Hz is not supported on {} (max {} Hz)",
                    rate.hz(),
                    conn.caps.connection_label(),
                    conn.caps.max_polling_rate.hz(),
                ));
            }
            conn.config.polling_rate = rate;
            Ok(format!("Polling rate set to {} Hz", rate.hz()))
        });
    }

    pub fn set_lod(mut self: Pin<&mut Self>, level: i32) {
        let lod = match level {
            1 => LodLevel::Mm1,
            2 => LodLevel::Mm2,
            3 => LodLevel::Mm07,
            _ => {
                self.as_mut().set_error_message(QString::from(&format!(
                    "Unsupported LOD level code {level}"
                )));
                return;
            }
        };
        self.mutate_config(|conn| {
            if !conn.caps.supports_lod_level(lod) {
                return Err(format!(
                    "Lift-off distance {} is not supported by {}",
                    lod.label(),
                    conn.caps.sensor_spec().name
                ));
            }
            conn.config.lod = lod;
            Ok(format!(
                "Lift-off distance set to {}",
                conn.config.lod.label()
            ))
        });
    }

    pub fn apply_debounce_ms(self: Pin<&mut Self>, ms: i32) {
        self.mutate_config(|conn| {
            let clamped = ms.clamp(i32::from(MIN_DEBOUNCE_MS), i32::from(MAX_DEBOUNCE_MS)) as u8;
            conn.config.debounce_ms = clamped;
            Ok(format!("Debounce time set to {clamped} ms"))
        });
    }

    pub fn apply_sleep_minutes(self: Pin<&mut Self>, minutes: i32) {
        self.mutate_config(|conn| {
            let mins =
                minutes.clamp(i32::from(MIN_SLEEP_MINUTES), i32::from(MAX_SLEEP_MINUTES)) as u8;
            conn.config.sleep_units_10s = mins.saturating_mul(6);
            Ok(format!("Sleep timer set to {mins} min"))
        });
    }

    pub fn apply_motion_sync(self: Pin<&mut Self>, enabled: bool) {
        self.mutate_config(|conn| {
            if enabled && !conn.caps.supports_motion_sync() {
                return Err(format!(
                    "Motion Sync is not supported by {}",
                    conn.caps.sensor_spec().name
                ));
            }
            conn.config.motion_sync = enabled;
            Ok(format!(
                "Motion Sync {}",
                if enabled { "enabled" } else { "disabled" }
            ))
        });
    }

    pub fn apply_angle_snapping(self: Pin<&mut Self>, enabled: bool) {
        self.mutate_config(|conn| {
            if enabled && !conn.caps.supports_angle_snapping() {
                return Err(format!(
                    "Angle Snapping is not supported by {}",
                    conn.caps.sensor_spec().name
                ));
            }
            conn.config.angle_snapping = enabled;
            Ok(format!(
                "Angle Snapping {}",
                if enabled { "enabled" } else { "disabled" }
            ))
        });
    }

    pub fn apply_ripple_control(self: Pin<&mut Self>, enabled: bool) {
        self.mutate_config(|conn| {
            if enabled && !conn.caps.supports_ripple_control() {
                return Err(format!(
                    "Ripple Control is not supported by {}",
                    conn.caps.sensor_spec().name
                ));
            }
            conn.config.ripple_control = enabled;
            Ok(format!(
                "Ripple Control {}",
                if enabled { "enabled" } else { "disabled" }
            ))
        });
    }

    pub fn apply_high_performance(self: Pin<&mut Self>, enabled: bool) {
        self.mutate_config(|conn| {
            if enabled && !conn.caps.supports_sensor_mode() {
                return Err(format!(
                    "High Performance Mode is not supported by {}",
                    conn.caps.sensor_spec().name
                ));
            }
            conn.config.high_performance = enabled;
            Ok(format!(
                "High Performance Mode {}",
                if enabled { "enabled" } else { "disabled" }
            ))
        });
    }

    pub fn factory_reset(mut self: Pin<&mut Self>) {
        let reset_res: Result<String, String> = {
            let mut rust = self.as_mut().rust_mut();
            let Some(conn) = rust.active.as_mut() else {
                return;
            };
            if let Some(dev) = conn.device.as_mut() {
                if let Err(err) = dev.factory_reset() {
                    Err(err.to_string())
                } else {
                    match dev.read_config(conn.caps.sensor) {
                        Ok(cfg) => {
                            conn.config = cfg;
                            Ok("Restored factory mouse defaults".to_string())
                        }
                        Err(err) => Err(err.to_string()),
                    }
                }
            } else {
                conn.config = MouseConfig::demo_default(conn.caps.sensor);
                Ok("Restored factory demo defaults".to_string())
            }
        };

        match reset_res {
            Ok(msg) => self.sync_ui_from_active(&msg, ""),
            Err(err) => self.as_mut().set_error_message(QString::from(&err)),
        }
    }

    pub fn poll_device(mut self: Pin<&mut Self>) {
        let mut stage_changed = false;
        let mut battery_changed = false;
        let mut went_offline = false;
        let mut new_stage_msg = String::new();

        {
            let mut rust = self.as_mut().rust_mut();
            rust.poll_ticks = rust.poll_ticks.wrapping_add(1);
            let tick = rust.poll_ticks;
            let Some(conn) = rust.active.as_mut() else {
                return;
            };
            let Some(dev) = conn.device.as_mut() else {
                return;
            };

            while let Ok(Some(_ev)) = dev.poll_nonblocking_event() {
                if let Ok(Some(stage)) = dev.read_current_dpi_stage() {
                    if stage < conn.config.dpi_stage_count && stage != conn.config.active_dpi_stage
                    {
                        conn.config.active_dpi_stage = stage;
                        let dpi = conn.config.stages[stage as usize].dpi_x;
                        new_stage_msg = format!(
                            "Hardware button switched to Stage {} ({dpi} DPI)",
                            stage + 1
                        );
                        stage_changed = true;
                    }
                }
            }

            // Refresh battery & online state every 15 seconds (30 * 500ms)
            if tick.is_multiple_of(30) {
                if let Ok(online) = dev.is_online() {
                    if !online {
                        went_offline = true;
                    } else if let Ok(bat) = dev.read_battery() {
                        conn.battery = Some(bat);
                        battery_changed = true;
                    }
                }
            }
        }

        if went_offline {
            self.as_mut().set_mouse_online(false);
            self.as_mut().set_status_message(QString::from(
                "Mouse entered sleep mode — move mouse and click Refresh",
            ));
            return;
        }

        if stage_changed {
            self.sync_ui_from_active(&new_stage_msg, "");
        } else if battery_changed {
            if let Some(bat) = self.rust().active.as_ref().and_then(|c| c.battery) {
                self.as_mut().set_battery_percent(i32::from(bat.percent));
                self.as_mut().set_battery_charging(bat.charging);
                self.as_mut().set_battery_mv(i32::from(bat.voltage_mv));
            }
        }
    }

    fn mutate_config<F>(mut self: Pin<&mut Self>, mutator: F)
    where
        F: FnOnce(&mut ActiveConnection) -> Result<String, String>,
    {
        let result: Result<(String, bool), String> = {
            let mut rust = self.as_mut().rust_mut();
            let Some(conn) = rust.active.as_mut() else {
                return;
            };
            let prev = conn.config.clone();
            match mutator(conn) {
                Ok(msg) => {
                    if let Some(dev) = conn.device.as_mut() {
                        match dev.apply_config_diff(&prev, &conn.config, &conn.caps) {
                            Ok(()) => Ok((msg, false)),
                            Err(err) => {
                                let offline = matches!(err, HidError::MouseOffline);
                                conn.config = prev;
                                if offline {
                                    Ok((
                                        "Mouse is asleep — move it and try again".to_string(),
                                        true,
                                    ))
                                } else {
                                    Err(err.to_string())
                                }
                            }
                        }
                    } else {
                        Ok((msg, false))
                    }
                }
                Err(err) => Err(err),
            }
        };

        match result {
            Ok((msg, offline)) => {
                if offline {
                    self.as_mut().set_mouse_online(false);
                    self.as_mut().set_error_message(QString::from(&msg));
                } else {
                    self.as_mut().set_mouse_online(true);
                    self.sync_ui_from_active(&msg, "");
                }
            }
            Err(err) => {
                self.as_mut().set_error_message(QString::from(&err));
            }
        }
    }

    fn sync_ui_from_active(mut self: Pin<&mut Self>, status: &str, err: &str) {
        let snapshot = {
            let rust = self.rust();
            let Some(conn) = rust.active.as_ref() else {
                return;
            };
            let spec = conn.caps.sensor_spec();
            let active_idx = (conn.config.active_dpi_stage as usize).min(7);
            let active_st = &conn.config.stages[active_idx];
            (
                conn.device_name.clone(),
                conn.caps.connection_label().to_string(),
                spec.name.to_string(),
                conn.firmware_version.clone(),
                conn.battery,
                spec.min_dpi as i32,
                conn.caps.max_dpi as i32,
                conn.caps.sensor.step_for(active_st.dpi_x) as i32,
                conn.caps.supports_independent_xy(),
                conn.caps.supports_lod(),
                spec.supports_lod_07mm,
                conn.caps.supports_motion_sync(),
                conn.caps.supports_angle_snapping(),
                conn.caps.supports_ripple_control(),
                conn.caps.supports_sensor_mode(),
                i32::from(conn.caps.max_dpi_stages),
                i32::from(conn.config.dpi_stage_count),
                active_idx as i32,
                active_st.dpi_x as i32,
                active_st.dpi_y as i32,
                format_hex(active_st.color),
                !conn.caps.supports_independent_xy() || active_st.dpi_x == active_st.dpi_y,
                serialize_stages(&conn.config, conn.caps.max_dpi_stages),
                conn.config.dpi_led_mode as i32,
                i32::from(conn.config.dpi_led_brightness),
                i32::from(conn.config.dpi_led_speed),
                conn.config.polling_rate.hz() as i32,
                conn.caps.max_polling_rate.hz() as i32,
                serialize_polling_rates(&conn.caps),
                conn.caps.polling_limit_note(),
                conn.config.lod as i32,
                i32::from(conn.config.debounce_ms),
                i32::from((conn.config.sleep_units_10s / 6).max(1)),
                conn.config.motion_sync,
                conn.config.angle_snapping,
                conn.config.ripple_control,
                conn.config.high_performance,
            )
        };

        let (
            dev_name,
            conn_mode,
            sensor_name,
            fw_ver,
            battery,
            dpi_min,
            dpi_max,
            dpi_step,
            supports_xy,
            supports_lod,
            supports_sub_mm_lod,
            supports_motion_sync,
            supports_angle_snapping,
            supports_ripple_control,
            supports_high_performance,
            max_stages,
            stage_count,
            active_stage,
            active_dpi_x,
            active_dpi_y,
            active_color,
            xy_linked,
            stages_json,
            dpi_led_mode,
            dpi_led_brightness,
            dpi_led_speed,
            polling_rate_hz,
            max_polling_rate_hz,
            supported_polling_rates_json,
            polling_rate_limit_note,
            lod_level,
            debounce_ms,
            sleep_minutes,
            motion_sync,
            angle_snapping,
            ripple_control,
            high_performance,
        ) = snapshot;

        self.as_mut().set_device_name(QString::from(&dev_name));
        self.as_mut().set_connection_mode(QString::from(&conn_mode));
        self.as_mut().set_sensor_name(QString::from(&sensor_name));
        self.as_mut().set_firmware_version(QString::from(&fw_ver));
        if let Some(bat) = battery {
            self.as_mut().set_battery_percent(i32::from(bat.percent));
            self.as_mut().set_battery_charging(bat.charging);
            self.as_mut().set_battery_mv(i32::from(bat.voltage_mv));
        } else {
            self.as_mut().set_battery_percent(-1);
            self.as_mut().set_battery_charging(false);
            self.as_mut().set_battery_mv(0);
        }
        self.as_mut().set_dpi_min(dpi_min);
        self.as_mut().set_dpi_max(dpi_max);
        self.as_mut().set_dpi_step(dpi_step);
        self.as_mut().set_supports_xy(supports_xy);
        self.as_mut().set_supports_lod(supports_lod);
        self.as_mut().set_supports_sub_mm_lod(supports_sub_mm_lod);
        self.as_mut().set_supports_motion_sync(supports_motion_sync);
        self.as_mut()
            .set_supports_angle_snapping(supports_angle_snapping);
        self.as_mut()
            .set_supports_ripple_control(supports_ripple_control);
        self.as_mut()
            .set_supports_high_performance(supports_high_performance);
        self.as_mut().set_max_stages(max_stages);
        self.as_mut().set_stage_count(stage_count);
        self.as_mut().set_active_stage(active_stage);
        self.as_mut().set_active_dpi_x(active_dpi_x);
        self.as_mut().set_active_dpi_y(active_dpi_y);
        self.as_mut().set_active_color(QString::from(&active_color));
        self.as_mut().set_xy_linked(xy_linked);
        self.as_mut().set_stages_json(QString::from(&stages_json));
        self.as_mut().set_dpi_led_mode(dpi_led_mode);
        self.as_mut().set_dpi_led_brightness(dpi_led_brightness);
        self.as_mut().set_dpi_led_speed(dpi_led_speed);
        self.as_mut().set_polling_rate_hz(polling_rate_hz);
        self.as_mut().set_max_polling_rate_hz(max_polling_rate_hz);
        self.as_mut()
            .set_supported_polling_rates_json(QString::from(&supported_polling_rates_json));
        self.as_mut()
            .set_polling_rate_limit_note(QString::from(&polling_rate_limit_note));
        self.as_mut().set_lod_level(lod_level);
        self.as_mut().set_debounce_ms(debounce_ms);
        self.as_mut().set_sleep_minutes(sleep_minutes);
        self.as_mut().set_motion_sync(motion_sync);
        self.as_mut().set_angle_snapping(angle_snapping);
        self.as_mut().set_ripple_control(ripple_control);
        self.as_mut().set_high_performance(high_performance);
        self.as_mut().set_status_message(QString::from(status));
        self.as_mut().set_error_message(QString::from(err));
    }
}
