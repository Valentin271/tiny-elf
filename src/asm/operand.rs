use super::*;
use crate::prelude::AsBytes;

#[derive(Debug, Clone)]
pub enum Operand {
    Mem(Memory),
    /// An immediate value
    Imm(Immediate),
    Reg(Register),
}

impl AsBytes for Operand {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Operand::Mem(a) => a.as_bytes(),
            Operand::Imm(i) => i.as_bytes(),
            Operand::Reg(r) => r.as_bytes(),
        }
    }
}

impl From<Memory> for Operand {
    fn from(value: Memory) -> Self {
        Self::Mem(value)
    }
}

impl<T> From<T> for Operand
where
    T: Into<Immediate>,
{
    fn from(value: T) -> Self {
        Self::Imm(value.into())
    }
}

impl From<Register> for Operand {
    fn from(value: Register) -> Self {
        Self::Reg(value)
    }
}

impl AsAsm for Operand {
    fn as_asm(&self) -> String {
        match self {
            Operand::Mem(a) => a.as_asm(),
            Operand::Imm(i) => i.as_asm(),
            Operand::Reg(r) => r.as_asm(),
        }
    }
}
