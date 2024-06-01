use std::fmt::Debug;
use std::sync::Arc;

use crate::evaltime::interpreter::Interpteter;
use crate::{test_size, PrimType, Primitive};

use super::variables::VarIdx;

#[derive(Clone, Debug)]
pub enum TypeValue<P> {
    Prim(PrimType),
    Function {
        dom: Arc<TypeValue<P>>,
        codom: Arc<TypeValue<P>>,
    },
    Record(Vec<Arc<Value<P>>>),
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
pub(crate) struct TypedValue<V> {
    value: Value<V>,
    ty: Option<Arc<TypeValue<V>>>,
}

test_size!(test_value Value<()> TypedValue<()>);

#[allow(unused)]
impl<P: Interpteter<P>> Value<P> {}


