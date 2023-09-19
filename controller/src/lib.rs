use fred::prelude::RedisClient;
use redis;

use protobuffers::{
    Controller, ControllerServer, GetSubmissionIdRequest, ScrappedResponse, StartScrapingRequest,
    StartScrapingResponse, StoreResult, SubmissionIdResponse,
};

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
        request: tonic::Request<ScrappedResponse>, // Accept request of type HelloRequest
    ) -> Result<tonic::Response<StoreResult>, tonic::Status> {
        println!("Got a request: {:?}", request);

        let scrapped_response = request.into_inner();

        let insert_submission_result = redis::insert_submission_into_queue(
            &self.redis_client,
            scrapped_response.submission_id,
            scrapped_response.data,
        )
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

        let request = request.into_inner();

        let submission_id =
            redis::mark_submission_id_as_scraping(&self.redis_client, request.submission_id).await;

        match submission_id {
            Ok(_) => Ok(tonic::Response::new(StartScrapingResponse {
                no_objection: true,
            })),
            //FIXME: Send error message
            Err(_) => Err(tonic::Status::new(
                tonic::Code::Ok,
                "ALready being scrapped",
            )),
        }
    }
}

#[tokio::main]
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(true)
        .init();

    let redis_config = configs::read_config::<configs::types::RedisConfigData>();
    let controller_config = configs::read_config::<configs::types::ControllerConfigData>();
    log::info!("Configuration file is Valid");

    let addr = controller_config.controller.get_bind_address().parse()?;

    let redis_client = redis::create_redis_client(redis_config.redis)
        .await
        .expect("Cannot establish redis connection");

    let controller = MyController::new(redis_client);
    log::info!("Running the server on {:?}", addr);

    tonic::transport::Server::builder()
        .add_service(ControllerServer::new(controller))
        .serve(addr)
        .await?;

    Ok(())
}
