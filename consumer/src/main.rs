use queues;
mod database;
mod looper;
mod pooper;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(true)
        .init();

    let redis_config = configs::read_config::<configs::types::RedisConfigData>();
    log::info!("Configuration file is Valid");

    let redis_client = redis::create_redis_client(redis_config.redis)
        .await
        .expect("Cannot establish redis connection");

    let redis_queue = queues::redis::RedisQueue {
        client: redis_client,
    };

    looper::loop_and_poop(redis_queue).await.unwrap();
}
