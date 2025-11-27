use crate::byte_formatter::byte_formatter::ByteFormatter;

#[derive(Default)]
pub struct LittleEndianFormatter {}

impl ByteFormatter for LittleEndianFormatter {
    fn unwrap_bytes(&self, bytes: u16) -> [u8; 2] {
        [(bytes & (0xff)) as u8, (bytes >> 8) as u8]
    }

    fn wrap_bytes(&self, bytes: [u8; 2]) -> u16 {
        ((bytes[1] as u16) << 8) + (bytes[0] as u16)
    }
}