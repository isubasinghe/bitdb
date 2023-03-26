use crate::bitcask::memstore::*;
use dashmap::DashMap;

pub struct DMap {
    pub map: DashMap<String, ValueMetadata>,
}

impl MemStore for DMap {
    fn new() -> DMap {
        let map = DashMap::new();
        DMap { map }
    }
    fn put(&mut self, key: String, metadata: ValueMetadata) {
        self.map.insert(key, metadata);
    }
    fn get_metadata(&self, key: String) -> Option<ValueMetadata> {
        let data = self.map.get(&key)?;
        let val = data.value().clone();
        Some(val)
    }
}
