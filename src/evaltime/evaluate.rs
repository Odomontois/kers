use thiserror::Error;

pub struct Runtime {}

#[allow(unused)]
#[derive(Error, Debug)]
pub(crate) enum EvalError {
    #[error("Value is not a function, {info}")]
    ValueIsNotAFunction { plugin_name: String, info: String },
}

use crate::Term;

use super::values::Value;

impl Runtime {
    #[allow(unused)]
    pub fn new() -> Self {
        todo!("new")
    }

    pub fn eval<'a>(&'a mut self, term: &Term, context: &Value) -> Result<Value, EvalError> {
        Ok(match term {
            Term::Empty => Value::Record(vec![]),
            Term::Reflect => context.clone(),
            Term::Append { left, right } => {
                let left = self.eval(left, context)?;
                let right = self.eval(right, &context.extend(left.clone()))?;
                left.extend(right)
            }

            t => todo!("typecheck"),
        })
    }
}
