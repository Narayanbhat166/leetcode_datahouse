use configs::types::Redis;
mod consts;

use fred::prelude::*;

pub async fn create_redis_client(redis_config: Redis) -> Result<RedisClient, RedisError> {
    let config = RedisConfig {
        server: ServerConfig::Centralized {
            server: fred::types::Server {
                host: redis_config.host.into(),
                port: redis_config.port,
                tls_server_name: None,
            },
        },
        username: redis_config.username,
        password: redis_config.password,
        ..RedisConfig::default()
    };

    let client = RedisClient::new(config, None, None);

    // connect to the server, returning a handle to a task that drives the connection
    let _ = client.connect();

    // wait for the client to connect
    let _ = client.wait_for_connect().await.unwrap();

    Ok(client)
}

pub async fn insert_submission_into_queue(
    redis_client: &RedisClient,
    submission_id: u32,
    data: String,
) -> Result<(), RedisError> {
    // Add the submission to the list
    redis_client
        .lpush::<usize, _, _>(consts::SUBMISSIONS_LIST, data)
        .await?;

    // Remove the submission_id from currently scraping list
    redis_client
        .srem::<usize, _, _>(consts::SCRAPPING_SET, submission_id.clone())
        .await?;

    Ok(())
}

pub async fn mark_submission_id_as_scraping(
    redis_client: &RedisClient,
    submission_id: u32,
) -> Result<(), RedisError> {
    let currently_scraping_result = redis_client
        .sadd::<usize, _, _>(consts::SCRAPPING_SET, submission_id)
        .await?;

    if currently_scraping_result == 0 {
        println!("{submission_id} is already being scraped")
    } else {
        println!("Marked {submission_id} as scraping");
    }

    Ok(())
}

pub async fn get_submission_from_list(
    redis_client: RedisClient,
) -> Result<Option<String>, RedisError> {
    redis_client
        .lrange::<Vec<String>, _>(consts::SUBMISSIONS_LIST, 0, 0)
        .await
        .map(|elements| elements.first().cloned())
}

pub async fn pop_submission_from_list(
    redis_client: RedisClient,
) -> Result<Option<String>, RedisError> {
    redis_client
        .lpop::<Option<String>, _>(consts::SUBMISSIONS_LIST, None)
        .await
}

pub async fn get_next_submission_id(redis_client: &RedisClient) -> Result<u32, RedisError> {
    let submission_id_res = redis_client.incr::<u32, _>(consts::SUBMISSION_KEY).await;

    let submission_id = match submission_id_res {
        Ok(submission_id) => submission_id,
        Err(err) => {
            // Maybe not found? or redis error
            if &RedisErrorKind::NotFound == err.kind() {
                // Set submission id to some value and return the submission id
                let submission_id = redis_client
                    .set::<u32, _, _>(
                        consts::SUBMISSION_KEY,
                        consts::DEFAULT_START_SUBMISSION_ID,
                        None,
                        None,
                        true,
                    )
                    .await
                    .expect("Error when setting the default submission id");
                submission_id
            } else {
                consts::DEFAULT_START_SUBMISSION_ID
            }
        }
    };

    Ok(submission_id)
}
