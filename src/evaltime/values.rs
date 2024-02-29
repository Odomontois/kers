use crate::{plugins::Interpteter, PrimType, Primitive};

use super::{checking::TypeError, variables::VarIdx};

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
impl<P: Interpteter> Value<P> {
    pub fn adapt(self, expected: Value<P>) -> Result<Value<P>, TypeError<P>> {
        todo!()
    }
}
