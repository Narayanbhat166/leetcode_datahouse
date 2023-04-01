// This service is supposed to scrape the data from leetcode and insert them into redis
// There can be many producers which try to scrape different submission id
// Lock should be acquired before scraping a submission id so that the same
// submission id is not scrapped again by other producers
// Lock feature is implemented using redis

use leetcode_datahouse::{configs, database, models, scrape::scrape_submission};

#[tokio::main]
async fn main() {
    let config = configs::read_config();
    let mut db_conn = database::create_connection(&config.db.url).unwrap_or_else(|db_error| {
        panic!(
            "Could not establish connection to the database {}",
            db_error
        )
    });

    // Producer must get the submission id from the redis server, acquire the lock to it
    // Scrape the data and then insert it back into redis queue.

    let submission_id = 925862080;
    let scrapped_data = scrape_submission(submission_id).await.unwrap();
    println!("{scrapped_data:?}");

    let res =
        database::insert_submission(database::NewSubmission::from(scrapped_data), &mut db_conn);

    println!("Submission in database -> {:?}", res);
}
