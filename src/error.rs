//! Central error type for the leetrs engine.
use thiserror::Error;

/// All errors that can be produced by the leetrs engine.
///
/// `EngineError` is the single error type propagated through most of the
/// library. It is thin wrapper around the underlying cause so callers can
/// pattern-match on the variant without depending on the concrete error types
/// from `reqwest` or `serde_json`.
#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Authentication error: Missing session token or CSRF token")]
    Auth,
    #[error("Serialization error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("GraphQL error: {0}")]
    GraphQL(String),
    #[error("System error")]
    System,
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Other: {0}")]
    Other(String),
}

impl EngineError {
    /// Wraps this error with additional context, producing an [`EngineError::Other`].
    pub fn with_context(self, msg: impl Into<String>) -> Self {
        EngineError::Other(format!("{}: {}", msg.into(), self))
    }
}

pub type Result<T> = std::result::Result<T, EngineError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_with_context_wraps_error_into_other() {
        let original = EngineError::System;
        let wrapped = original.with_context("file access failed");
        assert!(matches!(wrapped, EngineError::Other(_)));
        let msg = wrapped.to_string();
        assert!(msg.contains("file access failed"));
        assert!(msg.contains("System error"));
    }
}
