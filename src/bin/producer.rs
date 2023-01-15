// This service is supposed to scrape the data from leetcode and insert them into redis
// There can be many producers which try to scrape different submission id
// Lock should be acquired before scraping a submission id so that the same
// submission id is not scrapped again by other producers
// Lock feature is implemented using redis

fn main() {}
