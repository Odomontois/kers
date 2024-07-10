use std::sync::Arc;

use thiserror::Error;

#[derive(Default)]
pub struct Runtime {
    depth: usize,
}

use crate::{Term, ToBoxTerm};

use super::{EvalError, Feature, Record, Value};
pub(crate) type Res<T> = Result<T, EvalError>;

impl Runtime {
    fn deeper<A>(&mut self, f: impl FnOnce(&mut Self) -> A) -> A {
        self.depth += 1;
        let res = f(self);
        self.depth -= 1;
        res
    }
    /** synthesizing a value corresponding to a type possily allocating some variables along the way*/
    pub(crate) fn synthesize(&mut self, v: Value) -> Res<Value> {
        Feature::Synthesize.not_implemented()
    }

    pub fn eval<'a>(&'a mut self, term: &Term, context: &Value) -> Res<Value> {
        match term {
            Term::Empty => Ok(Value::Record(Record::default())),
            Term::Reflect => Ok(context.clone()),
            Term::Append { left, right } => {
                let left = self.eval(left, context)?;
                let right = self.eval(right, &(context.clone() + left.clone()))?;
                Ok(left + right)
            }
            Term::Unlambda(l) => {
                let l = self.eval(l, context)?;
                l.apply(context.clone())
            }
            Term::Lambda { dom, body } => {
                let dom = self.eval(dom, context)?;
                let extension = self.synthesize(dom)?;
                let full = context.clone() + extension;
                let body = self.deeper(|ctx| ctx.eval(body, &full))?;
                Ok(Value::Lambda {
                    body: Arc::new(body),
                    outer: self.depth,
                })
            }
            t => Feature::Evaluation.not_implemented(),
        }
    }
}
