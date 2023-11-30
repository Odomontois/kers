use pest::error::Error as PestError;
use thiserror::Error;

use super::Rule;

#[derive(Error, Debug, Clone)]
pub enum SyntaxError {
    #[error("parse error {0}")]
    ParseError(#[from] PestError<Rule>),
    #[error("invalid character: {0}")]
    CharError(String),
    #[error("syntax error: {0}")]
    Other(String),
}

impl From<String> for SyntaxError {
    fn from(s: String) -> Self {
        SyntaxError::Other(s)
    }
}

impl From<&str> for SyntaxError {
    fn from(s: &str) -> Self {
        SyntaxError::Other(s.to_string())
    }
}