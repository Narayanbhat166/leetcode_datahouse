use std::io::Write;

use diesel::PgConnection;
use models::{api_models, storage_models};

use crate::database;

/// Store the data into a persistent store

pub trait FileStorage {
    fn store_file(
        file_name: String,
        data: String,
    ) -> Result<(), errors::consumer::FileStorageError>;
}

pub struct LocalFileStorage;

impl FileStorage for LocalFileStorage {
    fn store_file(
        file_name: String,
        data: String,
    ) -> Result<(), errors::consumer::FileStorageError> {
        let mut file = std::fs::File::create(file_name)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

pub async fn poop(data: String, connection: &mut PgConnection) {
    let scrapped_response = serde_json::from_str::<api_models::ScrappedResponse>(&data).unwrap();
    let db_model = storage_models::NewSubmission::from(scrapped_response);

    database::insert_submission(db_model, connection).unwrap();
}
