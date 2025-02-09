use crate::errors::FastSocketError;
use async_trait::async_trait;

#[async_trait]
pub trait Message: Send + Sync {
    async fn respond(&self) -> Result<(), FastSocketError>;
}
