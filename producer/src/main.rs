use error_stack::ResultExt;
use producer::extractor::Extractor;
use producer::leetcode_extractor::Leetcode;

use grpc;

#[tokio::main]
async fn main() {
    let mut controller_server =
        grpc::controller_grpc::controller_client::ControllerClient::connect(
            "http://127.0.0.1:6969",
        )
        .await
        .change_context(errors::grpc::GrpcClientError::ConnectionToServerFailed)
        .unwrap();

    // Producer must get the submission id from the redis server, acquire the lock to it
    // Scrape the data and then insert it back into redis queue.
    let submission_id = controller_server
        .get_submission_id(grpc::GetSubmissionIdRequest {})
        .await
        .map_err(|error| println!("{error:?}"))
        .unwrap()
        .into_inner()
        .submission_id;

    let extractor = Leetcode;
    let data = extractor.fetch_data(submission_id).await;

    if let Ok(scrapped_response) = data {
        let stringified_response = serde_json::to_string(&scrapped_response).unwrap();
        let scrapped_request = grpc::ScrappedResponse {
            submission_id,
            data: stringified_response,
        };
        controller_server
            .accept_scrapped_response(scrapped_request)
            .await
            .unwrap();
    }
}
