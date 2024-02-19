use slotmap::SlotMap;
use thiserror::Error;

use crate::plugins::{EvalPlugin, PluginIdx};

#[allow(unused)]
struct Evaluation {
    plugins: SlotMap<PluginIdx, EvalPlugin>,
}

#[allow(unused)]
#[derive(Error, Debug)]
enum EvalError {
    #[error("Value is not a function, {info}")]
    ValueIsNotAFunction { plugin: PluginIdx, info: String },
}
