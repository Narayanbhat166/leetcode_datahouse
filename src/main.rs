use curl_parser::parse_curl;
use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
mod curl_parser;
mod scrape;

const CSRF_COOKIE: &str = "csrftoken";
const LEETCODE_SESSION_COOKIE: &str = "LEETCODE_SESSION";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Profile {
    real_name: String,
    user_avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct User {
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Question {
    question_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SubmissionDetails {
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
struct Details {
    #[serde(rename = "submissionDetails")]
    submission_details: SubmissionDetails,
}

#[derive(Serialize, Deserialize, Debug)]
struct SubmissionResponse {
    data: Details,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cookies = parse_curl();
    let url = "https://leetcode.com/graphql/";
    let csrf_header = HeaderName::from_static("x-csrftoken");
    let csrf_token = cookies.get(CSRF_COOKIE).unwrap(); //FIXME: raise appropriate error and panic
    let session = cookies.get(LEETCODE_SESSION_COOKIE).unwrap(); //FIXME: raise appropriate error and panic

    let cookie_value: HeaderValue =
        format!("csrftoken={}; LEETCODE_SESSION={}", csrf_token, session)
            .parse()
            .unwrap();

    let query = r#"query submissionDetails($submissionId: Int!) {
        submissionDetails(submissionId: $submissionId) {
          runtime
          runtimeDisplay
          runtimePercentile
          runtimeDistribution
          memory
          memoryDisplay
          memoryPercentile
          memoryDistribution
          code
          timestamp
          statusCode
          user {
            username
            profile {
              realName
              userAvatar
            }
          }
          lang {
            name
            verboseName
          }
          question {
            questionId
          }
          notes
          topicTags {
            tagId
            slug
            name
          }
          runtimeError
          compileError
          lastTestcase
        }
      }"#;

    let submission_id = "821085303";

    let mut body = std::collections::HashMap::new();
    let variables_body = &format!(" {{ \"submissionId\": {} }}", submission_id);
    body.insert("query", query);
    body.insert("variables", variables_body);

    let mut headers = HeaderMap::new();

    headers.insert(header::ORIGIN, "https://leetcode.com".parse().unwrap());
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.102 Safari/537.36".parse().unwrap());
    headers.insert(header::COOKIE, cookie_value);
    headers.insert(header::REFERER, "https://leetcode.com".parse().unwrap());
    headers.insert(csrf_header, csrf_token.parse().unwrap());

    let client = reqwest::Client::new();
    let request = client.post(url).headers(headers).json(&body);
    let resp = request.send().await?;

    let parsed_resp = resp.json::<SubmissionResponse>().await?;

    println!("Response: {:?}", parsed_resp);

    Ok(())
}
