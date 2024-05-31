use std::borrow::Cow;

use crate::evaltime::interpreter::Interpteter;
use crate::{PrimType, Primitive};

use super::variables::VarIdx;

#[derive(Clone, Debug)]
pub enum TypeValue<P> {
    Prim(PrimType),
    Function {
        dom: Box<TypeValue<P>>,
        codom: Box<TypeValue<P>>,
    },
    Record(Vec<Value<P>>),
}
use TypeValue::*;

impl<P: Clone> TypeValue<P> {
    pub(crate) fn extend(&self, with: TypeValue<P>) -> TypeValue<P> {
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
pub enum Value<V> {
    Prim(Primitive),
    Type(TypeValue<V>),
    Variable(VarIdx),
    Record {
        fields: Vec<Value<V>>,
    },
    Lambda {
        dom: Box<Value<V>>,
        term: Box<Value<V>>,
    },
    External(V),
}

#[allow(unused)]
impl<P: Interpteter<P>> Value<P> {}
