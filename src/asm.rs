mod immediate;
mod memory;
mod mnemonic;
mod operand;
mod program;
mod register;

pub use immediate::*;
pub use memory::*;
pub use mnemonic::*;
pub use operand::*;
pub use program::*;
pub use register::*;

pub trait AsAsm {
    fn as_asm(&self) -> String;
}

impl AsAsm for u32 {
    fn as_asm(&self) -> String {
        format!("{self}")
    }
}
