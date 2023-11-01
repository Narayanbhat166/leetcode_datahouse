pub use controller_grpc::{
    controller_server::{Controller, ControllerServer},
    GetSubmissionIdRequest, ScrappedResponse, StartScrapingRequest, StartScrapingResponse,
    StoreResult, SubmissionIdResponse,
};

use fred::prelude::RedisClient;
use queues::Queue;

pub mod controller_grpc {
    // The string specified here must match the proto package name
    tonic::include_proto!("controller");
}

pub struct MyController<T: Queue> {
    redis_client: RedisClient,
    queue: T,
}

impl<T: Queue> MyController<T> {
    pub fn new(redis_client: RedisClient, queue: T) -> Self
    where
        T: Queue,
    {
        Self {
            redis_client,
            queue,
        }
    }
}

#[tonic::async_trait]
impl<T: Send + Sync + Queue + 'static> Controller for MyController<T> {
    async fn accept_scrapped_response(
        &self,
        request: tonic::Request<ScrappedResponse>, // Accept request of type HelloRequest
    ) -> Result<tonic::Response<StoreResult>, tonic::Status> {
        println!("Got a submission request: {:?}", request);

        let scrapped_response = request.into_inner();

        let insert_submission_result = self
            .queue
            .push(consts::SUBMISSIONS_LIST, &scrapped_response.data)
            .await;

        let response = match insert_submission_result {
            Ok(_) => StoreResult {
                stored: true,
                error: None,
            },
            Err(error) => StoreResult {
                stored: true,
                error: Some(error.to_string()),
            },
        };

        Ok(tonic::Response::new(response))
    }

    async fn get_submission_id(
        &self,
        request: tonic::Request<GetSubmissionIdRequest>,
    ) -> Result<tonic::Response<SubmissionIdResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let submission_id = redis::get_next_submission_id(&self.redis_client)
            .await
            .unwrap();

        Ok(tonic::Response::new(SubmissionIdResponse { submission_id }))
    }

    async fn start_scraping(
        &self,
        request: tonic::Request<StartScrapingRequest>,
    ) -> Result<tonic::Response<StartScrapingResponse>, tonic::Status> {
        println!("Got a request: {:?}", request);

        Ok(tonic::Response::new(StartScrapingResponse {}))
    }
}
