use crate::schema;
use diesel::{ConnectionError, Insertable, PgConnection, Queryable, RunQueryDsl};

#[derive(Queryable, Debug)]
pub struct Submission {
    pub id: i64,
    pub memory: Option<f32>,
    pub memory_display: Option<String>,
    pub memory_percentile: Option<f32>,
    pub notes: Option<String>,
    pub runtime: Option<f32>,
    pub runtime_percentile: Option<f32>,
    pub status_code: Option<i32>,
    pub timestamp: Option<i32>,
    pub code_hash: Option<String>,
    pub username: Option<String>,
    pub question_id: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name= schema::submission)]
pub struct NewSubmission {
    pub id: i64,
    pub memory: Option<f32>,
    pub memory_display: Option<String>,
    pub memory_percentile: Option<f32>,
    pub notes: Option<String>,
    pub runtime: Option<f32>,
    pub runtime_percentile: Option<f32>,
    pub status_code: Option<i32>,
    pub timestamp: Option<i32>,
    pub code_hash: Option<String>,
    pub username: Option<String>,
    pub question_id: Option<String>,
}

pub fn create_connection(database_url: &str) -> Result<PgConnection, ConnectionError> {
    <PgConnection as diesel::Connection>::establish(database_url)
}

pub fn insert_submission(
    new_submission: NewSubmission,
    conn: &mut PgConnection,
) -> Result<Submission, diesel::result::Error> {
    let query = diesel::insert_into(schema::submission::dsl::submission).values(new_submission);
    query.get_result(conn)
}

impl From<crate::api_models::ScrappedResponse> for NewSubmission {
    fn from(item: crate::api_models::ScrappedResponse) -> Self {
        let submission_data = item.submission_data.data.submission_details;

        let code_hash =
            crate::utils::generate_hash(&submission_data.code.clone().unwrap_or_default());
        let username = submission_data.user.map(|user| user.username);

        let submission_id = i64::try_from(item.submission_id).unwrap();

        Self {
            id: submission_id,
            memory: submission_data.memory,
            memory_display: submission_data.memory_display,
            memory_percentile: submission_data.memory_percentile,
            notes: submission_data.notes,
            runtime: submission_data.runtime,
            runtime_percentile: submission_data.runtime_percentile,
            status_code: submission_data.status_code,
            timestamp: submission_data.timestamp,
            code_hash: Some(code_hash),
            username,
            question_id: submission_data
                .question
                .map(|question| question.question_id),
        }
    }
}
