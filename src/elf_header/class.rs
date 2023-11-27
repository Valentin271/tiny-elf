use crate::bytes::AsBytes;

pub enum Class {
    Bits32,
    Bits64,
}

impl AsBytes for Class {
    fn as_bytes(&self) -> Vec<u8> {
        vec![match self {
            Class::Bits32 => 1,
            Class::Bits64 => 2,
        }]
    }
}
