mod bitcask;
mod engine;
mod network;
use bitcask::dirman::Dirman;
use bitcask::dmap::DMap;
use bitcask::errors::BitcaskError;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::parser::Parser;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() -> Result<(), BitcaskError> {
    let mut dman = Dirman::<DMap>::open("/tmp/".to_string())?;
    let start = Instant::now();

    for i in 0..10 {
        dman.put(format!("KEY:{}", i), "Bye Bye World\n".to_string())?;
    }
    let end = Instant::now();

    let diff = end - start;
    println!("100000 inserts completed in {}ms", diff.as_millis());
    println!("{}ns per insert", diff.as_nanos() / 100000);

    let sql = r"CREATE TABLE Persons (
                    person_id int,
                    name varchar
                    )";

    let dialect = PostgreSqlDialect {};
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    println!("{:#?}", ast[0]);
    match &ast[0] {
        sqlparser::ast::Statement::Query(q) => {
            println!("{:#?}", q.body);
        }
        _ => {}
    }

    Ok(())
}
