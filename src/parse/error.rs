use pest::error::Error as PestError;
use thiserror::Error;

use super::Rule;

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("parse error {0}")]
    ParseError(#[from] PestError<Rule>),
    #[error("{0}")]
    ParseNumberError(#[from] std::num::ParseIntError),
    #[error("invalid character: {0}")]
    CharError(String),
    #[error("{msg}")]
    Other { msg: String },
}

impl From<String> for SyntaxError {
    fn from(s: String) -> Self {
        SyntaxError::Other { msg: s }
    }
}

impl From<&str> for SyntaxError {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}
