mod curl_parser;
mod redis_helpers;
mod scrape;
mod configs;

fn main() {
    // let res = scrape::get_submission("1".to_string()).unwrap();
    // println!("{:#?}", res);
    
    let config_data = configs::read_config();
    let client = redis_helpers::create_client(config_data).unwrap();

    let res = redis_helpers::lock_submission_id("1234".to_string(), client);
    println!("{:?}", res);

    // redis_helpers::redis_test();
}
