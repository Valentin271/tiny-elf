use crate::prelude::AsBytes;

mod flags;
mod ty;

pub use flags::*;
pub use ty::*;

/// An ELF program header.
///
/// See <https://en.wikipedia.org/wiki/Executable_and_Linkable_Format#Program_header>
pub struct ProgramHeader {
    /// Identifies the type of the segment
    ty: Type,
    /// Segment-dependent flags (position for 64-bit structure)
    flags: Flags,
    /// Offset of the segment in the file image
    offset: u64,
    /// Virtual address of the segment in memory
    vaddr: u64,
    /// On systems where physical address is relevant, reserved for segment's physical address
    paddr: u64,
    /// Size in bytes of the segment in the file image. May be 0.
    filesz: u64,
    /// Size in bytes of the segment in memory. May be 0.
    memsz: u64,
    /// 0 and 1 specify no alignment. Otherwise should be a positive, integral power of 2, with
    /// `vaddr` equating `offset` modulus `align`
    align: u64,
}

impl ProgramHeader {
    pub fn from_data(data: &[u8], flags: Flags) -> Self {
        let size = data.len();

        Self {
            flags,
            filesz: size as u64,
            memsz: size as u64,
            ..Default::default()
        }
    }

    pub fn set_addr(&mut self, addr: u64) {
        self.vaddr = addr;
        self.paddr = addr;
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }
}

impl Default for ProgramHeader {
    fn default() -> Self {
        Self {
            ty: Type::Load,
            flags: Flags::empty(),
            offset: 0,
            vaddr: 0,
            paddr: 0,
            filesz: 0,
            memsz: 0,
            align: 0,
        }
    }
}

impl AsBytes for ProgramHeader {
    fn as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(56);

        data.append(&mut self.ty.as_bytes());
        data.append(&mut self.flags.as_bytes());
        data.append(&mut self.offset.as_bytes());
        data.append(&mut self.vaddr.as_bytes());
        data.append(&mut self.paddr.as_bytes());
        data.append(&mut self.filesz.as_bytes());
        data.append(&mut self.memsz.as_bytes());
        data.append(&mut self.align.as_bytes());

        data
    }
}
