use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum ExtractorError {
    #[error("The provided cookie has been expired")]
    ExpiredCookie,
    #[error("Cookie is not provided")]
    CookieNotFound,
    #[error("CSRF token is not provided")]
    CSRFTokenNotFound,
    #[error("Submission data is not available for the provided submission id")]
    SubmissionDataNotAvailable,
}
