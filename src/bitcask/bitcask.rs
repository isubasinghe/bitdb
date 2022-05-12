use crate::bitcask::entry::Data;
use crate::bitcask::memstore::MemStore;
use bincode::serialize;
use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Seek;
use std::io::{BufReader, BufWriter};
use std::io::{Error, SeekFrom};
use std::time::{SystemTime, UNIX_EPOCH};

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
    fn open(name: String, version: u64, memstore: T) -> Result<Bitcask<T>, Error> {
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
    // fn open_opts() {}

    fn get(&mut self, key: String) -> Result<Vec<u8>, ()> {
        let metadata = self.mem_store.get_metadata(key);
        let metadata = match metadata {
            Some(a) => a,
            None => return Err(()),
        };
        self.reader.seek(SeekFrom::Start(metadata.value_pos));
        let mut buffer = Vec::with_capacity(metadata.value_size as usize);
        self.reader.read_exact(&mut buffer);
        Ok(buffer)
    }

    fn put<V>(&mut self, key: String, value: V)
    where
        for<'a> V: serde::de::Deserialize<'a>,
        V: Serialize,
    {
        let timestamp: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let keysize = key.len() as u64;
        let value_bytes = serialize(&value).unwrap();
        let value_size = value_bytes.len() as u64;

        let data: Data<V> = Data {
            timestamp,
            key,
            value,
        };
    }

    fn delete(key: String) {}
    fn merge() {}
}
