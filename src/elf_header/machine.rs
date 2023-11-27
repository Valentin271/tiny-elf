use crate::prelude::AsBytes;

#[non_exhaustive]
pub enum Machine {
    None,
    X86,
    AmdX86_64,
}

impl AsBytes for Machine {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Machine::None => vec![0x00, 0x00],
            Machine::X86 => vec![0x03, 0x00],
            Machine::AmdX86_64 => vec![0x3E, 0x00],
        }
    }
}
