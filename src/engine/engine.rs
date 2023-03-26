use crate::engine::sql::*;
use async_trait::async_trait;



#[async_trait]
pub trait StorageEngine {
    async fn execute_create(create: CreateTable);
}
