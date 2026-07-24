//! Data models shared across the crate.
//!
//! All types here are either serialised into HTTP request bodies or deserialised
//! from API responses. Serde field renames mirror LeetCode's camelCase JSON keys.
//!
//! # Module layout
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`language`] | [`Language`] enum and [`Identifier`] |
//! | [`problem`] | [`ProblemSummary`], [`Question`], [`UserDetail`], [`GraphQLQuery`] |
//! | [`submission`] | Payload/response types for submit and test-run |

pub mod language;
pub mod problem;
pub mod submission;

// Re-export everything at the models level to keep all existing `use` paths
// working without any changes in other modules.
pub use language::{Identifier, Language};
pub use problem::{
    GraphQLQuery, ProblemSummary, Question, QuestionSnippet, QuestionTopics, Topic, UserDetail,
};
pub use submission::{
    SubmissionCheckResult, SubmitPayload, SubmitResponse, TestPayload, TestSubmissionCheckResult,
    TestSubmitResponse,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // -----------------------------------------------------------------------
    // Language::to_lang_slug
    // -----------------------------------------------------------------------

    #[test]
    fn to_lang_slug_python() {
        assert_eq!(Language::Python.to_lang_slug(), "python3");
    }

    #[test]
    fn to_lang_slug_rust() {
        assert_eq!(Language::Rust.to_lang_slug(), "rust");
    }

    #[test]
    fn to_lang_slug_pandas() {
        assert_eq!(Language::Pandas.to_lang_slug(), "pythondata");
    }

    #[test]
    fn to_lang_slug_mysql() {
        assert_eq!(Language::Mysql.to_lang_slug(), "mysql");
    }

    #[test]
    fn to_lang_slug_postgres() {
        assert_eq!(Language::Postgres.to_lang_slug(), "postgresql");
    }

    // -----------------------------------------------------------------------
    // Language::from (via lang_slug string returned by LeetCode)
    // -----------------------------------------------------------------------

    #[test]
    fn language_from_python3_slug() {
        assert!(matches!(
            Language::from("python3".to_string()),
            Language::Python
        ));
    }

    #[test]
    fn language_from_rust_slug() {
        assert!(matches!(Language::from("rust".to_string()), Language::Rust));
    }

    #[test]
    fn language_from_pythondata_slug() {
        assert!(matches!(
            Language::from("pythondata".to_string()),
            Language::Pandas
        ));
    }

    #[test]
    fn language_from_mysql_slug() {
        assert!(matches!(
            Language::from("mysql".to_string()),
            Language::Mysql
        ));
    }

    #[test]
    fn language_from_postgresql_slug() {
        assert!(matches!(
            Language::from("postgresql".to_string()),
            Language::Postgres
        ));
    }

    /// Unknown slugs must fall back to Mysql rather than panicking.
    #[test]
    fn language_from_unknown_slug_falls_back_to_mysql() {
        assert!(matches!(
            Language::from("javascript".to_string()),
            Language::Mysql
        ));
        assert!(matches!(Language::from("".to_string()), Language::Mysql));
        assert!(matches!(
            Language::from("PYTHON3".to_string()),
            Language::Mysql
        )); // case-sensitive
    }

    // -----------------------------------------------------------------------
    // Language::from_extension
    // -----------------------------------------------------------------------

    #[test]
    fn from_extension_py_is_python() {
        assert!(matches!(Language::from_extension("py"), Language::Python));
    }

    #[test]
    fn from_extension_rs_is_rust() {
        assert!(matches!(Language::from_extension("rs"), Language::Rust));
    }

    #[test]
    fn from_extension_sql_is_mysql() {
        assert!(matches!(Language::from_extension("sql"), Language::Mysql));
    }

    /// Unknown extensions must fall back gracefully, not panic.
    #[test]
    fn from_extension_unknown_falls_back_to_mysql() {
        assert!(matches!(Language::from_extension("js"), Language::Mysql));
        assert!(matches!(Language::from_extension(""), Language::Mysql));
        assert!(matches!(Language::from_extension("txt"), Language::Mysql));
    }

    // -----------------------------------------------------------------------
    // to_lang_slug ↔ from round-trip
    // -----------------------------------------------------------------------

    fn roundtrip(lang: Language) -> bool {
        // Converts a Language variant → LeetCode slug string → back to Language,
        // then checks the slug string is preserved.
        let slug = lang.to_lang_slug();
        let recovered = Language::from(slug.to_string());
        recovered.to_lang_slug() == slug
    }

    #[test]
    fn lang_slug_roundtrip_all_variants() {
        assert!(roundtrip(Language::Python));
        assert!(roundtrip(Language::Rust));
        assert!(roundtrip(Language::Pandas));
        assert!(roundtrip(Language::Mysql));
        assert!(roundtrip(Language::Postgres));
    }

    // -----------------------------------------------------------------------
    // Serde: ProblemSummary
    // -----------------------------------------------------------------------

    #[test]
    fn problem_summary_serde_roundtrip() {
        let ps = ProblemSummary {
            id: 1,
            acceptance: 0.55,
            accepted: 5500,
            difficulty: 2,
            slug: "two-sum".to_string(),
            status: Some("ac".to_string()),
            submitted: 10000,
            title: "Two Sum".to_string(),
            is_paid: false,
            topics: vec!["Array".to_string(), "Hash Table".to_string()],
        };
        let json = serde_json::to_string(&ps).expect("serialization failed");
        let back: ProblemSummary = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(back.id, 1);
        assert_eq!(back.slug, "two-sum");
        assert!((back.acceptance - 0.55).abs() < 1e-9);
        assert_eq!(back.difficulty, 2);
        assert_eq!(back.status.as_deref(), Some("ac"));
        assert_eq!(back.topics, vec!["Array", "Hash Table"]);
        assert!(!back.is_paid);
    }

    #[test]
    fn problem_summary_no_status_roundtrip() {
        let ps = ProblemSummary {
            id: 42,
            acceptance: 0.0,
            accepted: 0,
            difficulty: 1,
            slug: "fizz-buzz".to_string(),
            status: None,
            submitted: 0,
            title: "Fizz Buzz".to_string(),
            is_paid: true,
            topics: vec![],
        };
        let json = serde_json::to_string(&ps).unwrap();
        let back: ProblemSummary = serde_json::from_str(&json).unwrap();
        assert!(back.status.is_none());
        assert!(back.topics.is_empty());
        assert!(back.is_paid);
    }

    // -----------------------------------------------------------------------
    // Serde: UserDetail (camelCase rename sanity)
    // -----------------------------------------------------------------------

    #[test]
    fn user_detail_deserializes_camel_case_json() {
        let raw = r#"{"username":"alice","isPremium":true,"isVerified":false}"#;
        let ud: UserDetail = serde_json::from_str(raw).expect("deserialization failed");
        assert_eq!(ud.username.as_deref(), Some("alice"));
        assert_eq!(ud.is_premium, Some(true));
        assert!(!ud.is_verified);
    }

    #[test]
    fn user_detail_null_username_and_premium() {
        let raw = r#"{"username":null,"isPremium":null,"isVerified":true}"#;
        let ud: UserDetail = serde_json::from_str(raw).expect("deserialization failed");
        assert!(ud.username.is_none());
        assert!(ud.is_premium.is_none());
        assert!(ud.is_verified);
    }

    #[test]
    fn user_detail_serde_roundtrip() {
        let ud = UserDetail {
            username: Some("bob".to_string()),
            is_premium: Some(false),
            is_verified: true,
        };
        let json = serde_json::to_string(&ud).unwrap();
        let back: UserDetail = serde_json::from_str(&json).unwrap();
        assert_eq!(back.username.as_deref(), Some("bob"));
        assert_eq!(back.is_premium, Some(false));
        assert!(back.is_verified);
    }

    // -----------------------------------------------------------------------
    // Serde: TestSubmissionCheckResult / SubmissionCheckResult
    // -----------------------------------------------------------------------

    #[test]
    fn test_submission_check_result_minimal_deserialize() {
        // Only the required `state` field is set; everything optional should be None.
        let raw = r#"{"state":"PENDING"}"#;
        let r: TestSubmissionCheckResult =
            serde_json::from_str(raw).expect("should accept minimal structure");
        assert_eq!(r.state, "PENDING");
        assert!(r.correct_answer.is_none());
        assert!(r.total_correct.is_none());
        assert!(r.code_answer.is_none());
    }

    #[test]
    fn test_submission_check_result_full_deserialize() {
        let raw = r#"{
            "state": "SUCCESS",
            "correct_answer": true,
            "total_correct": 3,
            "total_testcases": 3,
            "code_answer": ["1", "2"],
            "expected_code_answer": ["1", "2"],
            "status_msg": "Accepted",
            "status_runtime": "0 ms",
            "status_memory": "2 MB",
            "run_success": true,
            "runtime_percentile": 100.0,
            "memory_percentile": 99.5,
            "lang": "rust",
            "code_output": null,
            "full_runtime_error": null
        }"#;
        let r: TestSubmissionCheckResult = serde_json::from_str(raw).unwrap();
        assert_eq!(r.state, "SUCCESS");
        assert_eq!(r.correct_answer, Some(true));
        assert_eq!(r.total_correct, Some(3));
        assert_eq!(r.total_testcases, Some(3));
        assert_eq!(r.status_msg.as_deref(), Some("Accepted"));
        assert!((r.runtime_percentile.unwrap() - 100.0).abs() < 1e-9);
        assert!((r.memory_percentile.unwrap() - 99.5).abs() < 1e-9);
        assert_eq!(r.lang.as_deref(), Some("rust"));
    }

    #[test]
    fn submission_check_result_wrong_answer_fields() {
        let raw = r#"{
            "state": "SUCCESS",
            "status_msg": "Wrong Answer",
            "code_output": "[3,0]",
            "expected_output": "[0,1]",
            "input": "[2,7,11,15]\n9",
            "total_correct": 0,
            "total_testcases": 57,
            "compile_error": null,
            "full_runtime_error": null
        }"#;
        let r: SubmissionCheckResult = serde_json::from_str(raw).unwrap();
        assert_eq!(r.status_msg.as_deref(), Some("Wrong Answer"));
        assert_eq!(r.code_output.as_deref(), Some("[3,0]"));
        assert_eq!(r.expected_output.as_deref(), Some("[0,1]"));
        assert!(r.input.as_deref().unwrap().contains("[2,7,11,15]"));
        assert_eq!(r.total_correct, Some(0));
        assert_eq!(r.total_testcases, Some(57));
    }

    // -----------------------------------------------------------------------
    // GraphQLQuery serialization
    // -----------------------------------------------------------------------

    #[test]
    fn graphql_query_serializes_with_variables() {
        let q = GraphQLQuery {
            query: "query example { foo }".to_string(),
            variables: Some(serde_json::json!({ "id": 1 })),
            operation_name: Some("example".to_string()),
        };
        let json = serde_json::to_string(&q).unwrap();
        assert!(json.contains("operationName"));
        assert!(json.contains("example"));
        assert!(json.contains("variables"));
    }

    #[test]
    fn graphql_query_serializes_without_optional_fields() {
        let q = GraphQLQuery {
            query: "{ userStatus { username } }".to_string(),
            variables: None,
            operation_name: None,
        };
        let json = serde_json::to_string(&q).unwrap();
        // None fields should serialize as `null`, not be omitted (no #[serde(skip_serializing_if)])
        assert!(json.contains("variables"));
        assert!(json.contains("operationName"));
    }
}
