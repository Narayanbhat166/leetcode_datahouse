use errors;

pub mod redis;

/// A queue is used as a temporary storage space to support high throughput data.
/// It can be composed of any underlying technology, redis, kafka, rabbit-mq
///
/// The necessary functionalities ( traits ) are
///
/// GetData -> Get the data from the queue
/// InsertData -> Push the data into queue

#[async_trait::async_trait]
pub trait Queue {
    async fn push(&self, queue_name: &str, data: &String) -> Result<(), errors::queue::QueueError>;

    async fn pop(&self, queue_name: &str) -> Result<Option<String>, errors::queue::QueueError>;
}
