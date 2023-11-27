use crate::{asm::AsAsm, prelude::AsBytes};

#[derive(Clone)]
pub struct DataEntry {
    key: String,
    value: String,
}

impl DataEntry {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &str {
        self.key.as_ref()
    }
}

impl AsBytes for DataEntry {
    fn as_bytes(&self) -> Vec<u8> {
        self.value.as_bytes().to_vec()
    }
}

impl AsAsm for DataEntry {
    fn as_asm(&self) -> String {
        format!(
            r#"{} db "{}""#,
            self.key,
            self.value
                // TODO: other chars should be converted too
                .replace('\n', &format!(r#"", {}, ""#, '\n' as u32))
        )
    }
}
