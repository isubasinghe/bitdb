use async_trait::async_trait;
use sqlparser::ast::*;

#[async_trait]
pub trait StorageEngine {
    async fn execute_create(stmt: Statement);

    async fn get_table_definition(name: String);

    async fn execute_insert(stmt: Statement);

    async fn execute_select(stmt: Statement);

    async fn execute_delete(stmt: Statement);

    async fn execute_join(stmt: Statement);
}
