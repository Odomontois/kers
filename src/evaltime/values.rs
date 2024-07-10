use std::fmt::Debug;
use std::ops::Add;
use std::sync::Arc;

use crate::{test_size, Key, PrimType, Primitive, Term};

#[derive(Clone, Debug)]
pub enum TypeValue {
    Prim(PrimType),
    Function { dom: Arc<Value>, codom: Arc<Value> },
    Record(RecordType),
}

use super::evaluate::Res;
use super::external::ExternalValue;
use super::{EvalError, Feature, Record, RecordType, Runtime};

impl TypeValue {
    pub(crate) fn extend(&self, with: TypeValue) -> TypeValue {
        use TypeValue::*;
        match (self, with) {
            (Record(left), Record(mut right)) => {
                right.extend(left.clone());
                Record(right)
            }
            (_, with) => with,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AbstractValue {
    Variable(usize),
    Got { source: Arc<Value>, key: Key },
    Applied { func: Arc<Value>, arg: Arc<Value> },
}

#[derive(Clone, Debug)]
pub enum Value {
    Prim(Primitive),
    Type(TypeValue),
    Abstract(AbstractValue),
    Record(Record),
    External(ExternalValue),
    Lambda { body: Arc<Value>, outer: usize },
}
pub struct TypedValue {
    value: Value,
    typ: Option<Arc<Value>>,
}

test_size!(test_value Value TypedValue);

impl Add<Value> for Value {
    type Output = Value;
    fn add(self, rhs: Value) -> Value {
        use Value::*;
        match (self, rhs) {
            (Record(left), Record(right)) => Record(left + right),
            _ => todo!("add non records"),
        }
    }
}

impl Value {
    pub(crate) fn apply(&self, arg: Value) -> Res<Value> {
        Feature::Apply.not_implemented()
    }

    pub(crate) fn as_abstract(&self, arg: &Value) -> Res<Value> {
        Feature::ToAbstract.not_implemented()
    }


}
