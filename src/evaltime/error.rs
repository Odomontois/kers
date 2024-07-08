use derive_more::Display;
use thiserror::Error;

#[allow(unused)]
#[derive(Error, Debug)]
pub enum EvalError {
    #[error("Value is not a function, {info}")]
    ValueIsNotAFunction { plugin_name: String, info: String },

    #[error("Not implemented ({feature})")]
    NotImplemented { feature: Feature },
}

#[derive(Debug, Display)]
pub enum Feature {
    ToAbstract,
}
