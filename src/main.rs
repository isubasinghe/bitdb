mod bitcask;
mod network;
mod engine;
use bitcask::dmap::DMap;
use bitcask::dirman::Dirman;
use bitcask::errors::BitcaskError;
use std::time::Instant;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;

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

    

    let sql = "SELECT a, b, 123, myfunc(b) \
               FROM table_1 \
               WHERE a > b AND b < 100 \
               ORDER BY a DESC, b";

    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    
    Ok(())
}
