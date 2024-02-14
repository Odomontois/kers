use crate::{PrimType, Primitive};

pub enum Value {
    Prim(Primitive),
    Record { fields: Vec<Value> },
    Function { dom: Box<PrimType>, codom: Box<Value> },
}
