pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

impl AsBytes for [u8] {
    fn as_bytes(&self) -> Vec<u8> {
        self.to_vec()
    }
}

impl AsBytes for Vec<&dyn AsBytes> {
    fn as_bytes(&self) -> Vec<u8> {
        self.iter().flat_map(|i| i.as_bytes()).collect()
    }
}

/// Implements [`AsBytes`] for numeric types
macro_rules! impl_num_as_bytes {
    ($ty:ty) => {
        impl AsBytes for $ty {
            fn as_bytes(&self) -> Vec<u8> {
                self.to_ne_bytes().to_vec()
            }
        }
    };
}

impl_num_as_bytes!(i8);
impl_num_as_bytes!(i16);
impl_num_as_bytes!(i32);
impl_num_as_bytes!(i64);
impl_num_as_bytes!(u8);
impl_num_as_bytes!(u16);
impl_num_as_bytes!(u32);
impl_num_as_bytes!(u64);
