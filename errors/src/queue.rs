use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("Pushing data to queue failed")]
    PushFailed(#[from] fred::error::RedisError),
    #[error("Serialization failed")]
    SerializationFailed(#[from] serde_json::Error),
}
