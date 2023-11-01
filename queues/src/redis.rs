use fred::prelude::ListInterface;

use crate::Queue;

pub struct RedisQueue {
    pub client: fred::clients::RedisClient,
}

#[async_trait::async_trait]
impl Queue for RedisQueue {
    async fn push(&self, queue_name: &str, data: &String) -> Result<(), errors::queue::QueueError> {
        self.client.lpush(queue_name, data).await?;

        Ok(())
    }

    async fn pop(&self, queue_name: &str) -> Result<Option<String>, errors::queue::QueueError> {
        let data = self.client.lpop(queue_name, None).await?;

        Ok(data)
    }
}
