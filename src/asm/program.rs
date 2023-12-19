use std::collections::HashMap;

mod data_section;

pub use data_section::DataSection;

use super::{AsAsm, Mnemonic, Operand};
use crate::{patchable::Patchable, prelude::AsBytes};

#[derive(Default, Clone)]
pub struct Program {
    pub instructions: Vec<Mnemonic>,
    data: DataSection,
}

impl Program {
    /// Adds an instruction to the program.
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, mnemonic: Mnemonic) -> Self {
        self.instructions.push(mnemonic);
        self
    }

    pub fn insert_data(mut self, key: &str, value: &str) -> Self {
        self.data.insert(key.into(), value.into());
        self
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

    /// Declare a new function.
    ///
    /// This is a convenience method to write the prolog of a function.
    pub fn func(self, label: &str) -> Self {
        use crate::asm::{Mnemonic::*, Register::*};

        self.label(label)
            .add(Push(Rbp.into()))
            .add(Mov(Rbp, Rsp.into()))
    }

    /// Ends a function.
    ///
    /// This is a convenience method to write the epilog of a function.
    pub fn func_end(self) -> Self {
        use crate::asm::{Mnemonic::*, Register::*};

        self.add(Mov(Rsp, Rbp.into())).add(Pop(Rbp)).add(Ret)
    }
}

impl Patchable for Program {
    fn backpatch(&mut self, _start_addr: u32, data_addr: u32) {
        let mut labels = HashMap::<String, i32>::default();
        let data_labels = self.data.addresses(data_addr);
        let mut current_byte: i32 = 0;

        for inst in self.instructions.iter() {
            current_byte += inst.as_bytes().len() as i32;

            if let Mnemonic::Label(label) = inst {
                labels.insert(label.clone(), current_byte);
            }
        }

        let mut current_byte: i32 = 0;

        for inst in self.instructions.iter_mut() {
            current_byte += inst.as_bytes().len() as i32;

            match inst {
                Mnemonic::Call(addr)
                | Mnemonic::Je(addr)
                | Mnemonic::Jne(addr)
                | Mnemonic::Jg(addr)
                | Mnemonic::Jge(addr)
                | Mnemonic::Jl(addr)
                | Mnemonic::Jle(addr)
                | Mnemonic::Jmp(addr)
                    if !addr.label().is_empty() =>
                {
                    let label_addr = labels
                        .get(addr.label())
                        .unwrap_or_else(|| panic!("Label '{}' not found", addr.label()));

                    addr.set_addr(label_addr - current_byte);
                }
                Mnemonic::Mov(_, Operand::Mem(addr)) if !addr.label().is_empty() => {
                    let data_addr = *data_labels
                        .get(addr.label())
                        .unwrap_or_else(|| panic!("Label '{}' not found", addr.label()));

                    addr.set_addr(data_addr as i32);
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
