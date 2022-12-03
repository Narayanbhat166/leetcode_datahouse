use fred::prelude::*;
use std::future::Future;
use crate::configs::ConfigData;

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

#[tokio::main]
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
        .await.unwrap();
    res
}
