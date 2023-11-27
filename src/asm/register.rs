use std::ops::Add;

use super::AsAsm;
use crate::prelude::AsBytes;

#[derive(Debug, Clone)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsp,
}

impl Register {
    /// `op` is left-shifted (`<<`) because of the position it should be.
    ///
    /// See https://wiki.osdev.org/X86-64_Instruction_Encoding#ModR.2FM
    pub fn as_bytes_opcode_extend(&self, op: u8) -> Vec<u8> {
        vec![self
            .as_bytes()
            .first()
            .expect("Register is always referenced by at least a byte")
            .add(op << 3)]
    }
}

impl AsBytes for Register {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Register::Rax => vec![0xC0],
            Register::Rbx => vec![0xC3],
            Register::Rcx => vec![0xC1],
            Register::Rdx => vec![0xC2],
            Register::Rsp => vec![0xC4],
        }
    }
}

impl AsAsm for Register {
    fn as_asm(&self) -> String {
        match self {
            Register::Rax => "rax",
            Register::Rbx => "rbx",
            Register::Rcx => "rcx",
            Register::Rdx => "rdx",
            Register::Rsp => "rsp",
        }
        .into()
    }
}
