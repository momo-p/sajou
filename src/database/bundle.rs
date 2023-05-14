use super::{error::DatabaseError, moka::MokaWrapper};
use std::result::Result;

pub struct Database {
    pub moka: MokaWrapper,
}

impl Database {
    pub async fn new() -> Result<Self, DatabaseError> {
        let moka = MokaWrapper::new();
        let polls = futures::join!(moka);
        Ok(Self { moka: polls.0? })
    }
}
