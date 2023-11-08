pub use controller_grpc::{
    controller_server::{Controller, ControllerServer},
    GetSubmissionIdRequest, ScrappedResponse, StartScrapingRequest, StartScrapingResponse,
    StoreResult, SubmissionIdResponse,
};

use fred::prelude::{ListInterface, RedisClient};
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
        let parsed_submission_details =
            serde_json::from_str::<models::api_models::ScrappedResponse>(&scrapped_response.data);

        let response = if let Err(error_details) = parsed_submission_details {
            let res = self
                .redis_client
                .lpush::<u32, &str, _>(consts::DEAD_LETTER_QUEUE, scrapped_response.submission_id)
                .await
                .unwrap();
            println!("{res:?}");
            StoreResult {
                stored: false,
                error: Some(error_details.to_string()),
            }
        } else {
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
            response
        };

        Ok(tonic::Response::new(response))
    }

    async fn get_submission_id(
        &self,
        request: tonic::Request<GetSubmissionIdRequest>,
    ) -> Result<tonic::Response<SubmissionIdResponse>, tonic::Status> {
        println!(
            "Received a request for new submission scraping {:?}",
            request
        );

        println!("Checking dead letter queue for failed submissions");

        let submission_id = self
            .redis_client
            .lrange::<Vec<u32>, _>(consts::DEAD_LETTER_QUEUE, 0, 1)
            .await
            .unwrap();

        if submission_id.len() == 0 {
            println!("No submissions in dlq");
        } else {
            println!("Submissions found in dlq");
        }

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
