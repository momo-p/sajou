use super::error::DatabaseError;
use moka::future::Cache;
use std::sync::Arc;

const MAX_CAPACITY: u64 = 1_600;

pub struct MokaWrapper {
    pub table: Cache<String, Arc<Vec<u8>>>,
}

impl MokaWrapper {
    pub async fn new() -> Result<Self, DatabaseError> {
        Ok(Self {
            table: Cache::new(MAX_CAPACITY),
        })
    }
}
