
use std::sync::Arc;

use super::{Primitive, Str, Type};

pub enum Term {
    Type(Type),
    Append { left: Arc<Term>, right: Arc<Term> },
    Prim(Primitive),
    Get { name: Str, index: usize },
    Set { name: Str, value: Arc<Term> },
    Box(Arc<Term>),
    Lambda { dom: Arc<Term>, body: Arc<Term> },
    Apply { func: Arc<Term>, args: Arc<Term> },
}

