use crate::prelude::AsBytes;

#[non_exhaustive]
pub enum Type {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Tls,
}

impl AsBytes for Type {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Type::Null => 0u32.as_bytes(),
            Type::Load => 1u32.as_bytes(),
            Type::Dynamic => 2u32.as_bytes(),
            Type::Interp => 3u32.as_bytes(),
            Type::Note => 4u32.as_bytes(),
            Type::Shlib => 5u32.as_bytes(),
            Type::Phdr => 6u32.as_bytes(),
            Type::Tls => 7u32.as_bytes(),
        }
    }
}
