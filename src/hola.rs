mod configs;
mod curl_parser;
mod database;
mod redis_helpers;
mod schema;
mod scrape;

fn main() {
    // let res = scrape::get_submission("1".to_string()).unwrap();
    // println!("{:#?}", res);

    // let config_data = configs::read_config();
    // let client = redis_helpers::create_client(config_data).unwrap();

    // let res = redis_helpers::lock_submission_id("1234".to_string(), client);
    // println!("{:?}", res);
    let new_submission = database::NewSubmission {
        id: 3,
        last_testcase: "None".to_string(),
        memory: 1.0,
        memory_display: "1.0".to_string(),
        memory_percentile: 1.0,
        notes: "Hello boi".to_string(),
        runtime: 1.0,
        runtime_percentile: 1.0,
        status_code: 1,
        timestamp: 1,
        code_hash: "hash".to_string(),
    };

    // // redis_helpers::redis_test();
    let mut conn = database::create_connection();
    let submission = database::insert_submission(new_submission, &mut conn);
    database::create_connection();
}
