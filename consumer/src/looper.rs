/// Run a loop to fetch the data from queue and store it into a persistent store
use std::{thread, time::Duration};

use errors;
use queues::Queue;

use crate::{database, pooper::poop};

pub async fn loop_and_poop<T: Queue>(queue: T) -> Result<(), errors::consumer::ConsumerError> {
    let db_config = configs::read_config::<configs::types::DbConfigData>();

    let mut connection = database::create_connection(db_config).unwrap();

    loop {
        let data = queue.pop(consts::SUBMISSIONS_LIST).await?;

        if let Some(data) = data {
            log::info!("{data:?}");
            poop(data, &mut connection).await;
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
