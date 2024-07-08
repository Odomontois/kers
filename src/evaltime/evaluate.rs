use thiserror::Error;

pub struct Runtime {}

use crate::Term;

use super::{Record, Value, EvalError};
pub(crate) type Res<T> = Result<T, EvalError>;

impl Runtime {
    #[allow(unused)]
    pub fn new() -> Self {
        todo!("new")
    }

    pub fn eval<'a>(&'a mut self, term: &Term, context: &Value) -> Res<Value> {
        Ok(match term {
            Term::Empty => Value::Record(Record::default()),
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
