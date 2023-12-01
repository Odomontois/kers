use std::sync::Arc;

use super::{Primitive, Str};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenType<T> {
    Universe,
    Bool,
    Long,
    Text,
    Field { name: Str, typ: Arc<T> },
    Function { dom: Arc<T>, codom: Arc<T> },
    And { left: Arc<T>, right: Arc<T> },
    Array { elem: Arc<T> },
}

pub type Type = GenType<Term>;
pub type NormalType = GenType<Type>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl Default for Term {
    fn default() -> Self {
        Term::Empty
    }
}

impl Term {
    pub fn get(name: &str) -> Term {
        Term::Get {
            name: name.to_string().into(),
            index: 0,
        }
    }
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

impl ToTerm for &str {
    fn to_term(self) -> Term {
        Term::Prim(Primitive::Text(self.to_string()))
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

impl<S: ToString, A: ToTerm> ToTerm for (S, A) {
    fn to_term(self) -> Term {
        Term::Set {
            name: self.0.to_string().into(),
            value: self.1.to_arc_term(),
        }
    }
}

impl<const N: usize, X: ToTerm> ToTerm for [X; N] {
    fn to_term(self) -> Term {
        self.into_iter()
            .map(X::to_term)
            .reduce(|left, right| Term::Append {
                left: left.to_arc_term(),
                right: right.to_arc_term(),
            })
            .unwrap_or(Term::Empty)
    }
}

impl<X: ToTerm + Clone> ToTerm for &[X] {
    fn to_term(self) -> Term {
        self.into_iter()
            .map(|x| x.clone().to_term())
            .reduce(|left, right| Term::Append {
                left: left.to_arc_term(),
                right: right.to_arc_term(),
            })
            .unwrap_or(Term::Empty)
    }
}
