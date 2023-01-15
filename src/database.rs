use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::submission;

#[derive(Queryable, Debug)]
pub struct Submission {
    pub id: i32,
    pub last_testcase: Option<String>,
    pub memory: Option<f32>,
    pub memory_display: Option<String>,
    pub memory_percentile: Option<f32>,
    pub notes: Option<String>,
    pub runtime: Option<f32>,
    pub runtime_percentile: Option<f32>,
    pub status_code: Option<i32>,
    pub timestamp: Option<i32>,
    pub code_hash: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name= submission)]
pub struct NewSubmission {
    pub id: i32,
    pub last_testcase: String,
    pub memory: f32,
    pub memory_display: String,
    pub memory_percentile: f32,
    pub notes: String,
    pub runtime: f32,
    pub runtime_percentile: f32,
    pub status_code: i32,
    pub timestamp: i32,
    pub code_hash: String,
}

pub fn create_connection(database_url: &str) -> Result<PgConnection, ConnectionError> {
    PgConnection::establish(database_url)
}

pub fn insert_submission(
    new_submission: NewSubmission,
    conn: &mut PgConnection,
) -> Result<Submission, diesel::result::Error> {
    let query = diesel::insert_into(submission::dsl::submission).values(new_submission);
    let res = query.get_result::<Submission>(conn);

    println!("query res {:?}", res);
    res
}
