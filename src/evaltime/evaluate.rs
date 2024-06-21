use thiserror::Error;

pub struct Runtime {}

#[allow(unused)]
struct Evaluation<P> {
    plugins: P,
}

#[allow(unused)]
#[derive(Error, Debug)]
enum EvalError {
    #[error("Value is not a function, {info}")]
    ValueIsNotAFunction { plugin_name: String, info: String },
}

use crate::Term;

use super::values::Value;

pub enum TypeError {
    #[allow(unused)]
    Mismatch(Term, Value, Value),
}

#[derive(Debug, Default)]
pub(crate) struct TypeChecking {}

impl TypeChecking {
    #[allow(unused)]
    pub fn new() -> Self {
        todo!("new")
    }

    #[allow(unused)]
    pub fn check<'a>(&'a mut self, term: &Term, context: &Value) -> Result<Value, TypeError> {
        Ok(match term {
            Term::Empty => Value::Record(vec![]),
            Term::Reflect => context.clone(),
            Term::Append { left, right } => {
                let left = self.check(left, context)?;
                let right = self.check(right, &context.extend(left.clone()))?;
                left.extend(right)
            }

            t => todo!("typecheck"),
        })
    }
}
