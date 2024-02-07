mod to_term;

use std::sync::Arc;

use crate::{Primitive, Str};

pub use self::to_term::{ToTerm, Typ};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenType<T> {
    Universe,
    Field { name: Str, typ: Arc<T> },
    Function { dom: Arc<T>, codom: Arc<T> },
    And { left: Arc<T>, right: Arc<T> },
}

pub type Type = GenType<Term>;
pub type NormalType = GenType<Type>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Type(Type),
    Append { left: Arc<Term>, right: Arc<Term> },
    Prim(Primitive),
    Get(Str),
    Empty,
    Set { name: Str, value: Arc<Term> },
    Box(Arc<Term>),
    Lambda { dom: Arc<Term>, body: Arc<Term> },
    Unlambda(Arc<Term>),
    Then { first: Arc<Term>, next: Arc<Term> },
    Reflect,
}

impl Default for Term {
    fn default() -> Self {
        Term::Empty
    }
}

impl Term {
    pub fn get(name: &str) -> Term {
        Term::Get(name.to_string().into())
    }

    pub fn apply(func: Arc<Term>, args: Arc<Term>) -> Term {
        Term::Then {
            first: args,
            next: Term::Unlambda(func).to_arc_term(),
        }
    }
}
