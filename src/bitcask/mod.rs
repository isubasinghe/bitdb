use serde::{ser, Serialize, Deserialize};
pub mod entry;
pub mod keydir;
mod cmap;
mod ctrie;

pub struct Bitcask {
    name: String,
}



