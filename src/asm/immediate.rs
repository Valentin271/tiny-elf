use super::AsAsm;
use crate::prelude::AsBytes;

#[derive(Debug, Clone)]
pub enum Immediate {
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
}

impl AsBytes for Immediate {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Immediate::Imm8(n) => n.as_bytes(),
            Immediate::Imm16(n) => n.as_bytes(),
            Immediate::Imm32(n) => n.as_bytes(),
        }
    }
}

impl From<u8> for Immediate {
    fn from(value: u8) -> Self {
        Self::Imm8(value)
    }
}

impl From<u16> for Immediate {
    fn from(value: u16) -> Self {
        Self::Imm16(value)
    }
}

impl From<u32> for Immediate {
    fn from(value: u32) -> Self {
        Self::Imm32(value)
    }
}

impl AsAsm for Immediate {
    fn as_asm(&self) -> String {
        match self {
            Immediate::Imm8(n) => format!("{n}"),
            Immediate::Imm16(n) => format!("{n}"),
            Immediate::Imm32(n) => format!("{n}"),
        }
    }
}
