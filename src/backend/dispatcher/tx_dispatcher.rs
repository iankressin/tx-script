use crate::backend::prepared_transaction::PreparedTransaction;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait TransactionDispatcher {
    async fn dispatch(&self, tx: Vec<PreparedTransaction>) -> Result<(), Box<dyn Error>>;
}
