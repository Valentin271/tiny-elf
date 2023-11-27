pub trait Patchable {
    fn backpatch(&mut self, start_addr: u32, data_addr: u32);
}
