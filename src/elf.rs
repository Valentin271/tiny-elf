use crate::{
    elf_header::{ElfHeader, VADDR_START},
    patchable::Patchable,
    prelude::*,
    program_header::{Flags, ProgramHeader},
};

pub struct Elf<'a, T>
where
    T: AsBytes,
{
    elf_header: ElfHeader,
    program_header: ProgramHeader,
    program: T,
    headers: Vec<ProgramHeader>,
    datas: Vec<&'a dyn AsBytes>,
}

impl<'a, T> Elf<'a, T>
where
    T: AsBytes,
{
    /// Creates an ELF struct from a program.
    ///
    /// A program is something that can be converted to bytes using [`AsBytes`].
    pub fn new(program: T) -> Self {
        let mut header =
            ProgramHeader::from_data(&program.as_bytes(), Flags::Executable | Flags::Readable);
        header.set_addr(VADDR_START);

        let mut this = Self {
            elf_header: ElfHeader::default(),
            program_header: header,
            program,
            headers: Vec::new(),
            datas: Vec::new(),
        };

        this.elf_header.increment_pheader();

        this
    }

    /// Adds binary data to the file.
    pub fn add_data(&mut self, data: &'a dyn AsBytes, flags: Flags) {
        self.elf_header.increment_pheader();

        let mut header = ProgramHeader::from_data(&data.as_bytes(), flags);

        let previous_data_len =
            self.program.as_bytes().len() + self.datas.iter().flat_map(|d| d.as_bytes()).count();

        header.set_offset(previous_data_len as u64);
        header.set_addr(VADDR_START + previous_data_len as u64);
        self.headers.push(header);

        self.datas.push(data);
    }

    /// Effectively computes the final binary size.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len_headers()
            + self.program.as_bytes().len()
            + self.datas.iter().flat_map(|d| d.as_bytes()).count()
    }

    /// Size of all the headers in the file
    fn len_headers(&self) -> usize {
        self.elf_header.as_bytes().len()
            + self.program_header.as_bytes().len()
            + self.headers.iter().flat_map(AsBytes::as_bytes).count()
    }
}

impl<'a, T> Elf<'a, T>
where
    T: AsBytes + Patchable,
{
    pub fn backpatch(&mut self) {
        let data_addr = self.len_headers() + self.program.as_bytes().len();

        self.program.backpatch(
            self.elf_header.entry() as u32,
            VADDR_START as u32 + data_addr as u32,
        );
    }
}

impl<'a, T> AsBytes for Elf<'a, T>
where
    T: AsBytes,
{
    fn as_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.append(&mut self.elf_header.as_bytes());

        data.append(&mut self.program_header.as_bytes());
        for header in &self.headers {
            data.append(&mut header.as_bytes());
        }

        data.append(&mut self.program.as_bytes());
        for d in &self.datas {
            data.append(&mut d.as_bytes());
        }

        data
    }
}
