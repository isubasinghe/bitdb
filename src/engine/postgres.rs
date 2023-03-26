use crate::engine::engine::*;
use crate::engine::sql::CreateTable;
use async_trait::async_trait;
use sqlparser::ast::{Statement};


pub struct PGEngine {}

#[async_trait]
impl StorageEngine for PGEngine {
    async fn execute_create(_create_table: CreateTable) {}
}

impl PGEngine {
    pub fn execute(stmt: Statement) {
        match &stmt {
            Statement::Query(_) => {}
            Statement::CreateTable { .. } => {}
            _ => {}
        }
    }
}
