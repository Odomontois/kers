use std::sync::Arc;

use super::{Primitive, Str, Type};

#[derive(Debug, Clone)]
pub enum Term {
    Type(Type),
    Append { left: Arc<Term>, right: Arc<Term> },
    Prim(Primitive),
    Get { name: Str, index: usize },
    Empty,
    Set { name: Str, value: Arc<Term> },
    Box(Arc<Term>),
    Lambda { dom: Arc<Term>, body: Arc<Term> },
    Apply { func: Arc<Term>, args: Arc<Term> },
}

pub trait ToTerm {
    fn to_term(self) -> Term;

    fn to_arc_term(self) -> Arc<Term>
    where
        Self: Sized,
    {
        Arc::new(self.to_term())
    }

    fn to_arc_ok<E>(self) -> Result<Arc<Term>, E>
    where
        Self: Sized,
    {
        Ok(self.to_arc_term())
    }
}

impl ToTerm for Term {
    fn to_term(self) -> Term {
        self
    }
}

impl ToTerm for u64 {
    fn to_term(self) -> Term {
        Term::Prim(Primitive::Long(self))
    }
}

impl ToTerm for String {
    fn to_term(self) -> Term {
        Term::Prim(Primitive::Text(self))
    }
}

impl ToTerm for Primitive {
    fn to_term(self) -> Term {
        Term::Prim(self)
    }
}

impl ToTerm for Type {
    fn to_term(self) -> Term {
        Term::Type(self)
    }
}
