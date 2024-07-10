use std::{
    borrow::{Borrow, Cow},
    iter::once,
    sync::Arc,
};

use thiserror::Error;

#[derive(Default)]
pub struct Runtime {
    depth: usize,
}

use crate::{GenType, Term, ToBoxTerm, Type};

use super::{values::TypeValue, EvalError, Feature, Record, RecordType, Value};
pub(crate) type Res<T> = Result<T, EvalError>;

impl Runtime {
    fn deeper<A>(&mut self, f: impl FnOnce(&mut Self) -> A) -> A {
        self.depth += 1;
        let res = f(self);
        self.depth -= 1;
        res
    }
    /** synthesizing a value corresponding to a type possily allocating some variables along the way*/
    pub(crate) fn synthesize(&mut self, v: &Value) -> Res<Value> {
        Feature::Synthesize.not_implemented()
    }

    fn eval_typ(&mut self, term: &Type, context: Cow<Value>) -> Res<Value> {
        match term {
            Type::Prim(prim) => Ok(TypeValue::Prim(*prim).into()),
            Type::Field { name, typ } => {
                let typ = self.eval(typ, Cow::Borrowed(&context))?;
                Ok(TypeValue::Record(once((name.clone(), typ)).collect()).into())
            }
            Type::Function { dom, codom } => {
                let dom = self.eval_dom(dom, context)?;
                let codom = Arc::new(self.eval(codom, Cow::Borrowed(&dom))?);
                let dom = Arc::new(dom);
                Ok(TypeValue::Function { dom, codom }.into())
            }
            Type::And { left, right } => {
                let left = self.eval(left, Cow::Borrowed(&context))?;
                let extension = self.synthesize(&left)?;
                let full = context.into_owned() + extension;
                let right = self.eval(right, Cow::Owned(full))?;
                Ok(left & right)
            }
        }
    }

    pub fn eval(&mut self, term: &Term, context: Cow<Value>) -> Res<Value> {
        match term {
            Term::Empty => Ok(Value::Record(Record::default())),
            Term::Reflect => Ok(context.into_owned()),
            Term::Append { left, right } => {
                let left = self.eval(left, Cow::Borrowed(&context))?;
                let right = self.eval(right, Cow::Owned(context.into_owned() + left.clone()))?;
                Ok(left + right)
            }
            Term::Unlambda(l) => {
                let l = self.eval(l, Cow::Borrowed(&context))?;
                l.apply(context.into_owned())
            }
            Term::Lambda { dom, body } => {
                let dom = self.eval_dom(dom, context)?;
                let body = self.deeper(|rt| rt.eval(body, Cow::Owned(dom)))?;
                Ok(Value::Lambda {
                    body: Arc::new(body),
                    outer: self.depth,
                })
            }
            Term::Then { first, next } => {
                let first = self.eval(first, context)?;
                self.eval(term, Cow::Owned(first))
            }

            Term::Get(key) => context.get(key),
            Term::Type(t) => self.eval_typ(t, context),
            Term::Prim(_) => Feature::Evaluation("prim").not_implemented(),
            Term::Set { name, value } => {
                let value = self.eval(value, context)?;
                Ok(Value::Record([value].into()))
            }
        }
    }

    fn eval_dom(&mut self, dom: &Box<Term>, context: Cow<Value>) -> Result<Value, EvalError> {
        let dom_type = self.eval(dom, Cow::Borrowed(&context))?;
        let extension = self.synthesize(&dom_type)?;
        let full = context.into_owned() + extension;
        Ok(full)
    }
}
