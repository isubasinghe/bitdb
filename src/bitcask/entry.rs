use bincode::{deserialize, serialize};
use serde::ser::Serialize;
use std::fs::File;
use std::io::{prelude::*, Write};
use std::io::{BufReader, BufWriter};
use std::io::{Seek, SeekFrom};

use super::errors;

pub struct Data<V>
where
    for<'a> V: serde::de::Deserialize<'a>,
    V: Serialize,
{
    pub timestamp: u128,
    pub key: String,
    pub value: V,
}

impl<V: Serialize> Data<V>
where
    for<'a> V: serde::de::Deserialize<'a>,
{
    pub fn write(&self, writer: &mut BufWriter<File>) -> Result<(u64, u64), errors::BitcaskError> {
        let timestamp = unsafe { std::mem::transmute::<u128, [u8; 16]>(self.timestamp) };

        let keysize = self.key.len() as u64;
        let keysize = unsafe { std::mem::transmute::<u64, [u8; 8]>(keysize) };
        let value_bytes = serialize(&self.value)?;
        let value_size = value_bytes.len() as u64;
        let value_size_bytes = unsafe { std::mem::transmute::<u64, [u8; 8]>(value_size) };
        writer.write(&timestamp)?;
        writer.write(&keysize)?;
        writer.write(&self.key.as_ref())?;
        writer.write(&value_size_bytes)?;
        let pos = writer.stream_position()?;
        writer.write(&value_bytes)?;
        writer.flush()?;
        Ok((pos, value_size))
    }

    pub fn read_entry(
        pos: u64,
        reader: &mut BufReader<File>,
    ) -> Result<(u128, String, V), errors::BitcaskError> {
        reader.seek(SeekFrom::Start(pos))?;
        let mut timedata = [0 as u8; 16];
        reader.read_exact(&mut timedata)?;
        let timedata = unsafe { std::mem::transmute::<[u8; 16], u128>(timedata) };

        let mut keysize = [0 as u8; 8];
        reader.read_exact(&mut keysize)?;
        let keysize = unsafe { std::mem::transmute::<[u8; 8], u64>(keysize) };

        let mut key = vec![0u8; keysize as usize];
        reader.read_exact(&mut key)?;
        let key = unsafe { std::str::from_utf8_unchecked(&key) };

        let mut value_size = [0u8; 8];
        reader.read_exact(&mut value_size)?;
        let value_size = unsafe { std::mem::transmute::<[u8; 8], u64>(value_size) };

        let mut value = vec![0u8; value_size as usize];
        reader.read_exact(&mut value)?;
        let value = deserialize::<V>(&value)?;
        Ok((timedata, key.to_string(), value))
    }

    #[inline(always)]
    pub fn read_value_at_bytes(
        pos: u64,
        vsize: u64,
        reader: &mut BufReader<File>,
    ) -> Result<Vec<u8>, errors::BitcaskError> {
        reader.seek(SeekFrom::Start(pos))?;
        let mut value = vec![0u8; vsize as usize];
        reader.read_exact(&mut value)?;
        Ok(value)
    }

    pub fn read_value_at(
        pos: u64,
        vsize: u64,
        reader: &mut BufReader<File>,
    ) -> Result<V, errors::BitcaskError> {
        let value: Vec<u8> = Data::<V>::read_value_at_bytes(pos, vsize, reader)?;
        let d = deserialize::<V>(&value)?;
        Ok(d)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufWriter};

    #[test]
    fn write_read() {
        use std::fs::File;
        let file = File::create("/tmp/bitcask.test").unwrap();
        let file_copy = File::open("/tmp/bitcask.test").unwrap();
        let mut writer = BufWriter::new(file);
        let original_timestamp = 6597123;
        let original_key = "Hello World!".to_string();
        let original_value = "Original Value".to_string();
        let mydata = super::Data {
            timestamp: original_timestamp,
            key: original_key.to_owned(),
            value: original_value.to_owned(),
        };
        let (pos, sz) = mydata.write(&mut writer).unwrap();
        println!("POS - {}", pos);
        let mut reader = BufReader::new(file_copy);
        let (timestamp, key, value): (_, _, String) =
            super::Data::read_entry(0, &mut reader).unwrap();
        assert!(timestamp == original_timestamp);
        assert!(key == original_key);
        assert!(value == original_value);
        let value: String = super::Data::read_value_at(pos, sz, &mut reader).unwrap();
        assert!(value == original_value);
    }
}
