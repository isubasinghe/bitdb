use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Error;
use std::io::{prelude, Write};
use std::io::{BufReader, BufWriter};

pub struct Bitcask {
    name: String, // file name
    current_version: u64,
    reader: BufReader<File>,
    writer: BufWriter<File>,
}

impl Bitcask {
    fn open() -> Result<Bitcask, Error> {
        let name = "current.db";
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(name)?;
        let file_copy = file.try_clone()?;
        let reader = BufReader::new(file);
        let writer = BufWriter::new(file_copy);
        Ok(Bitcask {
            name: name.to_string(),
            current_version: 1,
            reader,
            writer,
        })
    }
    // fn open_opts() {}

    fn get(key: String) {}
    fn put<V>(key: String, value: V)
    where
        for<'a> V: serde::de::Deserialize<'a>,
    {
    }
    fn delete(key: String) {}
    fn merge() {}
}
