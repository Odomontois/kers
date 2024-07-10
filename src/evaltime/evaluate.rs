use thiserror::Error;

#[derive(Default)]
pub struct Runtime {
    depth: usize,
}

use crate::Term;

use super::{EvalError, Record, Value};
pub(crate) type Res<T> = Result<T, EvalError>;

impl Runtime {
    pub fn eval<'a>(&'a mut self, term: &Term, context: &Value) -> Res<Value> {
        Ok(match term {
            Term::Empty => Value::Record(Record::default()),
            Term::Reflect => context.clone(),
            Term::Append { left, right } => {
                let left = self.eval(left, context)?;
                let right = self.eval(right, &(context.clone() + left.clone()))?;
                left + right
            }

            t => todo!("typecheck"),
        })
    }
}
