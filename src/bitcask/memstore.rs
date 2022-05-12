#[derive(Clone)]
pub struct ValueMetadata {
    pub file_id: String,
    pub value_size: u64, 
    pub value_pos: u64, 
    pub timestamp: u128,
}

pub trait MemStore {
    fn new() -> Self;
    fn put(&mut self, key: String, metadata: ValueMetadata);
    fn get_metadata(&self, key: String) -> Option<ValueMetadata>;
}
