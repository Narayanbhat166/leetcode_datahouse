use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    real_name: String,
    user_avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    question_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionDetails {
    code: String,
    last_testcase: String,
    memory: f64,
    memory_display: String,
    memory_percentile: f64,
    notes: String,
    runtime: f64,
    runtime_display: String,
    runtime_percentile: f64,
    status_code: i32,
    timestamp: i64,
    user: User,
    question: Question,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    #[serde(rename = "submissionDetails")]
    submission_details: SubmissionDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmissionResponse {
    data: Details,
}

pub fn get_submission(submission_id: String) {}
