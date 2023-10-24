use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasteError {
    #[error("Some error")]
    BullShitError,
}
