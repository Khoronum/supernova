pub mod config;
pub mod devices;
pub mod hid;
pub mod protocol;
pub mod sensor;

pub use config::{
    ConfigValidationError, DpiLedMode, DpiStage, LodLevel, MAX_DEBOUNCE_MS, MAX_LED_SPEED,
    MAX_SLEEP_MINUTES, MIN_DEBOUNCE_MS, MIN_LED_SPEED, MIN_SLEEP_MINUTES, MouseConfig, PollingRate,
    SensorMode,
};
pub use devices::{
    ConnectionKind, DeviceCapabilities, DeviceProfile, HATOR_VID, KNOWN_HATOR_MODELS, LinkType,
    ModelSpec,
};
pub use hid::{
    BatteryStatus, DeviceInfo, DiscoveredDevice, HatorDevice, HidError, HidTransport,
    HidapiTransport, discover_devices,
};
pub use protocol::{
    FLASH_TOTAL_LEN, MAX_FLASH_CHUNK, Opcode, PACKET_LEN, REPORT_ID, compx_checksum,
};
pub use sensor::{SensorKind, SensorSpec};
