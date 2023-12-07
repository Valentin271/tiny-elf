use super::{
    register::Register, AsAsm, Either, Immediate::*, Instruction, Memory, Operand, RexPrefix,
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
    Dec(Register),
    Inc(Register),
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
    Xor(Register, Operand),
}

impl AsBytes for Mnemonic {
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Mnemonic::Add(r, op) => {
                match op {
                    Operand::Mem(_) => unimplemented!(),
                    Operand::Imm(imm) => {
                        match imm {
                            // http://ref.x86asm.net/coder64.html#x83
                            Imm8(_) => Instruction::new(0x83),
                            // http://ref.x86asm.net/coder64.html#x81
                            Imm16(_) | Imm32(_) => Instruction::new(0x81),
                        }
                        .operand((*r).into())
                        .operand((*imm).into())
                        .as_bytes()
                    }
                    // http://ref.x86asm.net/coder64.html#x03
                    Operand::Reg(r2) => Instruction::new(0x03)
                        .op_extended_register(*r, Either::Right(*r2))
                        .as_bytes(),
                }
            }
            // http://ref.x86asm.net/coder64.html#xE8
            Mnemonic::Call(mem) => Instruction::new(0xE8)
                .operand(mem.to_owned().into())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#x81_7
            Mnemonic::Cmp(r, v) => Instruction::new(0x81)
                .op_extended_register(*r, Either::Left(7))
                .operand((*v).into())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#xFF_1
            Mnemonic::Dec(r) => Instruction::new(0xFF)
                .op_extended_register(*r, Either::Left(1))
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#xFF_0
            Mnemonic::Inc(r) => Instruction::new(0xFF)
                .op_extended_register(*r, Either::Left(0))
                .as_bytes(),
            Mnemonic::Imul(r, op) => {
                match op {
                    Operand::Mem(_) => unimplemented!(),
                    Operand::Imm(imm) => {
                        match imm {
                            // http://ref.x86asm.net/coder64.html#x6B
                            Imm8(_) => Instruction::new(0x6B),
                            // http://ref.x86asm.net/coder64.html#x69
                            Imm16(_) | Imm32(_) => Instruction::new(0x69),
                        }
                        .op_extended_register(*r, Either::Right(*r))
                        .operand((*imm).into())
                        .as_bytes()
                    }
                    // http://ref.x86asm.net/coder64.html#x0FAF
                    Operand::Reg(r2) => Instruction::multibyte(vec![0x0F, 0xAF])
                        .op_extended_register(*r, Either::Right(*r2))
                        .as_bytes(),
                }
            }
            // http://ref.x86asm.net/coder64.html#x0F84
            Mnemonic::Je(a) => Instruction::multibyte(vec![0x0F, 0x84])
                .operand(a.to_owned().into())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#x0F8D
            Mnemonic::Jge(a) => Instruction::multibyte(vec![0x0F, 0x8D])
                .operand(a.to_owned().into())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#x0F8F
            Mnemonic::Jg(a) => Instruction::multibyte(vec![0x0F, 0x8F])
                .operand(a.to_owned().into())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#x0F8C
            Mnemonic::Jl(a) => Instruction::multibyte(vec![0x0F, 0x8C])
                .operand(a.to_owned().into())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#x0F8E
            Mnemonic::Jle(a) => Instruction::multibyte(vec![0x0F, 0x8E])
                .operand(a.to_owned().into())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#xE9
            Mnemonic::Jmp(mem) => Instruction::new(0xE9)
                .operand(mem.to_owned().into())
                .as_bytes(),
            // http://ref.x86asm.net/coder64.html#x0F85
            Mnemonic::Jne(mem) => Instruction::multibyte(vec![0x0F, 0x85])
                .operand(mem.to_owned().into())
                .as_bytes(),
            Mnemonic::Label(_) => vec![],
            Mnemonic::Mov(r, o) => match o {
                // http://ref.x86asm.net/coder64.html#x8B
                Operand::Reg(r2) => {
                    let mut prefix = RexPrefix::W.bits();
                    if r.is_extended() {
                        prefix |= RexPrefix::B.bits();
                    }
                    if r2.is_extended() {
                        prefix |= RexPrefix::R.bits();
                    }
                    let mut inst = vec![prefix, 0x89];
                    // for some reason move register are reversed
                    inst.append(&mut r.as_bytes_opcode_extend(
                        *r2.as_bytes().first().expect(EXPECT_ONE_BYTE_REGISTER),
                    ));
                    inst
                }
                // http://ref.x86asm.net/coder64.html#xC7
                Operand::Mem(_) | Operand::Imm(Imm16(_) | Imm32(_)) => Instruction::new(0xC7)
                    .operand((*r).into())
                    .operand(o.to_owned())
                    .as_bytes(),
                Operand::Imm(Imm8(_)) => unimplemented!(),
            },
            // http://ref.x86asm.net/coder64.html#x8F
            Mnemonic::Pop(r) => Instruction::new(0x8F).operand((*r).into()).as_bytes(),
            Mnemonic::Push(o) => match o {
                // http://ref.x86asm.net/coder64.html#xFF_6
                Operand::Reg(r) => Instruction::new(0xFF)
                    .op_extended_register(*r, Either::Left(6))
                    .as_bytes(),
                Operand::Imm(i) => {
                    match i {
                        // http://ref.x86asm.net/coder64.html#x6A
                        Imm8(_) => Instruction::new(0x6A),
                        // http://ref.x86asm.net/coder64.html#x68
                        Imm16(_) | Imm32(_) => Instruction::new(0x68),
                    }
                    .operand((*i).into())
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
                        match imm {
                            // http://ref.x86asm.net/coder64.html#x83_5
                            Imm8(_) => Instruction::new(0x83),
                            // http://ref.x86asm.net/coder64.html#x81_5
                            Imm16(_) | Imm32(_) => Instruction::new(0x81),
                        }
                        .op_extended_register(*r, Either::Left(5))
                        .operand((*imm).into())
                        .as_bytes()
                    }
                    // http://ref.x86asm.net/coder64.html#x2B
                    Operand::Reg(r2) => Instruction::new(0x2B)
                        .op_extended_register(*r, Either::Right(*r2))
                        .as_bytes(),
                }
            }
            // http://ref.x86asm.net/coder64.html#x0F05
            Mnemonic::Syscall => vec![0x0f, 0x05],
            Mnemonic::Xor(r, op) => match op {
                Operand::Imm(_) | Operand::Mem(_) => unimplemented!(),
                Operand::Reg(r2) => Instruction::new(0x33)
                    .op_extended_register(*r, Either::Right(*r2))
                    .as_bytes(),
            },
        }
    }
}

impl AsAsm for Mnemonic {
    fn as_asm(&self) -> String {
        match self {
            Mnemonic::Add(r, v) => format!("add {}, {}", r.as_asm(), v.as_asm()),
            Mnemonic::Call(mem) => format!("call {}", mem.as_asm()),
            Mnemonic::Cmp(r, v) => format!("cmp {}, {}", r.as_asm(), v),
            Mnemonic::Dec(r) => format!("dec {}", r.as_asm()),
            Mnemonic::Inc(r) => format!("inc {}", r.as_asm()),
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
            Mnemonic::Xor(r, o) => format!("xor {}, {}", r.as_asm(), o.as_asm()),
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
                    RexPrefix::W.bits(),
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
                    RexPrefix::W.bits() | RexPrefix::B.bits(),
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
                    RexPrefix::W.bits() | RexPrefix::R.bits(),
                    0x89,
                    *Rax.as_bytes_opcode_extend(*R8.as_bytes().first().unwrap())
                        .first()
                        .unwrap()
                ]
            );
        }
    }
}
