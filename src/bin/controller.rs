use fred::prelude::RedisClient;
use leetcode_datahouse::redis_helpers;
use tonic::{transport::Server, Request, Response, Status};

use controller_grpc::controller_server::{Controller, ControllerServer};
use controller_grpc::{
    GetSubmissionIdRequest, ScrappedResponse, StartScrapingRequest, StartScrapingResponse,
    StoreResult, SubmissionIdResponse,
};

pub mod controller_grpc {
    // The string specified here must match the proto package name
    tonic::include_proto!("controller");
}

pub struct MyController {
    redis_client: RedisClient,
}

impl MyController {
    fn new(redis_client: RedisClient) -> Self {
        Self { redis_client }
    }
}

#[tonic::async_trait]
impl Controller for MyController {
    async fn accept_scrapped_response(
        &self,
        request: Request<ScrappedResponse>, // Accept request of type HelloRequest
    ) -> Result<Response<StoreResult>, Status> {
        println!("Got a request: {:?}", request);

        let scrapped_response = request.into_inner();

        let insert_submission_result = redis_helpers::insert_submission(
            &self.redis_client,
            scrapped_response.submission_id,
            scrapped_response.data,
        )
        .await;

        let response = match insert_submission_result {
            Ok(_) => controller_grpc::StoreResult {
                stored: true,
                error: None,
            },
            Err(error) => controller_grpc::StoreResult {
                stored: true,
                error: Some(error.to_string()),
            },
        };

        Ok(Response::new(response))
    }

    async fn get_submission_id(
        &self,
        request: Request<GetSubmissionIdRequest>,
    ) -> Result<Response<SubmissionIdResponse>, Status> {
        println!("Got a request: {:?}", request);

        let submission_id = redis_helpers::get_next_submission_id(&self.redis_client)
            .await
            .unwrap();

        Ok(Response::new(controller_grpc::SubmissionIdResponse {
            submission_id,
        }))
    }

    async fn start_scraping(
        &self,
        request: Request<StartScrapingRequest>,
    ) -> Result<Response<StartScrapingResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = request.into_inner();

        let submission_id = redis_helpers::mark_submission_id_as_scraping(
            &self.redis_client,
            request.submission_id,
        )
        .await;

        match submission_id {
            Ok(_) => Ok(Response::new(controller_grpc::StartScrapingResponse {
                no_objection: true,
            })),
            //FIXME: Send error message
            Err(_) => Err(Status::new(tonic::Code::Ok, "ALready being scrapped")),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let config = leetcode_datahouse::configs::read_config();

    let redis_client = redis_helpers::create_redis_client(config.redis)
        .await
        .expect("Cannot establish redis connection");

    let controller = MyController::new(redis_client);

    println!("Running the server on {:?}", addr);

    Server::builder()
        .add_service(ControllerServer::new(controller))
        .serve(addr)
        .await?;

    Ok(())
}
