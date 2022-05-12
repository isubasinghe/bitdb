use bincode::serialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Seek;
use std::io::{BufReader, BufWriter};
use std::io::{Error, SeekFrom};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::bitcask::entry::Data;
use crate::bitcask::memstore::MemStore;
use crate::bitcask::errors;

use super::memstore::ValueMetadata;

pub struct Bitcask<T>
where
    T: MemStore,
{
    name: String, // file name
    current_version: u64,
    reader: BufReader<File>,
    writer: BufWriter<File>,
    mem_store: T,
}

impl<T: MemStore> Bitcask<T>
where
    T: MemStore,
{
    pub fn open(root_dir: String, version: u64, memstore: T) -> Result<Bitcask<T>, Error> {
        let name = format!("{}/b{}", root_dir, version);
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(name.clone())?;

        let file_copy = file.try_clone()?;
        let reader = BufReader::new(file);
        let writer = BufWriter::new(file_copy);
        Ok(Bitcask {
            name: name.to_string(),
            current_version: version,
            reader,
            writer,
            mem_store: memstore,
        })
    }


    pub fn get(&mut self, key: String) -> Result<Vec<u8>, errors::BitcaskError> {
        let metadata = self.mem_store.get_metadata(key);
        let metadata = match metadata {
            Some(a) => a,
            None => return Err(errors::BitcaskError::NonExistentKey()),
        };
        self.reader.seek(SeekFrom::Start(metadata.value_pos))?;
        let mut buffer = Vec::with_capacity(metadata.value_size as usize);
        self.reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn put<V>(&mut self, key: String, value: V) -> Result<(), errors::BitcaskError>
    where
        for<'b> V: serde::de::Deserialize<'b>,
        V: Serialize,
    {
        let timestamp: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let keysize = key.len() as u64;
        let value_bytes = serialize(&value)?;
        let value_size = value_bytes.len() as u64;

        let data: Data<V> = Data {
            timestamp,
            key: key.to_owned(),
            value,
        };
        let (pos, value_sz) = data.write(&mut self.writer)?;
        let metadata = ValueMetadata{ file_id: self.name.to_owned(), timestamp, value_pos: pos, value_size: value_sz};
        self.mem_store.put(key, metadata);
        Ok(())
    }

    pub fn get_as<V>(&mut self, key: String) -> Result<V, errors::BitcaskError> 
        where 
            for<'b> V: serde::de::Deserialize<'b>,
            V: Serialize
    {
        let val = match self.mem_store.get_metadata(key) {
            Some(val) => val, 
            None => return Err(errors::BitcaskError::NonExistentKey())
        };
        Data::<V>::read_value_at(val.value_pos, val.value_size, &mut self.reader)
    }

    pub fn delete(key: String) {
        //TODO: finish off tombstombing
        todo!();
    }

    fn merge() {
        //TODO: finish this off
        todo!();
    }

    pub fn done(self) -> T {
        self.mem_store
    }

}
