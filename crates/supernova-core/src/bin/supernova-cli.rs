//! `supernova-cli` CLI for inspecting and configuring HATOR mice over Linux `hidraw`.

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use supernova_core::{
    DeviceCapabilities, DeviceInfo, HatorDevice, HidError, HidapiTransport, LodLevel, MouseConfig,
    PollingRate, SensorKind, discover_devices,
};

#[derive(Parser)]
#[command(
    name = "supernova-cli",
    about = "Supernova CLI — configure HATOR gaming mice on Linux"
)]
struct Cli {
    /// Override detected sensor (e.g. paw3950, paw3395, paw3311, paw3335)
    #[arg(long)]
    sensor: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List detected HATOR / Compx USB HID control interfaces
    List,
    /// Show connected mouse status, battery, firmware, polling rate, and DPI stages
    Status,
    /// Manage mouse DPI settings
    Dpi {
        #[command(subcommand)]
        action: DpiAction,
    },
    /// Set USB polling rate in Hz (125, 250, 500, 1000, 2000, 4000, 8000)
    Rate { hz: u32 },
    /// Restore factory mouse settings stored in onboard flash
    Reset {
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Subcommand)]
enum DpiAction {
    /// Read current DPI stages and active stage from the mouse
    Get,
    /// Switch active DPI stage (1..=8)
    Stage { stage: u8 },
    /// Set DPI (and optional Y DPI / indicator color) for a stage (1..=8)
    Set {
        stage: u8,
        dpi: u32,
        #[arg(long)]
        dpi_y: Option<u32>,
        #[arg(long)]
        color: Option<String>,
    },
    /// Set how many DPI stages are enabled (1..=8)
    Count { count: u8 },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

fn parse_sensor_override(s: &str) -> Result<SensorKind, String> {
    match s.trim().to_ascii_lowercase().as_str() {
        "3950" | "paw3950" => Ok(SensorKind::Paw3950),
        "3395" | "paw3395" => Ok(SensorKind::Paw3395),
        "3311" | "paw3311" => Ok(SensorKind::Paw3311 {
            max_dpi: 24_000,
            ext_flag: 0x33,
        }),
        "3335" | "paw3335" => Ok(SensorKind::Paw3335),
        "3325" | "paw3325" => Ok(SensorKind::Paw3325),
        "3370" | "paw3370" => Ok(SensorKind::Paw3370),
        _ => Err(format!("Unknown sensor override '{s}'")),
    }
}

fn parse_hex_color(hex: &str) -> Result<[u8; 3], String> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() != 6 {
        return Err(format!("Expected #RRGGBB color, got '{hex}'"));
    }
    let r = u8::from_str_radix(&clean[0..2], 16).map_err(|e| e.to_string())?;
    let g = u8::from_str_radix(&clean[2..4], 16).map_err(|e| e.to_string())?;
    let b = u8::from_str_radix(&clean[4..6], 16).map_err(|e| e.to_string())?;
    Ok([r, g, b])
}

fn open_first_mouse(
    sensor_override: Option<&str>,
) -> Result<
    (
        supernova_core::DiscoveredDevice,
        HatorDevice<HidapiTransport>,
        DeviceInfo,
        DeviceCapabilities,
    ),
    String,
> {
    let devices = discover_devices().map_err(|e| e.to_string())?;
    let dev_meta = devices
        .into_iter()
        .next()
        .ok_or_else(|| HidError::DeviceNotFound.to_string())?;

    let transport = HidapiTransport::open(&dev_meta.path).map_err(|e| e.to_string())?;
    let mut dev = HatorDevice::new(transport);
    let sensor_ov = match sensor_override {
        Some(raw) => Some(parse_sensor_override(raw)?),
        None => None,
    };
    let (info, caps) = dev
        .capabilities(&dev_meta, sensor_ov)
        .map_err(|e| e.to_string())?;
    Ok((dev_meta, dev, info, caps))
}

fn print_dpi_summary(cfg: &MouseConfig, caps: &DeviceCapabilities) {
    let spec = caps.sensor_spec();
    println!(
        "Sensor: {} ({}..={} DPI) | Active Stage: {}/{} (max {})",
        spec.name,
        spec.min_dpi,
        caps.max_dpi,
        cfg.active_dpi_stage + 1,
        cfg.dpi_stage_count,
        caps.max_dpi_stages
    );
    for idx in 0..cfg.dpi_stage_count as usize {
        let st = &cfg.stages[idx];
        let marker = if idx as u8 == cfg.active_dpi_stage {
            "*"
        } else {
            " "
        };
        if st.dpi_x == st.dpi_y {
            println!(
                "  {marker} Stage {}: {:>5} DPI   #{:02X}{:02X}{:02X}",
                idx + 1,
                st.dpi_x,
                st.color[0],
                st.color[1],
                st.color[2]
            );
        } else {
            println!(
                "  {marker} Stage {}: {:>5} x {:<5} DPI   #{:02X}{:02X}{:02X}",
                idx + 1,
                st.dpi_x,
                st.dpi_y,
                st.color[0],
                st.color[1],
                st.color[2]
            );
        }
    }
}

