pub trait ByteFormatter {
    fn unwrap_bytes(&self, bytes: u16) -> [u8; 2];

    fn wrap_bytes(&self, bytes: [u8; 2]) -> u16;
}