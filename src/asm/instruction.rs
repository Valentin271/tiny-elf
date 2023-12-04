use crate::prelude::AsBytes;

/// Enables the use of 64-bit addressing
pub const REXW: u8 = 0x48;
/// Enables the use of extended registers as op2
pub const REXR: u8 = 0x44;
/// Enables the use of extended registers as op1
pub const REXB: u8 = 0x41;

pub struct Instruction {
    prefix: Vec<u8>,
    opcode: Vec<u8>,
    operands: Vec<u8>,
}

impl Instruction {
    pub fn new(opcode: u8) -> Self {
        Self {
            prefix: vec![REXW],
            opcode: vec![opcode],
            operands: Vec::default(),
        }
    }

    pub fn with_prefix(prefix: u8, opcode: u8) -> Self {
        Self {
            prefix: vec![REXW | prefix],
            opcode: vec![opcode],
            operands: Vec::default(),
        }
    }

    pub fn operand(mut self, operand: &mut Vec<u8>) -> Self {
        self.operands.append(operand);
        self
    }
}

impl AsBytes for Instruction {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.prefix.clone();
        bytes.append(&mut self.opcode.clone());
        bytes.append(&mut self.operands.clone());
        bytes
    }
}
