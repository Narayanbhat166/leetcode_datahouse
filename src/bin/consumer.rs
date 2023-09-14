use std::time::Duration;

use leetcode_datahouse::{
    configs,
    database::{self, NewSubmission},
    models::ScrappedResponse,
    redis_helpers,
};

// This service is supposed to get scrapped data from redis and
// insert them into the persistent store ( currently postgres)
#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting Consumer");
    let config = configs::read_config();
    let mut db_connection = database::create_connection(&config.db.url).unwrap();
    let redis_conn = redis_helpers::create_redis_client(config.redis)
        .await
        .unwrap();

    loop {
        let submission = redis_helpers::get_submission_from_list(redis_conn.clone())
            .await
            .unwrap();

        if let Some(submission) = submission {
            let parsed_submission = serde_json::from_str::<ScrappedResponse>(&submission).unwrap();
            let submission_id = parsed_submission.submission_id;

            let db_submission = NewSubmission::from(parsed_submission);

            match database::insert_submission(db_submission, &mut db_connection) {
                Ok(res) => {
                    log::info!("Inserted {submission_id} into database");
                }
                Err(error) => {
                    log::error!("{error:?}");
                }
            }

            redis_helpers::pop_submission_from_list(redis_conn.clone())
                .await
                .unwrap();
        } else {
            log::info!("No new submissions waiting");
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
