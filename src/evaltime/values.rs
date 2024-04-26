use crate::evaltime::interpreter::Interpteter;
use crate::{PrimType, Primitive};

use super::variables::VarIdx;

#[derive(Clone)]
pub enum TypeValue<P> {
    Prim(PrimType),
    Function {
        dom: Box<TypeValue<P>>,
        codom: Box<TypeValue<P>>,
    },
    Record {
        fields: Vec<Value<P>>,
    },
}

#[derive(Clone)]
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
