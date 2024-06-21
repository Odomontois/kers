use std::fmt::Debug;
use std::sync::Arc;

use crate::evaltime::interpreter::Interpteter;
use crate::{test_size, PrimType, Primitive, Term};

#[derive(Clone, Debug)]
pub(crate) enum TypeValue {
    Prim(PrimType),
    Function { dom: Arc<Value>, codom: Arc<Value> },
    Record(RecordType),
}

use super::external::ExternalValue;
use super::record::RecordType;

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
pub(crate) enum Value {
    Prim(Primitive),
    Type(TypeValue),
    Variable(usize),
    Record(Vec<Value>),
    Lambda { dom: Box<Value>, term: Box<Term> },
}
pub struct TypedValue {
    value: Value,
}

test_size!(test_value Value TypedValue);

impl Value {
    pub(crate) fn extend(&self, with: Value) -> Value {
        use Value::*;

        match (self, with) {
            (Record(left), Record(mut right)) => {
                right.extend(left.clone());
                Record(right)
            }
            (_, with) => with,
        }
    }
}
