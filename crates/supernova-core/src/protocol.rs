//! Low-level 17-byte Compx/HATOR USB HID packet framing and checksums.

/// Vendor HID Report ID used for all configuration & telemetry transfers.
pub const REPORT_ID: u8 = 0x08;

/// Total size of each HID output/input packet in bytes (including Report ID).
pub const PACKET_LEN: usize = 17;

/// Maximum number of flash payload bytes transferred per ReadFlash / WriteFlash packet.
pub const MAX_FLASH_CHUNK: usize = 10;

/// Total onboard configuration flash region size read by MouseHUB (`0x00..0xC0`).
pub const FLASH_TOTAL_LEN: usize = 0xC0;

/// Protocol command opcodes at packet byte index `1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    ReadDeviceInfo = 0x01,
    SetPcDriverStatus = 0x02,
    ReadOnline = 0x03,
    ReadBattery = 0x04,
    WriteFlash = 0x07,
    ReadFlash = 0x08,
    ClearSetting = 0x09,
    EventNotify = 0x0A,
    ReadProfile = 0x0E,
    SetProfile = 0x0F,
    ReadVersion = 0x12,
    ReadSlaveVersion = 0xB3,
}

impl Opcode {
    #[must_use]
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x01 => Some(Self::ReadDeviceInfo),
            0x02 => Some(Self::SetPcDriverStatus),
            0x03 => Some(Self::ReadOnline),
            0x04 => Some(Self::ReadBattery),
            0x07 => Some(Self::WriteFlash),
            0x08 => Some(Self::ReadFlash),
            0x09 => Some(Self::ClearSetting),
            0x0A => Some(Self::EventNotify),
            0x0E => Some(Self::ReadProfile),
            0x0F => Some(Self::SetProfile),
            0x12 => Some(Self::ReadVersion),
            0xB3 => Some(Self::ReadSlaveVersion),
            _ => None,
        }
    }
}

/// Compx checksum: `0x55u8.wrapping_sub(sum(bytes))`.
///
/// Used both for 17-byte HID packets (computed over `pkt[0..16]`, including Report ID `0x08`)
/// and for individual onboard flash configuration fields.
#[must_use]
pub fn compx_checksum(bytes: &[u8]) -> u8 {
    let sum = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    0x55u8.wrapping_sub(sum)
}

/// Build a 17-byte HID command packet for the given opcode, address, and payload (max 10 bytes).
///
/// # Panics
/// Panics if `data.len() > MAX_FLASH_CHUNK`.
#[must_use]
pub fn build_packet(opcode: Opcode, addr: u16, data: &[u8]) -> [u8; PACKET_LEN] {
    assert!(
        data.len() <= MAX_FLASH_CHUNK,
        "HID payload length {} exceeds MAX_FLASH_CHUNK ({})",
        data.len(),
        MAX_FLASH_CHUNK
    );

    let mut pkt = [0u8; PACKET_LEN];
    pkt[0] = REPORT_ID;
    pkt[1] = opcode as u8;
    pkt[2] = 0x00;
    pkt[3] = (addr >> 8) as u8;
    pkt[4] = (addr & 0xFF) as u8;
    pkt[5] = data.len() as u8;
    pkt[6..6 + data.len()].copy_from_slice(data);
    pkt[16] = compx_checksum(&pkt[..16]);
    pkt
}

/// Build a `ReadFlash` (`0x08`) request packet; `pkt[5]` specifies the byte count to read (`1..=10`).
#[must_use]
pub fn build_read_flash_packet(addr: u16, len: u8) -> [u8; PACKET_LEN] {
    assert!(
        (1..=MAX_FLASH_CHUNK as u8).contains(&len),
        "ReadFlash length {} must be in 1..={}",
        len,
        MAX_FLASH_CHUNK
    );

    let mut pkt = [0u8; PACKET_LEN];
    pkt[0] = REPORT_ID;
    pkt[1] = Opcode::ReadFlash as u8;
    pkt[2] = 0x00;
    pkt[3] = (addr >> 8) as u8;
    pkt[4] = (addr & 0xFF) as u8;
    pkt[5] = len;
    pkt[16] = compx_checksum(&pkt[..16]);
    pkt
}

/// Chunk an arbitrary flash write `(addr, data)` into `WriteFlash` (`0x07`) packets of <= 10 bytes.
#[must_use]
pub fn build_write_flash_packets(addr: u16, data: &[u8]) -> Vec<[u8; PACKET_LEN]> {
    let mut packets = Vec::new();
    let mut offset = 0usize;
    while offset < data.len() {
        let chunk_len = (data.len() - offset).min(MAX_FLASH_CHUNK);
        let chunk_addr = addr.wrapping_add(offset as u16);
        packets.push(build_packet(
            Opcode::WriteFlash,
            chunk_addr,
            &data[offset..offset + chunk_len],
        ));
        offset += chunk_len;
    }
    packets
}

/// Verify Report ID (`0x08`) and Compx checksum byte at index 16.
#[must_use]
pub fn verify_packet(pkt: &[u8; PACKET_LEN]) -> bool {
    pkt[0] == REPORT_ID && pkt[16] == compx_checksum(&pkt[..16])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksum_known_packets() {
        // ReadOnline (0x08, 0x03, ...zeros): sum = 0x0B -> 0x55 - 0x0B = 0x4A
        let pkt = build_packet(Opcode::ReadOnline, 0, &[]);
        assert_eq!(pkt[0], 0x08);
        assert_eq!(pkt[1], 0x03);
        assert_eq!(pkt[16], 0x4A);
        assert!(verify_packet(&pkt));

        // ReadBattery (0x08, 0x04, ...zeros): sum = 0x0C -> 0x55 - 0x0C = 0x49
        let bat = build_packet(Opcode::ReadBattery, 0, &[]);
        assert_eq!(bat[16], 0x49);
        assert!(verify_packet(&bat));
    }

    #[test]
    fn write_flash_chunking() {
        let payload: Vec<u8> = (0..25).collect();
        let pkts = build_write_flash_packets(0x0010, &payload);
        assert_eq!(pkts.len(), 3);
        assert_eq!(pkts[0][3..6], [0x00, 0x10, 10]);
        assert_eq!(pkts[1][3..6], [0x00, 0x1A, 10]);
        assert_eq!(pkts[2][3..6], [0x00, 0x24, 5]);
        assert!(pkts.iter().all(verify_packet));
    }
}
