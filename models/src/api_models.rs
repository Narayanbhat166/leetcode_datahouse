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
    pub username: String,
    pub profile: UserProfile,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub real_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionDetails {
    pub code: Option<String>,
    pub memory: Option<f32>,
    pub memory_display: Option<String>,
    pub memory_percentile: Option<f32>,
    pub notes: Option<String>,
    pub runtime: Option<f32>,
    pub runtime_display: Option<String>,
    pub runtime_percentile: Option<f32>,
    pub status_code: Option<i32>,
    pub timestamp: Option<i32>,
    pub user: Option<User>,
    pub question: Option<Question>,
    pub lang: Option<Language>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    #[serde(rename = "submissionDetails")]
    pub submission_details: SubmissionDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmissionResponse {
    pub data: Details,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrappedResponse {
    pub submission_id: u32,
    pub submission_data: SubmissionResponse,
}
