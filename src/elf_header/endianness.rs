use crate::bytes::AsBytes;

pub enum Endianness {
    LittleEndian,
    BigEndian,
}

impl AsBytes for Endianness {
    fn as_bytes(&self) -> Vec<u8> {
        vec![match self {
            Endianness::LittleEndian => 1,
            Endianness::BigEndian => 2,
        }]
    }
}
