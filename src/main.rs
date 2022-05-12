mod bitcask;
use bitcask::memstore::MemStore;
use bitcask::dmap::DMap;
use bitcask::dirman::Dirman;
use bitcask::errors::BitcaskError;

fn main() -> Result<(), BitcaskError> {
    let mut dman = Dirman::<DMap>::open("/tmp/".to_string())?;
    dman.put("Hello World".to_string(), "Bye Bye World\n".to_string())?;

    let x: String = dman.get_as("Hello World".to_string())?;

    print!("RETRIEVED {}", x);
    
    Ok(())
}
