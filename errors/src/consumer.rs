use thiserror::{self, Error};

use crate::queue::QueueError;

#[derive(Debug, Error)]
pub enum ConsumerError {
    #[error("Failed to fetch data from queue")]
    FetchDataFailed(#[from] QueueError),
}

#[derive(Debug, Error)]
pub enum FileStorageError {
    #[error("There was an error when calling external file api")]
    ExternalApiCallFailed,
    #[error("Failed to save file in local storage")]
    FileSaveFailed(#[from] std::io::Error),
}
