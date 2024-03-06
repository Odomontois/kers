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
pub enum Value<P> {
    Prim(Primitive),
    Type(TypeValue<P>),
    Variable(VarIdx),
    Record {
        fields: Vec<Value<P>>,
    },
    Lambda {
        dom: Box<Value<P>>,
        term: Box<Value<P>>,
    },
    External(P),
}

#[allow(unused)]
impl<P: Interpteter> Value<P> {}
