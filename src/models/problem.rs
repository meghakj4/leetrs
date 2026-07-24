//! Problem summary, question detail, and topic models.
use serde::{Deserialize, Serialize};

/// A GraphQL request body sent to `https://leetcode.com/graphql`.
#[derive(Serialize, Debug)]
pub struct GraphQLQuery {
    pub query: String,
    pub variables: Option<serde_json::Value>,
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,
}

/// A single code snippet returned by LeetCode's GraphQL API for one language.
#[derive(Deserialize, Debug)]
pub struct QuestionSnippet {
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}

/// Minimal user profile returned by the `userStatus` GraphQL query.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDetail {
    pub username: Option<String>,
    #[serde(rename = "isPremium")]
    pub is_premium: Option<bool>,
    #[serde(rename = "isVerified")]
    pub is_verified: bool,
}

/// Full problem details fetched from the `questionData` GraphQL query.
#[derive(Deserialize, Debug)]
pub struct Question {
    #[serde(rename = "questionId")]
    pub question_id: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub title: String,
    pub content: String,
    #[serde(rename = "exampleTestcases")]
    pub example_test_cases: String,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<QuestionSnippet>,
}

/// Lightweight problem summary used to populate the TUI problem list.
///
/// These are deserialized from the cached `data.json` file and from the
/// `/api/problems/all/` REST endpoint.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemSummary {
    pub id: u64,
    pub acceptance: f64,
    pub accepted: u64,
    /// Difficulty level: `1` = Easy, `2` = Medium, `3` = Hard.
    pub difficulty: u8,
    pub slug: String,
    /// `"ac"` if solved, `"notac"` if attempted but not solved, `None` if untouched.
    pub status: Option<String>,
    pub submitted: u64,
    pub title: String,
    pub is_paid: bool,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Topic {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuestionTopics {
    pub name: String,
    pub id: String,
    pub slug: String,
    #[serde(rename = "translatedName")]
    pub translated_name: Option<String>,
    #[serde(rename = "questionIds")]
    pub question_ids: Vec<u64>,
}