fn run(cli: Cli) -> Result<(), String> {
    match cli.command {
        Commands::List => {
            let devs = discover_devices().map_err(|e| e.to_string())?;
            if devs.is_empty() {
                println!("No HATOR / Compx mice detected (VID 0x379A).");
                return Ok(());
            }
            for (idx, d) in devs.iter().enumerate() {
                println!(
                    "[{}] {} ({:04X}:{:04X}) | {} | if={} usage_page=0x{:04X} usage=0x{:04X} | path={}",
                    idx,
                    d.product_name,
                    d.vid,
                    d.pid,
                    d.connection.label(),
                    d.interface_number,
                    d.usage_page,
                    d.usage,
                    d.path
                );
            }
            Ok(())
        }
        Commands::Status => {
            let (meta, mut dev, info, caps) = open_first_mouse(cli.sensor.as_deref())?;
            let online = dev.is_online().map_err(|e| e.to_string())?;
            let fw = dev.read_firmware_version().map_err(|e| e.to_string())?;
            println!(
                "Device: {} ({:04X}:{:04X}) [{}] CID={} MID={} Type={} FW=0x{:04X}",
                meta.product_name,
                meta.vid,
                meta.pid,
                caps.connection_label(),
                info.cid,
                info.mid,
                info.device_type,
                fw
            );
            if let Some(model) = caps.model {
                println!(
                    "Model Spec: {} ({}) | Sensor: {} (max {} DPI)",
                    model.name,
                    model.skus.join(", "),
                    model.sensor_name,
                    model.max_dpi
                );
            }
            let supported_rates = caps
                .supported_polling_rates()
                .iter()
                .map(|r| format!("{} Hz", r.hz()))
                .collect::<Vec<_>>()
                .join(", ");
            let lod_cap = if caps.supports_lod_level(LodLevel::Mm07) {
                "0.7/1.0/2.0 mm"
            } else if caps.supports_lod() {
                "1.0/2.0 mm"
            } else {
                "unsupported"
            };
            println!(
                "Capabilities: MaxRate={} Hz ({}) | MaxDPI={} | MaxStages={} | XY={} | LOD={} | MotionSync={}",
                caps.max_polling_rate.hz(),
                supported_rates,
                caps.max_dpi,
                caps.max_dpi_stages,
                caps.supports_independent_xy(),
                lod_cap,
                caps.supports_motion_sync()
            );
            println!("Online: {}", if online { "yes" } else { "no (asleep)" });
            if !online {
                return Err(HidError::MouseOffline.to_string());
            }
            if let Ok(bat) = dev.read_battery() {
                println!(
                    "Battery: {}% ({} mV){}",
                    bat.percent,
                    bat.voltage_mv,
                    if bat.charging { " [charging]" } else { "" }
                );
            }
            let cfg = dev.read_config(caps.sensor).map_err(|e| e.to_string())?;
            println!(
                "Polling Rate: {} Hz | LOD: {} | Debounce: {} ms | MotionSync: {} | AngleSnap: {} | Ripple: {}",
                cfg.polling_rate.hz(),
                cfg.lod.label(),
                cfg.debounce_ms,
                cfg.motion_sync,
                cfg.angle_snapping,
                cfg.ripple_control
            );
            if cfg.polling_rate > caps.max_polling_rate {
                println!(
                    "WARNING: Onboard polling rate ({} Hz) exceeds current connection limit ({} Hz for {}).",
                    cfg.polling_rate.hz(),
                    caps.max_polling_rate.hz(),
                    caps.connection_label()
                );
            }
            print_dpi_summary(&cfg, &caps);
            Ok(())
        }
        Commands::Dpi { action } => {
            let (_meta, mut dev, _info, caps) = open_first_mouse(cli.sensor.as_deref())?;
            let old = dev.read_config(caps.sensor).map_err(|e| e.to_string())?;
            match action {
                DpiAction::Get => {
                    print_dpi_summary(&old, &caps);
                    Ok(())
                }
                DpiAction::Stage { stage } => {
                    if stage == 0 || stage > old.dpi_stage_count {
                        return Err(format!(
                            "Stage must be in 1..={} (currently enabled count)",
                            old.dpi_stage_count
                        ));
                    }
                    let mut next = old.clone();
                    next.active_dpi_stage = stage - 1;
                    dev.apply_config_diff(&old, &next, &caps)
                        .map_err(|e| e.to_string())?;
                    println!(
                        "Active DPI stage switched to {} ({} DPI)",
                        stage,
                        next.stages[(stage - 1) as usize].dpi_x
                    );
                    Ok(())
                }
                DpiAction::Set {
                    stage,
                    dpi,
                    dpi_y,
                    color,
                } => {
                    if !(1..=caps.max_dpi_stages).contains(&stage) {
                        return Err(format!(
                            "Stage {stage} is out of range (valid: 1..={})",
                            caps.max_dpi_stages
                        ));
                    }
                    let spec = caps.sensor_spec();
                    if dpi < spec.min_dpi || dpi > caps.max_dpi {
                        return Err(format!(
                            "DPI {dpi} is outside {} supported range ({}..={})",
                            spec.name, spec.min_dpi, caps.max_dpi
                        ));
                    }
                    let target_y = match dpi_y {
                        Some(y) => {
                            if !caps.supports_independent_xy() && y != dpi {
                                return Err(format!(
                                    "Independent X/Y DPI is not supported by {}",
                                    spec.name
                                ));
                            }
                            if y < spec.min_dpi || y > caps.max_dpi {
                                return Err(format!(
                                    "Y DPI {y} is outside {} supported range ({}..={})",
                                    spec.name, spec.min_dpi, caps.max_dpi
                                ));
                            }
                            y
                        }
                        None => dpi,
                    };
                    let idx = (stage - 1) as usize;
                    let mut next = old.clone();
                    next.set_stage_dpi(idx, dpi, target_y, caps.sensor);
                    if let Some(hex) = color {
                        next.stages[idx].color = parse_hex_color(&hex)?;
                    }
                    dev.apply_config_diff(&old, &next, &caps)
                        .map_err(|e| e.to_string())?;
                    let st = &next.stages[idx];
                    println!(
                        "Stage {} updated to {}x{} DPI (#{:02X}{:02X}{:02X})",
                        stage, st.dpi_x, st.dpi_y, st.color[0], st.color[1], st.color[2]
                    );
                    Ok(())
                }
                DpiAction::Count { count } => {
                    if !(1..=caps.max_dpi_stages).contains(&count) {
                        return Err(format!(
                            "Stage count {count} is out of range (valid: 1..={})",
                            caps.max_dpi_stages
                        ));
                    }
                    let mut next = old.clone();
                    next.dpi_stage_count = count;
                    if next.active_dpi_stage >= count {
                        next.active_dpi_stage = count - 1;
                    }
                    dev.apply_config_diff(&old, &next, &caps)
                        .map_err(|e| e.to_string())?;
                    println!("Enabled DPI stages set to {count}");
                    Ok(())
                }
            }
        }
        Commands::Rate { hz } => {
            let rate = PollingRate::from_hz(hz).ok_or_else(|| {
                format!(
                    "Unsupported polling rate {hz}; valid: 125, 250, 500, 1000, 2000, 4000, 8000"
                )
            })?;
            let (_meta, mut dev, _info, caps) = open_first_mouse(cli.sensor.as_deref())?;
            if !caps.supports_polling_rate(rate) {
                let supported = caps
                    .supported_polling_rates()
                    .iter()
                    .map(|r| format!("{} Hz", r.hz()))
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(format!(
                    "Polling rate {hz} Hz is not supported on {} (max {} Hz; supported: {supported})",
                    caps.connection_label(),
                    caps.max_polling_rate.hz(),
                ));
            }
            let old = dev.read_config(caps.sensor).map_err(|e| e.to_string())?;
            let mut next = old.clone();
            next.polling_rate = rate;
            dev.apply_config_diff(&old, &next, &caps)
                .map_err(|e| e.to_string())?;
            println!("Polling rate set to {hz} Hz");
            Ok(())
        }
        Commands::Reset { yes } => {
            if !yes {
                return Err("Pass --yes to confirm factory reset".to_string());
            }
            let (_meta, mut dev, _info, _caps) = open_first_mouse(cli.sensor.as_deref())?;
            dev.factory_reset().map_err(|e| e.to_string())?;
            println!("Mouse settings reset to factory defaults");
            Ok(())
        }
    }
}
