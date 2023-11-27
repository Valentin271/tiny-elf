use bitflags::bitflags;

use crate::prelude::AsBytes;

bitflags! {
    pub struct Flags:u32 {
        const Executable = 0x1;
        const Writeable  = 0x2;
        const Readable   = 0x4;
    }
}

impl AsBytes for Flags {
    fn as_bytes(&self) -> Vec<u8> {
        self.bits().as_bytes()
    }
}
