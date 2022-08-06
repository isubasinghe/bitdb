mod bitcask;
mod network;
use bitcask::dmap::DMap;
use bitcask::dirman::Dirman;
use bitcask::errors::BitcaskError;
use std::time::Instant;

fn main() -> Result<(), BitcaskError> {
    let mut dman = Dirman::<DMap>::open("/tmp/".to_string())?;
    let start = Instant::now(); 

    for i in 0..100000 {
        dman.put(format!("KEY:{}", i), "Bye Bye World\n".to_string())?;
    }
    let end = Instant::now();

    let diff = end-start;
    println!("100000 inserts completed in {}ms", diff.as_millis());
    println!("{}ns per insert", diff.as_nanos()/100000);
    
    Ok(())
}
