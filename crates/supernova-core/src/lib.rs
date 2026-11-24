pub mod config;
pub mod devices;
pub mod hid;
pub mod protocol;
pub mod sensor;

pub use config::{DpiLedMode, DpiStage, LodLevel, MouseConfig, PollingRate, SensorMode};
pub use devices::{ConnectionKind, DeviceProfile, HATOR_VID};
pub use hid::{
    BatteryStatus, DeviceInfo, DiscoveredDevice, HatorDevice, HidError, HidTransport,
    HidapiTransport, discover_devices,
};
pub use protocol::{
    FLASH_TOTAL_LEN, MAX_FLASH_CHUNK, Opcode, PACKET_LEN, REPORT_ID, compx_checksum,
};
pub use sensor::{SensorKind, SensorSpec};
