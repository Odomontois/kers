use derive_more::Display;
use thiserror::Error;

#[allow(unused)]
#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Value is not a function, {info}")]
    ValueIsNotAFunction { plugin_name: String, info: String },

    #[error("Not implemented ({feature})")]
    NotImplemented {
        feature: Feature,
        comment: Option<String>,
    },
}

#[derive(Debug, Display)]
pub enum Feature {
    ToAbstract,
    Synthesize,
}

impl Feature {
    pub fn not_implemented_with<A>(self, comment: Option<String>) -> Result<A, EvalError> {
        let feature = self;
        Err(EvalError::NotImplemented { feature, comment })
    }

    pub fn not_implemented<A>(self) -> Result<A, EvalError> {
        self.not_implemented_with(None)
    }

    pub fn not_implemented_for<A>(self, comment: String) -> Result<A, EvalError> {
        self.not_implemented_with(Some(comment))
    }
}
