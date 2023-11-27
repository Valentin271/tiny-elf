pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

impl AsBytes for [u8] {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl AsBytes for u8 {
    fn as_bytes(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl AsBytes for u16 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl AsBytes for u32 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl AsBytes for u64 {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }
}

impl AsBytes for Vec<&dyn AsBytes> {
    fn as_bytes(&self) -> Vec<u8> {
        self.iter().flat_map(|i| i.as_bytes()).collect()
    }
}
