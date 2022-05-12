use crate::bitcask::bitcask::Bitcask;
use crate::bitcask::errors::BitcaskError;
use crate::bitcask::memstore::MemStore;
use serde::{Serialize, Deserialize};

pub struct Dirman<T>
where
    T: MemStore,
{
    current_version: u64,
    root_dir: String,
    current_bitcask: Bitcask<T>,
}

impl<T: MemStore> Dirman<T> {
    pub fn open(root_dir: String) -> Result<Dirman<T>, BitcaskError> {
        // TODO: scan initial directory
        let mem_store = T::new();

        let bitcask = Bitcask::open(root_dir.to_owned(), 0, mem_store)?;
        Ok(Dirman {
            root_dir,
            current_version: 1,
            current_bitcask: bitcask,
        })
    }

    pub fn put<V>(&mut self, key: String, value: V) -> Result<(), BitcaskError> 
        where 
            for<'b> V: serde::de::Deserialize<'b>,
            V: Serialize,
    {
        self.current_bitcask.put(key, value)
    }
    
    pub fn get_as<V>(&mut self, key: String) -> Result<V, BitcaskError> 
        where 
            for<'b> V: serde::de::Deserialize<'b>,
            V: Serialize,
    {
        self.current_bitcask.get_as(key)
    }

}
