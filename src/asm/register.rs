use std::ops::Add;

use super::AsAsm;
use crate::prelude::AsBytes;

/// # See also
/// - <https://wiki.osdev.org/X86-64_Instruction_Encoding#Registers> for detail on available
/// registers and their memory representation.
/// - <https://en.wikibooks.org/wiki/X86_Assembly/X86_Architecture> especially for name
/// explanation.
#[derive(Debug, Clone, Copy)]
pub enum Register {
    /// Primary accumulator.
    ///
    /// On Linux, this is also the syscall number, syscall return value.
    ///
    /// See <http://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/>.
    Rax,
    /// Counter register.
    Rcx,
    /// Data register
    ///
    /// On Linux, this is also the syscall parameter 3.
    Rdx,
    // Base register
    Rbx,
    /// Stack pointer
    Rsp,
    /// Base pointer
    Rbp,
    /// Source index register.
    ///
    /// On Linux, this is also the syscall parameter 2
    Rsi,
    /// Destination index register.
    ///
    /// On Linux, this is also the syscall parameter 1
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
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
            Register::Rcx => vec![0xC1],
            Register::Rdx => vec![0xC2],
            Register::Rbx => vec![0xC3],
            Register::Rsp => vec![0xC4],
            Register::Rbp => vec![0xC5],
            Register::Rsi => vec![0xC6],
            Register::Rdi => vec![0xC7],
            Register::R8 => vec![0xC8],
            Register::R9 => vec![0xC9],
            Register::R10 => vec![0xCA],
            Register::R11 => vec![0xCB],
            Register::R12 => vec![0xCC],
            Register::R13 => vec![0xCD],
            Register::R14 => vec![0xCE],
            Register::R15 => vec![0xCF],
        }
    }
}

impl AsAsm for Register {
    fn as_asm(&self) -> String {
        match self {
            Register::Rax => "rax",
            Register::Rcx => "rcx",
            Register::Rdx => "rdx",
            Register::Rbx => "rbx",
            Register::Rsp => "rsp",
            Register::Rbp => "rbp",
            Register::Rsi => "rsi",
            Register::Rdi => "rdi",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
        }
        .into()
    }
}
