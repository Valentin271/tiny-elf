use super::{register::Register, AsAsm, Immediate, Immediate::*, Memory, Operand};
use crate::prelude::AsBytes;

/// # See
///
/// <http://ref.x86asm.net/coder64-abc.html>
/// <https://wiki.osdev.org/X86-64_Instruction_Encoding#ModR.2FM>
#[derive(Debug, Clone)]
pub enum Mnemonic {
    Add(Register, Immediate),
    Cmp(Register, u32),
    Imul(Register, Immediate),
    Je(Memory),
    Jg(Memory),
    Jl(Memory),
    Jmp(Memory),
    Label(String),
    Mov(Register, Operand),
    Pop(Register),
    Push(Operand),
    Sub(Register, Immediate),
    Syscall,
}

impl AsBytes for Mnemonic {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Mnemonic::Add(r, imm) => {
                let mut inst = match imm {
                    // http://ref.x86asm.net/coder64.html#x83
                    Imm8(_) => vec![0x48, 0x83],
                    // http://ref.x86asm.net/coder64.html#x81
                    Imm16(_) | Imm32(_) => vec![0x48, 0x81],
                };
                inst.append(&mut r.as_bytes());
                inst.append(&mut imm.as_bytes());
                inst
            }
            // http://ref.x86asm.net/coder64.html#x81_7
            Mnemonic::Cmp(r, v) => {
                let mut inst = vec![0x48, 0x81];
                inst.append(&mut r.as_bytes_opcode_extend(7));
                inst.append(&mut v.as_bytes());
                inst
            }
            Mnemonic::Imul(r, imm) => {
                let mut inst = match imm {
                    // http://ref.x86asm.net/coder64.html#x6B
                    Imm8(_) => vec![0x48, 0x6B],
                    // http://ref.x86asm.net/coder64.html#x69
                    Imm16(_) | Imm32(_) => vec![0x48, 0x69],
                };
                inst.append(&mut r.as_bytes_opcode_extend(
                    *r.as_bytes().first().expect("Registers are always 1 byte"),
                ));
                inst.append(&mut imm.as_bytes());
                inst
            }
            // http://ref.x86asm.net/coder64.html#x0F84
            Mnemonic::Je(a) => {
                let mut inst = vec![0x0F, 0x84];
                inst.append(&mut a.as_bytes());
                inst
            }
            // http://ref.x86asm.net/coder64.html#x0F8F
            Mnemonic::Jg(a) => {
                let mut inst = vec![0x0F, 0x8F];
                inst.append(&mut a.as_bytes());
                inst
            }
            // http://ref.x86asm.net/coder64.html#x0F8C
            Mnemonic::Jl(a) => {
                let mut inst = vec![0x0F, 0x8C];
                inst.append(&mut a.as_bytes());
                inst
            }
            // http://ref.x86asm.net/coder64.html#xE9
            Mnemonic::Jmp(mem) => {
                let mut inst = vec![0x48, 0xE9];
                inst.append(&mut mem.as_bytes());
                inst
            }
            Mnemonic::Label(_) => vec![],
            Mnemonic::Mov(r, o) => match o {
                // http://ref.x86asm.net/coder64.html#x8B
                Operand::Reg(r2) => {
                    let mut inst = vec![0x48, 0x8B];
                    inst.append(&mut r2.as_bytes_opcode_extend(
                        *r.as_bytes().first().expect("Registers are always 1 byte"),
                    ));
                    inst
                }
                // http://ref.x86asm.net/coder64.html#xC7
                Operand::Mem(_) | Operand::Imm(Imm16(_) | Imm32(_)) => {
                    let mut inst = vec![0x48, 0xC7];
                    inst.append(&mut r.as_bytes());
                    inst.append(&mut o.as_bytes());
                    inst
                }
                Operand::Imm(Imm8(_)) => unimplemented!(),
            },
            // http://ref.x86asm.net/coder64.html#x8F
            Mnemonic::Pop(r) => {
                let mut inst = vec![0x8F];
                inst.append(&mut r.as_bytes());
                inst
            }
            Mnemonic::Push(o) => match o {
                // http://ref.x86asm.net/coder64.html#xFF_6
                Operand::Reg(r) => {
                    let mut inst = vec![0xFF];
                    inst.append(&mut r.as_bytes_opcode_extend(6));
                    inst
                }
                Operand::Imm(i) => {
                    let mut inst = match i {
                        // http://ref.x86asm.net/coder64.html#x6A
                        Imm8(_) => vec![0x6A],
                        // http://ref.x86asm.net/coder64.html#x68
                        Imm16(_) | Imm32(_) => vec![0x68],
                    };
                    inst.append(&mut i.as_bytes());
                    inst
                }
                Operand::Mem(_) => unimplemented!(),
            },
            Mnemonic::Sub(r, imm) => {
                let mut inst = match imm {
                    // http://ref.x86asm.net/coder64.html#x83_5
                    Imm8(_) => vec![0x48, 0x83],
                    // http://ref.x86asm.net/coder64.html#x81_5
                    Imm16(_) | Imm32(_) => vec![0x48, 0x81],
                };
                inst.append(&mut r.as_bytes_opcode_extend(5));
                inst.append(&mut imm.as_bytes());
                inst
            }
            // http://ref.x86asm.net/coder64.html#x0F05
            Mnemonic::Syscall => vec![0x0f, 0x05],
        }
    }
}

impl AsAsm for Mnemonic {
    fn as_asm(&self) -> String {
        match self {
            Mnemonic::Add(r, v) => format!("add {}, {}", r.as_asm(), v.as_asm()),
            Mnemonic::Cmp(r, v) => format!("cmp {}, {}", r.as_asm(), v.as_asm()),
            Mnemonic::Imul(r, imm) => format!("imul {}, {}", r.as_asm(), imm.as_asm()),
            Mnemonic::Je(a) => format!("je {}", a.as_asm()),
            Mnemonic::Jg(a) => format!("jg {}", a.as_asm()),
            Mnemonic::Jl(a) => format!("jl {}", a.as_asm()),
            Mnemonic::Jmp(a) => format!("jmp {}", a.as_asm()),
            Mnemonic::Label(l) => format!("\n{l}:"),
            Mnemonic::Mov(r, o) => format!("mov {}, {}", r.as_asm(), o.as_asm()),
            Mnemonic::Pop(r) => format!("pop {}", r.as_asm()),
            Mnemonic::Push(o) => format!("push {}", o.as_asm()),
            Mnemonic::Sub(r, v) => format!("sub {}, {}", r.as_asm(), v.as_asm()),
            Mnemonic::Syscall => "syscall".into(),
        }
    }
}
