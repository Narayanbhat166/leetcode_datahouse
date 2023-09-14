use leetcode_datahouse::scrape::scrape_submission;

use controller_grpc::controller_client::ControllerClient;
use controller_grpc::{GetSubmissionIdRequest, ScrappedResponse, StartScrapingRequest};

pub mod controller_grpc {
    // The string specified here must match the proto package name
    tonic::include_proto!("controller");
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut controller_server = ControllerClient::connect("http://[::1]:50051")
        .await
        .unwrap();

    log::info!("Started Server");

    loop {
        // Producer must get the submission id from the redis server, acquire the lock to it
        // Scrape the data and then insert it back into redis queue.
        let submission_id = controller_server
            .get_submission_id(GetSubmissionIdRequest {})
            .await
            .unwrap()
            .into_inner()
            .submission_id;

        let request = leetcode_datahouse::scrape::create_scrape_request(submission_id).unwrap();

        controller_server
            .start_scraping(StartScrapingRequest { submission_id })
            .await
            .unwrap();

        let scrapped_data = scrape_submission(request, submission_id).await.unwrap();

        if scrapped_data.contains_data() {
            let stringified_data = serde_json::to_string(&scrapped_data).unwrap();

            let grpc_request = ScrappedResponse {
                submission_id,
                data: stringified_data,
            };

            controller_server
                .accept_scrapped_response(grpc_request)
                .await
                .unwrap();

            log::info!("Success {submission_id}");
        } else {
            log::warn!("No data present for submission_id {submission_id}");
        }
    }
}
