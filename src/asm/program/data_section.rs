use std::collections::HashMap;

use crate::{asm::AsAsm, prelude::AsBytes};

mod data_entry;

pub use data_entry::*;

#[derive(Default, Clone)]
pub struct DataSection {
    data: Vec<DataEntry>,
}

impl DataSection {
    pub fn insert(&mut self, key: String, value: String) {
        let entry = DataEntry::new(key, value);
        self.data.push(entry);
    }

    pub fn addresses(&self, start_addr: u32) -> HashMap<String, u32> {
        let mut map = HashMap::default();

        let mut len: u32 = 0;
        for entry in &self.data {
            map.insert(entry.key().to_string(), start_addr + len);
            len += entry.value().len() as u32;
        }

        map
    }
}

impl AsBytes for DataSection {
    fn as_bytes(&self) -> Vec<u8> {
        self.data.iter().flat_map(AsBytes::as_bytes).collect()
    }
}

impl AsAsm for DataSection {
    fn as_asm(&self) -> String {
        let mut out: String = "section .data\n".into();

        for d in &self.data {
            out += &format!("    {}\n", d.as_asm())
        }

        out
    }
}
