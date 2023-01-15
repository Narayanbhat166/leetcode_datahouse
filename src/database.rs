use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

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

pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

    // let new_submission = NewSubmission {
    //     id: 2,
    //     last_testcase: "None".to_string(),
    //     memory: 1.0,
    //     memory_display: "1.0".to_string(),
    //     memory_percentile: 1.0,
    //     notes: "Hello boi".to_string(),
    //     runtime: 1.0,
    //     runtime_percentile: 1.0,
    //     status_code: 1,
    //     timestamp: 1,
    //     code_hash: "hash".to_string(),
    // };

    // let query = diesel::insert_into(submission::dsl::submission).values(new_submission);

    // let res = query.get_result::<Submission>(&mut conn);

    // println!("query res {:?}", res);
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
