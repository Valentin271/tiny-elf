use std::collections::HashMap;

mod data_section;

pub use data_section::DataSection;

use super::{AsAsm, Mnemonic, Operand};
use crate::{patchable::Patchable, prelude::AsBytes};

#[derive(Default, Clone)]
pub struct Program {
    instructions: Vec<Mnemonic>,
    data: DataSection,
}

impl Program {
    /// Adds an instruction to the program.
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, mnemonic: Mnemonic) -> Self {
        self.instructions.push(mnemonic);
        self
    }

    pub fn insert_data(&mut self, key: &str, value: &str) {
        self.data.insert(key.into(), value.into());
    }

    pub fn data(&self) -> &DataSection {
        &self.data
    }

    /// Creates a label in the program.
    ///
    /// Useful to jump to or to display in the output assembly.
    pub fn label(self, label: &str) -> Self {
        self.add(Mnemonic::Label(label.into()))
    }
}

impl Patchable for Program {
    fn backpatch(&mut self, _start_addr: u32, data_addr: u32) {
        let mut labels = HashMap::<String, u32>::default();
        let data_labels = self.data.addresses(data_addr);
        let mut bytes: u32 = 0;

        for inst in self.instructions.iter_mut().rev() {
            let instr_size = inst.as_bytes().len() as u32;
            bytes += instr_size;

            match inst {
                Mnemonic::Label(label) => {
                    labels.insert(label.clone(), bytes);
                }
                Mnemonic::Jg(addr) | Mnemonic::Jl(addr) | Mnemonic::Jmp(addr)
                    if !addr.label().is_empty() =>
                {
                    let t = labels
                        .get(addr.label())
                        .unwrap_or_else(|| panic!("Jump label '{}' not found", addr.label()));

                    addr.set_addr(bytes - t - instr_size);
                }
                Mnemonic::Mov(_, Operand::Mem(addr)) if !addr.label().is_empty() => {
                    let t = *data_labels
                        .get(addr.label())
                        .unwrap_or_else(|| panic!("Mov label '{}' not found", addr.label()));

                    addr.set_addr(t);
                }
                _ => (),
            }
        }
    }
}

impl AsAsm for Program {
    fn as_asm(&self) -> String {
        let mut out: String = "GLOBAL _start\n\n".into();

        out += &(self.data.as_asm() + "\n\n");

        out += "section .text\n";
        out += "_start:\n";

        for inst in &self.instructions {
            out += &if matches!(inst, Mnemonic::Label(_)) {
                format!("{}\n", inst.as_asm())
            } else {
                format!("    {}\n", inst.as_asm())
            };
        }

        out
    }
}

impl AsBytes for Program {
    fn as_bytes(&self) -> Vec<u8> {
        self.instructions
            .iter()
            .flat_map(AsBytes::as_bytes)
            .collect()
    }
}
