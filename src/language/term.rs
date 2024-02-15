mod to_term;

pub use self::to_term::{AsTyp, ToTerm};
use crate::Str;
use std::sync::Arc;
use derive_more::From;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenType<T> {
    Prim(PrimType),
    Field { name: Key, typ: Arc<T> },
    Function { dom: Arc<T>, codom: Arc<T> },
    And { left: Arc<T>, right: Arc<T> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimType {
    Text,
    Long,
    Universe,
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Primitive {
    Long(u64),
    Text(String),
}

pub type Type = GenType<Term>;
pub type NormalType = GenType<Type>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum Key {
    Name(String),
    Index(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Type(Type),
    Prim(Primitive),
    Empty,
    Append { left: Arc<Term>, right: Arc<Term> },
    Set { name: Key, value: Arc<Term> },
    Get(Key),
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
