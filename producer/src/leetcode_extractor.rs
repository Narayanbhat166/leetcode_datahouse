use crate::{consts, extractor::Extractor};
use errors;

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    RequestBuilder,
};

fn get_headers(cookie: String) -> HeaderMap {
    let mut header_map = HeaderMap::new();
    header_map.insert(
        header::ORIGIN,
        HeaderValue::from_static("https://leetcode.com"),
    );

    header_map.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(consts::CONTENT_TYPE),
    );

    header_map.insert(header::COOKIE, HeaderValue::from_str(&cookie).unwrap());
    header_map.insert(header::REFERER, HeaderValue::from_static(consts::ORIGIN));

    header_map
}

/// Takes the submission id and then return the request
pub fn create_scrape_request(
    submission_id: u32,
) -> Result<RequestBuilder, errors::producer::ExtractorError> {
    let submission_id_string = submission_id.to_string();
    let cookies = crate::utils::parse_curl();

    let csrf_token = cookies
        .get(consts::CSRF_COOKIE)
        .ok_or(errors::producer::ExtractorError::CSRFTokenNotFound)?;

    let session = cookies
        .get(consts::LEETCODE_SESSION_COOKIE)
        .ok_or(errors::producer::ExtractorError::CookieNotFound)?;

    let cookie_value = format!("csrftoken={}; LEETCODE_SESSION={}", csrf_token, session);

    let mut body = std::collections::HashMap::new();
    let variables_body = &format!(" {{ \"submissionId\": {} }}", submission_id_string);
    body.insert("query", consts::GET_SUBMISSION);
    body.insert("variables", variables_body);

    let headers = get_headers(cookie_value);

    let client = reqwest::Client::new();
    let request = client.post(consts::BASE_URL).headers(headers).json(&body);

    Ok(request)
}
pub async fn scrape_submission(
    request: RequestBuilder,
    submission_id: u32,
) -> Result<models::api_models::ScrappedResponse, errors::producer::ExtractorError> {
    let resp = request.send().await.unwrap();

    let submission_data = resp
        .json::<models::api_models::SubmissionResponse>()
        .await
        .unwrap();
    Ok(models::api_models::ScrappedResponse {
        submission_id,
        submission_data,
    })
}

pub struct Leetcode;

#[async_trait::async_trait]
impl Extractor<u32, models::api_models::ScrappedResponse> for Leetcode {
    async fn fetch_data(
        &self,
        id: u32,
    ) -> Result<models::api_models::ScrappedResponse, errors::producer::ExtractorError> {
        let scrape_request = create_scrape_request(id)?;
        scrape_submission(scrape_request, id).await
    }
}
