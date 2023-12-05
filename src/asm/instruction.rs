use crate::prelude::AsBytes;
use bitflags::bitflags;

use super::{Operand, Register};

bitflags! {
    pub struct RexPrefix: u8 {
        /// Enables the use of 64-bit addressing
        const W = 0x48;
        /// Enables the use of extended registers as second operand
        const R = 0x44;
        /// Enables the use of extended registers as first operand
        const B = 0x41;
    }
}

pub struct Instruction {
    prefix: RexPrefix,
    opcode: Vec<u8>,
    operands: Vec<u8>,
    operand_count: u8,
}

impl Instruction {
    pub fn new(opcode: u8) -> Self {
        Self {
            prefix: RexPrefix::W,
            opcode: vec![opcode],
            operands: Vec::default(),
            operand_count: 0,
        }
    }

    pub fn multibyte(opcode: Vec<u8>) -> Self {
        Self {
            prefix: RexPrefix::W,
            opcode,
            operands: Vec::default(),
            operand_count: 0,
        }
    }

    pub fn operand(mut self, operand: Operand) -> Self {
        self.operand_count += 1;

        if matches!(operand, Operand::Reg(r) if r.is_extended()) {
            if self.operand_count == 1 {
                self.prefix |= RexPrefix::B;
            } else if self.operand_count == 2 {
                self.prefix |= RexPrefix::R;
            }
        }

        self.operands.append(&mut operand.as_bytes());

        self
    }

    /// Defines an opcode extended register as operand.
    ///
    /// # See
    ///
    /// - <http://ref.x86asm.net/#column_o>
    pub fn op_extended_register(mut self, reg: Register, ext: Either<u8, Register>) -> Self {
        if reg.is_extended() {
            self.prefix |= RexPrefix::B;
        }

        let mut bytes = match ext {
            Either::Left(n) => reg.as_bytes_opcode_extend(n),
            Either::Right(r2) => {
                if r2.is_extended() {
                    self.prefix |= RexPrefix::R;
                }
                r2.as_bytes_opcode_extend(*reg.as_bytes().first().unwrap())
            }
        };

        self.operands.append(&mut bytes);

        self
    }
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl AsBytes for Instruction {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![self.prefix.bits()];

        bytes.append(&mut self.opcode.clone());
        bytes.append(&mut self.operands.clone());

        bytes
    }
}
