use super::{
    register::Register, AsAsm, Immediate::*, Instruction, Memory, Operand, REXB, REXR, REXW,
};
use crate::prelude::AsBytes;

const EXPECT_ONE_BYTE_REGISTER: &str = "Registers are always 1 byte";

/// # See
///
/// <http://ref.x86asm.net/coder64-abc.html>
/// <https://wiki.osdev.org/X86-64_Instruction_Encoding#ModR.2FM>
#[derive(Debug, Clone)]
pub enum Mnemonic {
    Add(Register, Operand),
    Call(Memory),
    Cmp(Register, i32),
    Imul(Register, Operand),
    Je(Memory),
    Jg(Memory),
    Jge(Memory),
    Jl(Memory),
    Jle(Memory),
    Jmp(Memory),
    Jne(Memory),
    Label(String),
    Mov(Register, Operand),
    Pop(Register),
    Push(Operand),
    /// Alias for RETN
    Ret,
    Sub(Register, Operand),
    Syscall,
}

impl AsBytes for Mnemonic {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Mnemonic::Add(r, op) => {
                match op {
                    Operand::Mem(_) => unimplemented!(),
                    Operand::Imm(imm) => {
                        let mut prefix = REXW;
                        if r.is_extended() {
                            prefix |= REXB;
                        }
                        let mut inst = match imm {
                            // http://ref.x86asm.net/coder64.html#x83
                            Imm8(_) => vec![prefix, 0x83],
                            // http://ref.x86asm.net/coder64.html#x81
                            Imm16(_) | Imm32(_) => vec![prefix, 0x81],
                        };
                        inst.append(&mut r.as_bytes());
                        inst.append(&mut imm.as_bytes());
                        inst
                    }
                    // http://ref.x86asm.net/coder64.html#x03
                    Operand::Reg(r2) => {
                        let mut prefix = REXW;
                        if r.is_extended() {
                            prefix |= REXB;
                        }
                        if r2.is_extended() {
                            prefix |= REXR;
                        }
                        let mut inst = vec![prefix, 0x03];
                        inst.append(&mut r2.as_bytes_opcode_extend(
                            *r.as_bytes().first().expect(EXPECT_ONE_BYTE_REGISTER),
                        ));
                        inst
                    }
                }
            }
            // http://ref.x86asm.net/coder64.html#xE8
            Mnemonic::Call(mem) => Instruction::new(0xE8)
                .operand(&mut mem.as_bytes())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#x81_7
            Mnemonic::Cmp(r, v) => {
                let mut prefix = REXW;
                if r.is_extended() {
                    prefix |= REXB;
                }
                let mut inst = vec![prefix, 0x81];
                inst.append(&mut r.as_bytes_opcode_extend(7));
                inst.append(&mut v.as_bytes());
                inst
            }
            Mnemonic::Imul(r, op) => {
                match op {
                    Operand::Mem(_) => unimplemented!(),
                    Operand::Imm(imm) => {
                        let mut prefix = REXW;
                        if r.is_extended() {
                            prefix |= REXB;
                        }
                        let mut inst = match imm {
                            // http://ref.x86asm.net/coder64.html#x6B
                            Imm8(_) => vec![prefix, 0x6B],
                            // http://ref.x86asm.net/coder64.html#x69
                            Imm16(_) | Imm32(_) => vec![prefix, 0x69],
                        };
                        inst.append(&mut r.as_bytes_opcode_extend(
                            *r.as_bytes().first().expect(EXPECT_ONE_BYTE_REGISTER),
                        ));
                        inst.append(&mut imm.as_bytes());
                        inst
                    }
                    // http://ref.x86asm.net/coder64.html#x0FAF
                    Operand::Reg(r2) => {
                        let mut prefix = REXW;
                        if r.is_extended() {
                            prefix |= REXB;
                        }
                        if r2.is_extended() {
                            prefix |= REXR;
                        }
                        let mut inst = vec![prefix, 0x0F, 0xAF];
                        inst.append(&mut r2.as_bytes_opcode_extend(
                            *r.as_bytes().first().expect(EXPECT_ONE_BYTE_REGISTER),
                        ));
                        inst
                    }
                }
            }
            // http://ref.x86asm.net/coder64.html#x0F84
            Mnemonic::Je(a) => {
                let mut inst = vec![0x0F, 0x84];
                inst.append(&mut a.as_bytes());
                inst
            }
            // http://ref.x86asm.net/coder64.html#x0F8D
            Mnemonic::Jge(a) => {
                let mut inst = vec![0x0F, 0x8D];
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
            // http://ref.x86asm.net/coder64.html#x0F8E
            Mnemonic::Jle(a) => {
                let mut inst = vec![0x0F, 0x8E];
                inst.append(&mut a.as_bytes());
                inst
            }
            // http://ref.x86asm.net/coder64.html#xE9
            Mnemonic::Jmp(mem) => Instruction::with_prefix(REXW, 0xE9)
                .operand(&mut mem.as_bytes())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#x0F85
            Mnemonic::Jne(mem) => {
                let mut inst = vec![0x0F, 0x85];
                inst.append(&mut mem.as_bytes());
                inst
            }
            Mnemonic::Label(_) => vec![],
            Mnemonic::Mov(r, o) => match o {
                // http://ref.x86asm.net/coder64.html#x8B
                Operand::Reg(r2) => {
                    let mut prefix = REXW;
                    if r.is_extended() {
                        prefix |= REXB;
                    }
                    if r2.is_extended() {
                        prefix |= REXR;
                    }
                    let mut inst = vec![prefix, 0x89];
                    inst.append(&mut r.as_bytes_opcode_extend(
                        *r2.as_bytes().first().expect(EXPECT_ONE_BYTE_REGISTER),
                    ));
                    inst
                }
                // http://ref.x86asm.net/coder64.html#xC7
                Operand::Mem(_) | Operand::Imm(Imm16(_) | Imm32(_)) => {
                    let mut inst = if r.is_extended() {
                        vec![REXB | REXW]
                    } else {
                        vec![REXW]
                    };
                    inst.append(&mut vec![0xC7]);
                    inst.append(&mut r.as_bytes());
                    inst.append(&mut o.as_bytes());
                    inst
                }
                Operand::Imm(Imm8(_)) => unimplemented!(),
            },
            // http://ref.x86asm.net/coder64.html#x8F
            Mnemonic::Pop(r) => {
                let mut prefix = REXW;
                if r.is_extended() {
                    prefix |= REXB;
                }
                Instruction::with_prefix(prefix, 0x8F)
                    .operand(&mut r.as_bytes())
                    .as_bytes()
            }
            Mnemonic::Push(o) => match o {
                // http://ref.x86asm.net/coder64.html#xFF_6
                Operand::Reg(r) => {
                    let mut prefix = REXW;
                    if r.is_extended() {
                        prefix |= REXB;
                    }
                    Instruction::with_prefix(prefix, 0xFF)
                        .operand(&mut r.as_bytes_opcode_extend(6))
                        .as_bytes()
                }
                Operand::Imm(i) => {
                    match i {
                        // http://ref.x86asm.net/coder64.html#x6A
                        Imm8(_) => Instruction::new(0x6A),
                        // http://ref.x86asm.net/coder64.html#x68
                        Imm16(_) | Imm32(_) => Instruction::new(0x68),
                    }
                    .operand(&mut i.as_bytes())
                    .as_bytes()
                }
                Operand::Mem(_) => unimplemented!(),
            },
            // http://ref.x86asm.net/coder64.html#xC3
            Mnemonic::Ret => Instruction::new(0xC3).as_bytes(),
            Mnemonic::Sub(r, op) => {
                match op {
                    Operand::Mem(_) => unimplemented!(),
                    Operand::Imm(imm) => {
                        let mut prefix = REXW;
                        if r.is_extended() {
                            prefix |= REXB;
                        }
                        let mut inst = match imm {
                            // http://ref.x86asm.net/coder64.html#x83_5
                            Imm8(_) => vec![prefix, 0x83],
                            // http://ref.x86asm.net/coder64.html#x81_5
                            Imm16(_) | Imm32(_) => vec![prefix, 0x81],
                        };
                        inst.append(&mut r.as_bytes_opcode_extend(5));
                        inst.append(&mut imm.as_bytes());
                        inst
                    }
                    // http://ref.x86asm.net/coder64.html#x2B
                    Operand::Reg(r2) => {
                        let mut prefix = REXW;
                        if r.is_extended() {
                            prefix |= REXB;
                        }
                        if r2.is_extended() {
                            prefix |= REXR;
                        }
                        let mut inst = vec![prefix, 0x2B];
                        inst.append(&mut r2.as_bytes_opcode_extend(
                            *r.as_bytes().first().expect(EXPECT_ONE_BYTE_REGISTER),
                        ));
                        inst
                    }
                }
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
            Mnemonic::Call(mem) => format!("call {}", mem.as_asm()),
            Mnemonic::Cmp(r, v) => format!("cmp {}, {}", r.as_asm(), v),
            Mnemonic::Imul(r, imm) => format!("imul {}, {}", r.as_asm(), imm.as_asm()),
            Mnemonic::Je(a) => format!("je {}", a.as_asm()),
            Mnemonic::Jg(a) => format!("jg {}", a.as_asm()),
            Mnemonic::Jge(a) => format!("jge {}", a.as_asm()),
            Mnemonic::Jl(a) => format!("jl {}", a.as_asm()),
            Mnemonic::Jle(a) => format!("jle {}", a.as_asm()),
            Mnemonic::Jmp(a) => format!("jmp {}", a.as_asm()),
            Mnemonic::Jne(a) => format!("jne {}", a.as_asm()),
            Mnemonic::Label(l) => format!("\n{l}:"),
            Mnemonic::Mov(r, o) => format!("mov {}, {}", r.as_asm(), o.as_asm()),
            Mnemonic::Pop(r) => format!("pop {}", r.as_asm()),
            Mnemonic::Push(o) => format!("push {}", o.as_asm()),
            Mnemonic::Ret => "ret".into(),
            Mnemonic::Sub(r, v) => format!("sub {}, {}", r.as_asm(), v.as_asm()),
            Mnemonic::Syscall => "syscall".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Mnemonic::*;
    use super::Register::*;
    use super::*;
    use crate::asm::instruction::*;

    mod mov {
        use super::*;

        #[test]
        fn simple() {
            let bytes = Mov(Rax, 1.into()).as_bytes();
            assert_eq!(
                bytes,
                vec![
                    REXW,
                    0xC7,
                    *Rax.as_bytes().first().unwrap(),
                    0x01,
                    0x0,
                    0x0,
                    0x0
                ]
            );
        }

        #[test]
        fn extended_register_op1() {
            let bytes = Mov(R8, 1.into()).as_bytes();
            assert_eq!(
                bytes,
                vec![
                    REXW | REXB,
                    0xC7,
                    *R8.as_bytes().first().unwrap(),
                    0x1,
                    0x0,
                    0x0,
                    0x0
                ]
            );
        }

        #[test]
        fn extended_register_op2() {
            let bytes = Mov(Rax, R8.into()).as_bytes();
            assert_eq!(
                bytes,
                vec![
                    REXW | REXR,
                    0x89,
                    *Rax.as_bytes_opcode_extend(*R8.as_bytes().first().unwrap())
                        .first()
                        .unwrap()
                ]
            );
        }
    }
}
