use crate::{PrimType, Primitive};

pub enum EvalValue {
    Prim(Primitive),
    Record { fields: Vec<EvalValue> },
    Function { dom: Box<PrimType>, codom: Box<EvalValue> },
}
