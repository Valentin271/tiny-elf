use super::AsAsm;
use crate::prelude::AsBytes;

#[derive(Debug, Clone, Copy)]
pub enum Immediate {
    Imm8(i8),
    Imm16(i16),
    Imm32(i32),
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

macro_rules! impl_from_for_immediate {
    ($ty:ty, $variant:ident) => {
        impl From<$ty> for Immediate {
            fn from(value: $ty) -> Self {
                Self::$variant(value)
            }
        }
    };
}

impl_from_for_immediate!(i8, Imm8);
impl_from_for_immediate!(i16, Imm16);
impl_from_for_immediate!(i32, Imm32);

impl AsAsm for Immediate {
    fn as_asm(&self) -> String {
        match self {
            Immediate::Imm8(n) => format!("{n}"),
            Immediate::Imm16(n) => format!("{n}"),
            Immediate::Imm32(n) => format!("{n}"),
        }
    }
}
