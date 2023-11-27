use crate::prelude::AsBytes;

#[non_exhaustive]
pub enum Type {
    Unknown,
    Relocatable,
    Executable,
    SharedObject,
    Core,
}

impl AsBytes for Type {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Type::Unknown => vec![0x00, 0x00],
            Type::Relocatable => vec![0x01, 0x00],
            Type::Executable => vec![0x02, 0x00],
            Type::SharedObject => vec![0x03, 0x00],
            Type::Core => vec![0x04, 0x00],
        }
    }
}
