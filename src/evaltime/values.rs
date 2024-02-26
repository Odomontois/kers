use crate::{plugins::Extension, PrimType, Primitive, Term};

pub enum TypeValue {
    Prim(PrimType),
}
pub enum Value<P: Extension> {
    Prim(Primitive),
    Type(TypeValue),
    Record {
        fields: Vec<Value<P>>,
    },
    Function {
        dom: Box<Value<P>>,
        codom: Box<Value<P>>,
    },
    External(P::Value),
}
