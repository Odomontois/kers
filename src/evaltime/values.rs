use std::fmt::Debug;
use std::ops::{Add, BitAnd};
use std::sync::Arc;

use crate::{test_size, Key, PrimType, Primitive, Term};
use derive_more::From;

#[derive(Clone, Debug, From)]
pub enum TypeValue {
    Prim(PrimType),
    Function {
        dom: Arc<Value>,
        codom: Arc<Value>,
    },
    #[from]
    Record(RecordType),
}

use super::evaluate::Res;
use super::external::ExternalValue;
use super::{EvalError, Feature, Record, RecordType, Runtime};

impl Add for TypeValue {
    type Output = TypeValue;
    fn add(self, rhs: TypeValue) -> TypeValue {
        use TypeValue::*;
        match (self, rhs) {
            (Record(left), Record(right)) => Record(left + right),
            _ => todo!("add non records"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AbstractValue {
    Variable(usize),
    Got { source: Arc<Value>, key: Key },
    Applied { func: Arc<Value>, arg: Arc<Value> },
}

#[derive(Clone, Debug, From)]
pub enum Value {
    Prim(Primitive),
    #[from]
    Type(TypeValue),
    Abstract(AbstractValue),
    Record(Record),
    External(ExternalValue),
    Lambda {
        body: Arc<Value>,
        outer: usize,
    },
}

#[derive(Clone, Debug, From)]
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

impl BitAnd for Value {
    type Output = Value;
    fn bitand(self, rhs: Value) -> Value {
        todo!("and")
    }
}

impl Value {
    pub(crate) fn apply(&self, arg: Value) -> Res<Value> {
        Feature::Apply.not_implemented()
    }

    pub(crate) fn as_abstract(&self, arg: &Value) -> Res<Value> {
        Feature::ToAbstract.not_implemented()
    }

    pub(crate) fn get(&self, key: &Key) -> Res<Value> {
        Feature::Get.not_implemented()
    }
}
