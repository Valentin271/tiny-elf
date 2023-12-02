use super::AsAsm;
use crate::prelude::AsBytes;

#[derive(Debug, Clone)]
pub struct Memory {
    addr: u32,
    label: String,
}

impl Memory {
    pub fn label(&self) -> &str {
        self.label.as_ref()
    }

    pub fn set_addr(&mut self, addr: u32) {
        self.addr = addr;
    }
}

impl From<&str> for Memory {
    fn from(value: &str) -> Self {
        Self {
            addr: 0,
            label: value.into(),
        }
    }
}

impl From<String> for Memory {
    fn from(label: String) -> Self {
        Self { addr: 0, label }
    }
}

impl AsBytes for Memory {
    fn as_bytes(&self) -> Vec<u8> {
        self.addr.as_bytes()
    }
}

impl AsAsm for Memory {
    fn as_asm(&self) -> String {
        if self.label.is_empty() {
            format!("{}", self.addr)
        } else {
            self.label.clone()
        }
    }
}
