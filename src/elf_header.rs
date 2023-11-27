use self::{class::Class, endianness::Endianness, machine::Machine, os_abi::OsAbi, ty::Type};
use crate::prelude::AsBytes;

pub mod class;
pub mod endianness;
pub mod machine;
pub mod os_abi;
pub mod ty;

pub const VADDR_START: u64 = 0x400000;

/// See <https://en.wikipedia.org/wiki/Executable_and_Linkable_Format>
pub struct ElfHeader {
    magic: [u8; 4],
    class: Class,
    data: Endianness,
    elf_version: u8,
    os_abi: OsAbi,
    abi_version: u8,
    pad: [u8; 7],
    ty: Type,
    machine: Machine,
    version: [u8; 4],
    entry: u64,
    phoff: u64,
    shoff: u64,
    flags: u32,
    ehsize: u16,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16,
}

impl Default for ElfHeader {
    fn default() -> Self {
        Self {
            magic: [0x7F, b'E', b'L', b'F'],
            class: Class::Bits64,
            data: Endianness::LittleEndian,
            elf_version: 1,
            os_abi: OsAbi::SystemV,
            abi_version: 0,
            pad: [0; 7],
            ty: Type::Executable,
            machine: Machine::AmdX86_64,
            version: [1, 0, 0, 0],
            entry: VADDR_START + 0x40, // + this header size, start point is always first data
            phoff: 0x40,
            shoff: 0,
            flags: 0,
            ehsize: 0x40,
            phentsize: 0x38,
            phnum: 0,
            shentsize: 0,
            shnum: 0,
            shstrndx: 0,
        }
    }
}

impl ElfHeader {
    /// Increments the number of program header
    pub fn increment_pheader(&mut self) {
        self.phnum += 1;
        self.entry += 0x38;
    }

    /// Gets the entrypoint address of the file
    pub fn entry(&self) -> u64 {
        self.entry
    }
}

impl AsBytes for ElfHeader {
    fn as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(64);

        data.append(&mut self.magic.as_bytes());
        data.append(&mut self.class.as_bytes());
        data.append(&mut self.data.as_bytes());
        data.append(&mut self.elf_version.as_bytes());
        data.append(&mut self.os_abi.as_bytes());
        data.append(&mut self.abi_version.as_bytes());
        data.append(&mut self.pad.as_bytes());
        data.append(&mut self.ty.as_bytes());
        data.append(&mut self.machine.as_bytes());
        data.append(&mut self.version.as_bytes());
        data.append(&mut self.entry.as_bytes());
        data.append(&mut self.phoff.as_bytes());
        data.append(&mut self.shoff.as_bytes());
        data.append(&mut self.flags.as_bytes());
        data.append(&mut self.ehsize.as_bytes());
        data.append(&mut self.phentsize.as_bytes());
        data.append(&mut self.phnum.as_bytes());
        data.append(&mut self.shentsize.as_bytes());
        data.append(&mut self.shnum.as_bytes());
        data.append(&mut self.shstrndx.as_bytes());

        data
    }
}
