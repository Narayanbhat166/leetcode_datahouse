pub const CSRF_COOKIE: &str = "csrftoken";
pub const LEETCODE_SESSION_COOKIE: &str = "LEETCODE_SESSION";
pub const BASE_URL: &str = "https://leetcode.com/graphql/";
pub const ORIGIN: &str = "https://leetcode.com/";
pub const CONTENT_TYPE: &str = "application/json";

pub const GET_SUBMISSION: &str = r#"query submissionDetails($submissionId: Int!) {
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
