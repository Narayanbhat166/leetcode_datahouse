use thiserror::Error;

#[derive(Error, Debug)]
pub enum GrpcServerError {
    #[error("Some error")]
    BullShitError,
}

#[derive(Error, Debug)]
pub enum GrpcClientError {
    #[error("Failed to connect to the grpc server")]
    ConnectionToServerFailed,
}
