use crate::configs::ConfigData;
use crate::consts;
use fred::prelude::*;

#[tokio::main]
pub async fn create_client(config_data: ConfigData) -> Result<RedisClient, RedisError> {
    let config = RedisConfig {
        server: ServerConfig::Centralized {
            host: config_data.redis.host,
            port: config_data.redis.port,
        },
        username: config_data.redis.username,
        password: config_data.redis.password,
        ..RedisConfig::default()
    };
    let client = RedisClient::new(config);

    Ok(client)
}

pub async fn lock_submission_id(submission_id: String, client: RedisClient) -> Option<String> {
    let policy = ReconnectPolicy::default();
    // connect to the server, returning a handle to a task that drives the connection
    let _ = client.connect(Some(policy));

    // wait for the client to connect
    let _ = client.wait_for_connect().await.unwrap();

    let res: Option<String> = client
        .set(
            "submission_id",
            submission_id,
            Some(Expiration::EX(10)),
            Some(SetOptions::NX),
            false,
        )
        .await
        .unwrap();
    res
}

pub async fn get_submission_id(redis_client: RedisClient) -> String {
    let policy = ReconnectPolicy::default();
    // connect to the server, returning a handle to a task that drives the connection
    let _ = redis_client.connect(Some(policy));

    // wait for the client to connect
    let _ = redis_client.wait_for_connect().await.unwrap();

    let submission_id_res = redis_client.get::<String, _>("next_submission_id").await;

    let submission_id = match submission_id_res {
        Ok(submission_id) => submission_id,
        Err(err) => {
            // Maybe not found? or redis error
            if &RedisErrorKind::NotFound == err.kind() {
                // Set submission id to some value and return the submission id
                let submission_id = redis_client
                    .set::<String, _, _>(
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
                consts::DEFAULT_START_SUBMISSION_ID.to_owned()
            }
        }
    };

    submission_id
}
