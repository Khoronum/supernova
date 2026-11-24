//! USB HID transport (`hidapi` `hidraw` backend) and high-level HATOR device session.

use std::collections::VecDeque;
use std::ffi::CString;

use hidapi::{HidApi, HidDevice};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::{ConfigValidationError, MouseConfig};
use crate::devices::{
    ConnectionKind, DeviceCapabilities, DeviceProfile, FALLBACK_INTERFACE_NUMBER, HATOR_VID,
    VENDOR_USAGE, VENDOR_USAGE_PAGE, classify_pid, friendly_product_name, profile_for_mid,
};
use crate::protocol::{
    FLASH_TOTAL_LEN, MAX_FLASH_CHUNK, Opcode, PACKET_LEN, build_packet, build_read_flash_packet,
    build_write_flash_packets, compx_checksum, verify_packet,
};
use crate::sensor::SensorKind;

const READ_TIMEOUT_MS: i32 = 150;
const MAX_EXCHANGE_RETRIES: usize = 3;

#[derive(Debug, Error)]
pub enum HidError {
    #[error("No HATOR mouse found on USB HID")]
    DeviceNotFound,
    #[error("Permission denied opening {path} (install udev/70-hator-mouse.rules)")]
    PermissionDenied { path: String },
    #[error("Mouse is offline or asleep — move the mouse to wake it")]
    MouseOffline,
    #[error("Timed out waiting for HID response from mouse")]
    Timeout,
    #[error("HID response checksum or Report ID validation failed")]
    ChecksumMismatch,
    #[error("Configuration limit violation: {0}")]
    Validation(#[from] ConfigValidationError),
    #[error("Protocol error: {0}")]
    Protocol(String),
    #[error("HID I/O error: {0}")]
    Hid(String),
}

pub trait HidTransport {
    fn write_packet(&mut self, pkt: &[u8; PACKET_LEN]) -> Result<(), HidError>;
    fn read_packet(&mut self, timeout_ms: i32) -> Result<Option<[u8; PACKET_LEN]>, HidError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredDevice {
    pub path: String,
    pub vid: u16,
    pub pid: u16,
    pub product_name: String,
    pub manufacturer: String,
    pub connection: ConnectionKind,
    pub interface_number: i32,
    pub usage_page: u16,
    pub usage: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub cid: u8,
    pub mid: u8,
    pub device_type: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatteryStatus {
    pub percent: u8,
    pub charging: bool,
    pub voltage_mv: u16,
}

pub struct HidapiTransport {
    dev: HidDevice,
    path: String,
}

impl HidapiTransport {
    pub fn open(path: &str) -> Result<Self, HidError> {
        let api = HidApi::new().map_err(|e| HidError::Hid(e.to_string()))?;
        let c_path =
            CString::new(path).map_err(|e| HidError::Hid(format!("Invalid HID path: {e}")))?;
        let dev = api.open_path(&c_path).map_err(|e| {
            let msg = e.to_string();
            if is_permission_error(&msg) {
                HidError::PermissionDenied {
                    path: path.to_string(),
                }
            } else {
                HidError::Hid(msg)
            }
        })?;
        Ok(Self {
            dev,
            path: path.to_string(),
        })
    }

    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl HidTransport for HidapiTransport {
    fn write_packet(&mut self, pkt: &[u8; PACKET_LEN]) -> Result<(), HidError> {
        let written = self.dev.write(pkt).map_err(|e| {
            let msg = e.to_string();
            if is_permission_error(&msg) {
                HidError::PermissionDenied {
                    path: self.path.clone(),
                }
            } else {
                HidError::Hid(msg)
            }
        })?;
        if written == 0 {
            return Err(HidError::Hid("Wrote 0 bytes to HID device".to_string()));
        }
        Ok(())
    }

    fn read_packet(&mut self, timeout_ms: i32) -> Result<Option<[u8; PACKET_LEN]>, HidError> {
        let mut buf = [0u8; PACKET_LEN];
        let n = self
            .dev
            .read_timeout(&mut buf, timeout_ms)
            .map_err(|e| HidError::Hid(e.to_string()))?;
        if n == 0 {
            return Ok(None);
        }
        if n < PACKET_LEN {
            return Err(HidError::Protocol(format!(
                "Short HID read: expected {PACKET_LEN} bytes, got {n}"
            )));
        }
        Ok(Some(buf))
    }
}

fn is_permission_error(msg: &str) -> bool {
    let lower = msg.to_ascii_lowercase();
    lower.contains("permission denied")
        || lower.contains("access denied")
        || lower.contains("operation not permitted")
        || lower.contains("eacces")
        || lower.contains("eperm")
}

/// Enumerate all connected HATOR / Compx vendor control HID interfaces (`VID 0x379A`).
pub fn discover_devices() -> Result<Vec<DiscoveredDevice>, HidError> {
    let api = HidApi::new().map_err(|e| HidError::Hid(e.to_string()))?;
    let mut candidates = Vec::new();
    let mut fallback_candidates = Vec::new();

    for info in api.device_list() {
        if info.vendor_id() != HATOR_VID {
            continue;
        }
        let path = info.path().to_string_lossy().into_owned();
        let pid = info.product_id();
        let raw_prod = info.product_string().unwrap_or("");
        let mfg = info
            .manufacturer_string()
            .unwrap_or("Compx / HATOR")
            .to_string();
        let dev = DiscoveredDevice {
            path,
            vid: info.vendor_id(),
            pid,
            product_name: friendly_product_name(pid, raw_prod),
            manufacturer: mfg,
            connection: classify_pid(pid),
            interface_number: info.interface_number(),
            usage_page: info.usage_page(),
            usage: info.usage(),
        };

        if dev.usage_page == VENDOR_USAGE_PAGE && dev.usage == VENDOR_USAGE {
            if !candidates
                .iter()
                .any(|d: &DiscoveredDevice| d.path == dev.path)
            {
                candidates.push(dev);
            }
        } else if dev.interface_number == FALLBACK_INTERFACE_NUMBER
            && !fallback_candidates
                .iter()
                .any(|d: &DiscoveredDevice| d.path == dev.path)
        {
            fallback_candidates.push(dev);
        }
    }

    if candidates.is_empty() {
        Ok(fallback_candidates)
    } else {
        Ok(candidates)
    }
}

pub struct HatorDevice<T: HidTransport> {
    transport: T,
    pending_events: VecDeque<[u8; PACKET_LEN]>,
}

impl<T: HidTransport> HatorDevice<T> {
    #[must_use]
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            pending_events: VecDeque::new(),
        }
    }

    pub fn into_inner(self) -> T {
        self.transport
    }

    pub fn pop_event(&mut self) -> Option<[u8; PACKET_LEN]> {
        self.pending_events.pop_front()
    }

    /// Send a 17-byte request and wait for the matching validated response, queuing any
    /// unsolicited `0x0A` notification events encountered along the way.
    pub fn exchange(&mut self, req: &[u8; PACKET_LEN]) -> Result<[u8; PACKET_LEN], HidError> {
        let mut last_err = HidError::Timeout;
        for _attempt in 0..MAX_EXCHANGE_RETRIES {
            self.transport.write_packet(req)?;

            // Read up to several packets per attempt in case unsolicited 0x0A events precede our reply.
            for _reads in 0..6 {
                let Some(resp) = self.transport.read_packet(READ_TIMEOUT_MS)? else {
                    last_err = HidError::Timeout;
                    break;
                };

                if !verify_packet(&resp) {
                    last_err = HidError::ChecksumMismatch;
                    continue;
                }

                if resp[1] == Opcode::EventNotify as u8 {
                    self.pending_events.push_back(resp);
                    continue;
                }

                if resp[1] != req[1] {
                    last_err = HidError::Protocol(format!(
                        "Unexpected reply opcode 0x{:02X} for request 0x{:02X}",
                        resp[1], req[1]
                    ));
                    continue;
                }

                if req[1] == Opcode::WriteFlash as u8 && resp != *req {
                    last_err = HidError::Protocol("WriteFlash echo mismatch".to_string());
                    continue;
                }

                return Ok(resp);
            }
        }
        Err(last_err)
    }

    /// Read device CID/MID/type (`0x01`). Uses fixed challenge bytes `[0x48, 0x41, 0x54, 0x52]` (`HATR`).
    pub fn read_device_info(&mut self) -> Result<DeviceInfo, HidError> {
        let mut pkt = [0u8; PACKET_LEN];
        pkt[0] = crate::protocol::REPORT_ID;
        pkt[1] = Opcode::ReadDeviceInfo as u8;
        pkt[5] = 0x08;
        pkt[6] = 0x48;
        pkt[7] = 0x41;
        pkt[8] = 0x54;
        pkt[9] = 0x52;
        pkt[16] = compx_checksum(&pkt[..16]);

        let resp = self.exchange(&pkt)?;
        Ok(DeviceInfo {
            cid: resp[10],
            mid: resp[11],
            device_type: resp[12],
        })
    }

    pub fn profile(&mut self) -> Result<(DeviceInfo, DeviceProfile), HidError> {
        let info = self.read_device_info()?;
        Ok((info, profile_for_mid(info.mid)))
    }

    pub fn capabilities(
        &mut self,
        meta: &DiscoveredDevice,
        sensor_override: Option<SensorKind>,
    ) -> Result<(DeviceInfo, DeviceCapabilities), HidError> {
        let (info, profile) = self.profile()?;
        let sensor = sensor_override.unwrap_or(profile.sensor);
        let caps = DeviceCapabilities::resolve(
            profile,
            sensor,
            info.device_type,
            meta.connection,
            &meta.product_name,
        );
        Ok((info, caps))
    }

    pub fn set_pc_driver_status(&mut self, active: bool) -> Result<(), HidError> {
        let pkt = build_packet(Opcode::SetPcDriverStatus, 0, &[u8::from(active)]);
        let _ = self.exchange(&pkt)?;
        Ok(())
    }

    pub fn is_online(&mut self) -> Result<bool, HidError> {
        let pkt = build_packet(Opcode::ReadOnline, 0, &[]);
        let resp = self.exchange(&pkt)?;
        Ok(resp[6] > 0)
    }

    pub fn read_battery(&mut self) -> Result<BatteryStatus, HidError> {
        let pkt = build_packet(Opcode::ReadBattery, 0, &[]);
        let resp = self.exchange(&pkt)?;
        Ok(BatteryStatus {
            percent: resp[6].min(100),
            charging: resp[7] != 0,
            voltage_mv: u16::from_be_bytes([resp[8], resp[9]]),
        })
    }

    pub fn read_firmware_version(&mut self) -> Result<u16, HidError> {
        let pkt = build_packet(Opcode::ReadVersion, 0, &[]);
        let resp = self.exchange(&pkt)?;
        Ok(u16::from_be_bytes([resp[6], resp[7]]))
    }

    pub fn read_slave_version(&mut self) -> Result<u16, HidError> {
        let pkt = build_packet(Opcode::ReadSlaveVersion, 0, &[]);
        let resp = self.exchange(&pkt)?;
        Ok(u16::from_be_bytes([resp[6], resp[7]]))
    }

    pub fn read_flash(&mut self, addr: u16, len: usize) -> Result<Vec<u8>, HidError> {
        let mut out = Vec::with_capacity(len);
        let mut offset = 0usize;
        while offset < len {
            let chunk = (len - offset).min(MAX_FLASH_CHUNK) as u8;
            let pkt = build_read_flash_packet(addr + offset as u16, chunk);
            let resp = self.exchange(&pkt)?;
            out.extend_from_slice(&resp[6..6 + chunk as usize]);
            offset += chunk as usize;
        }
        Ok(out)
    }

    pub fn read_all_flash(&mut self) -> Result<[u8; FLASH_TOTAL_LEN], HidError> {
        let bytes = self.read_flash(0x0000, FLASH_TOTAL_LEN)?;
        let mut flash = [0u8; FLASH_TOTAL_LEN];
        flash.copy_from_slice(&bytes);
        Ok(flash)
    }

    pub fn write_flash(&mut self, addr: u16, data: &[u8]) -> Result<(), HidError> {
        for pkt in build_write_flash_packets(addr, data) {
            let _ = self.exchange(&pkt)?;
        }
        Ok(())
    }

    pub fn read_config(&mut self, sensor: SensorKind) -> Result<MouseConfig, HidError> {
        if !self.is_online()? {
            return Err(HidError::MouseOffline);
        }
        let flash = self.read_all_flash()?;
        Ok(MouseConfig::from_flash(&flash, sensor))
    }

    pub fn apply_config_diff(
        &mut self,
        old: &MouseConfig,
        new: &MouseConfig,
        caps: &DeviceCapabilities,
    ) -> Result<(), HidError> {
        old.validate_diff(new, caps)?;
        let writes = old.diff_writes(new, caps.sensor);
        if writes.is_empty() {
            return Ok(());
        }
        if !self.is_online()? {
            return Err(HidError::MouseOffline);
        }
        for (addr, payload) in writes {
            self.write_flash(addr, &payload)?;
        }
        Ok(())
    }

    pub fn read_current_dpi_stage(&mut self) -> Result<Option<u8>, HidError> {
        let bytes = self.read_flash(0x0004, 2)?;
        if compx_checksum(&[bytes[0]]) == bytes[1] {
            Ok(Some(bytes[0]))
        } else {
            Ok(None)
        }
    }

    pub fn factory_reset(&mut self) -> Result<(), HidError> {
        let pkt = build_packet(Opcode::ClearSetting, 0, &[]);
        let _ = self.exchange(&pkt)?;
        Ok(())
    }

    /// Non-blocking poll for any unsolicited `0x0A` hardware notifications (e.g. DPI button).
    pub fn poll_nonblocking_event(&mut self) -> Result<Option<[u8; PACKET_LEN]>, HidError> {
        if let Some(ev) = self.pending_events.pop_front() {
            return Ok(Some(ev));
        }
        let Some(pkt) = self.transport.read_packet(0)? else {
            return Ok(None);
        };
        if verify_packet(&pkt) && pkt[1] == Opcode::EventNotify as u8 {
            return Ok(Some(pkt));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PollingRate;

    struct MockTransport {
        flash: [u8; FLASH_TOTAL_LEN],
        online: bool,
        battery: BatteryStatus,
        info: DeviceInfo,
        version: u16,
        queued_reads: VecDeque<[u8; PACKET_LEN]>,
        written_packets: Vec<[u8; PACKET_LEN]>,
    }

    impl MockTransport {
        fn new(sensor: SensorKind) -> Self {
            let cfg = MouseConfig::demo_default(sensor);
            let mut flash = [0xFFu8; FLASH_TOTAL_LEN];
            let base = MouseConfig {
                polling_rate: PollingRate::Hz125,
                dpi_stage_count: 1,
                active_dpi_stage: 0,
                ..cfg.clone()
            };
            // Populate flash via diff_writes from a differing base
            for (addr, bytes) in base.diff_writes(&cfg, sensor) {
                let start = addr as usize;
                flash[start..start + bytes.len()].copy_from_slice(&bytes);
            }
            // Also write polling rate explicitly
            flash[0x00] = cfg.polling_rate as u8;
            flash[0x01] = compx_checksum(&[cfg.polling_rate as u8]);
            flash[0x02] = cfg.dpi_stage_count;
            flash[0x03] = compx_checksum(&[cfg.dpi_stage_count]);
            flash[0x04] = cfg.active_dpi_stage;
            flash[0x05] = compx_checksum(&[cfg.active_dpi_stage]);

            Self {
                flash,
                online: true,
                battery: BatteryStatus {
                    percent: 87,
                    charging: false,
                    voltage_mv: 4050,
                },
                info: DeviceInfo {
                    cid: 40,
                    mid: 8,
                    device_type: 1,
                },
                version: 0x0105,
                queued_reads: VecDeque::new(),
                written_packets: Vec::new(),
            }
        }
    }

    impl HidTransport for MockTransport {
        fn write_packet(&mut self, pkt: &[u8; PACKET_LEN]) -> Result<(), HidError> {
            assert!(verify_packet(pkt));
            self.written_packets.push(*pkt);

            let mut resp = [0u8; PACKET_LEN];
            resp[0] = crate::protocol::REPORT_ID;
            resp[1] = pkt[1];

            match Opcode::from_byte(pkt[1]) {
                Some(Opcode::ReadDeviceInfo) => {
                    resp[5] = 8;
                    resp[6..10].copy_from_slice(&pkt[6..10]);
                    resp[10] = self.info.cid;
                    resp[11] = self.info.mid;
                    resp[12] = self.info.device_type;
                }
                Some(Opcode::ReadOnline) => {
                    resp[5] = 1;
                    resp[6] = u8::from(self.online);
                }
                Some(Opcode::ReadBattery) => {
                    resp[5] = 4;
                    resp[6] = self.battery.percent;
                    resp[7] = u8::from(self.battery.charging);
                    resp[8..10].copy_from_slice(&self.battery.voltage_mv.to_be_bytes());
                }
                Some(Opcode::ReadVersion) => {
                    resp[5] = 2;
                    resp[6..8].copy_from_slice(&self.version.to_be_bytes());
                }
                Some(Opcode::ReadFlash) => {
                    let addr = u16::from_be_bytes([pkt[3], pkt[4]]) as usize;
                    let len = pkt[5] as usize;
                    resp[3] = pkt[3];
                    resp[4] = pkt[4];
                    resp[5] = pkt[5];
                    resp[6..6 + len].copy_from_slice(&self.flash[addr..addr + len]);
                }
                Some(Opcode::WriteFlash) => {
                    let addr = u16::from_be_bytes([pkt[3], pkt[4]]) as usize;
                    let len = pkt[5] as usize;
                    self.flash[addr..addr + len].copy_from_slice(&pkt[6..6 + len]);
                    resp = *pkt;
                }
                _ => {}
            }

            resp[16] = compx_checksum(&resp[..16]);
            self.queued_reads.push_back(resp);
            Ok(())
        }

        fn read_packet(&mut self, _timeout_ms: i32) -> Result<Option<[u8; PACKET_LEN]>, HidError> {
            Ok(self.queued_reads.pop_front())
        }
    }

    #[test]
    fn mock_device_end_to_end_and_unsolicited_event() {
        let sensor = SensorKind::Paw3950;
        let mut transport = MockTransport::new(sensor);

        // Inject an unsolicited 0x0A event before the first reply
        let mut notify = [0u8; PACKET_LEN];
        notify[0] = crate::protocol::REPORT_ID;
        notify[1] = Opcode::EventNotify as u8;
        notify[6] = 0x40;
        notify[16] = compx_checksum(&notify[..16]);
        transport.queued_reads.push_back(notify);

        let mut dev = HatorDevice::new(transport);
        let info = dev.read_device_info().unwrap();
        assert_eq!(info.cid, 40);
        assert_eq!(info.mid, 8);
        assert!(dev.pop_event().is_some());

        let bat = dev.read_battery().unwrap();
        assert_eq!(bat.percent, 87);
        assert_eq!(bat.voltage_mv, 4050);

        let cfg = dev.read_config(sensor).unwrap();
        assert_eq!(cfg.active_dpi_stage, 2);

        let caps = DeviceCapabilities::resolve(
            profile_for_mid(8),
            sensor,
            5,
            ConnectionKind::Wireless8K,
            "HATOR Quasar 3 Ultra 8K+8K Wireless",
        );
        let mut next = cfg.clone();
        next.active_dpi_stage = 3;
        next.set_stage_dpi(3, 6400, 6400, sensor);
        dev.apply_config_diff(&cfg, &next, &caps).unwrap();

        let reread = dev.read_config(sensor).unwrap();
        assert_eq!(reread.active_dpi_stage, 3);
        assert_eq!(reread.stages[3].dpi_x, 6400);
        assert_eq!(dev.read_current_dpi_stage().unwrap(), Some(3));
    }

    #[test]
    fn apply_config_diff_rejects_8k_on_1k_device_with_zero_writes() {
        let sensor = SensorKind::Paw3950;
        let transport = MockTransport::new(sensor);
        let mut dev = HatorDevice::new(transport);
        let cfg = dev.read_config(sensor).unwrap();
        let written_before = dev.into_inner();

        let mut transport2 = MockTransport::new(sensor);
        transport2.info.device_type = 0; // Wireless 1K
        let mut dev2 = HatorDevice::new(transport2);
        let caps_1k = DeviceCapabilities::resolve(
            profile_for_mid(8),
            sensor,
            0,
            ConnectionKind::Wireless24G,
            "HATOR Quasar 3 Pro Wireless",
        );
        let mut next = cfg.clone();
        next.polling_rate = PollingRate::Hz8000;

        let err = dev2.apply_config_diff(&cfg, &next, &caps_1k).unwrap_err();
        assert!(matches!(
            err,
            HidError::Validation(ConfigValidationError::UnsupportedPollingRate {
                requested_hz: 8000,
                max_hz: 1000,
                ..
            })
        ));
        let inner = dev2.into_inner();
        assert!(
            inner.written_packets.is_empty(),
            "Validation failure must abort before sending any HID packets"
        );
        drop(written_before);
    }
}
