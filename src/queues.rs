/// A queue is used as a temporary storage space to support high throughput data
/// It can be composed of any underlying technology, redis, kafka, rabbit-mq
///
/// The necessary functionalities ( traits ) are
///
/// GetData -> Get the data from the queue
/// InsertData -> Push the data into queue
#[async_trait::async_trait]
pub trait Queue {
    type Client;
}
