use queues::redis::RedisQueue;
use redis;

use grpc::{ControllerServer, MyController};

#[tokio::main]
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(true)
        .init();

    let redis_config = configs::read_config::<configs::types::RedisConfigData>();
    let controller_config = configs::read_config::<configs::types::ControllerConfigData>();
    log::info!("Configuration file is Valid");

    let addr = controller_config.controller.get_bind_address().parse()?;

    let redis_client = redis::create_redis_client(redis_config.redis)
        .await
        .expect("Cannot establish redis connection");

    let redis_queue = RedisQueue {
        client: redis_client.clone(),
    };

    let controller = MyController::new(redis_client, redis_queue);
    log::info!("Running the server on {:?}", addr);

    tonic::transport::Server::builder()
        .add_service(ControllerServer::new(controller))
        .serve(addr)
        .await?;

    Ok(())
}
