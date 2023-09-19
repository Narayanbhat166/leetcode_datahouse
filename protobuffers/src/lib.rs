pub use controller_grpc::{
    controller_server::{Controller, ControllerServer},
    GetSubmissionIdRequest, ScrappedResponse, StartScrapingRequest, StartScrapingResponse,
    StoreResult, SubmissionIdResponse,
};

pub mod controller_grpc {
    // The string specified here must match the proto package name
    tonic::include_proto!("controller");
}
