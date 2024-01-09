use diesel::{Connection, ConnectionError, PgConnection, RunQueryDsl};

pub fn create_connection(
    db_config: configs::types::DbConfigData,
) -> Result<PgConnection, ConnectionError> {
    PgConnection::establish(&db_config.db.url)
}

pub fn insert_submission(
    new_submission: models::storage_models::NewSubmission,
    conn: &mut PgConnection,
) -> Result<models::storage_models::Submission, diesel::result::Error> {
    let query =
        diesel::insert_into(models::schema::submission::dsl::submission).values(new_submission);
    query.get_result::<models::storage_models::Submission>(conn)
}
