mod bitcask;
use bitcask::memstore::MemStore;
use bitcask::dmap::DMap;
use bitcask::dirman::Dirman;
use bitcask::errors::BitcaskError;

fn main() -> Result<(), BitcaskError> {
    let mut dman = Dirman::<DMap>::open("/tmp/".to_string())?;
    dman.put("Hello World".to_string(), 213)?;
    dman.get("Hello World".to_string());
    
    Ok(())
}
