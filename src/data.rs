use std::{borrow::Cow, sync::Arc};

use super::Term;

pub type Str = Cow<'static, str>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Point,
    Prim(Primitive),
    Rec(Record),
    Extern(Calculate),
    Append { left: Arc<Value>, right: Arc<Value> },
    Box { context: Arc<Value>, proceed: Term },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Calculate;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Primitive {
    Long(u64),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Record {
    pub values: Vec<Value>,
    pub names: Arc<Vec<Str>>,
}



