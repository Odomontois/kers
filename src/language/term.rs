mod to_term;

pub use self::to_term::{AsTyp, ToArcTerm, ToTerm};
use derive_more::From;
use std::sync::Arc;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum BuiltInOperation {
    Plus,
    Minus,
    Times,
    Div,
    Mod,
    Pow,
    Neg,
    Len,
    CharAt,
    Concat,
    Eq,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Primitive {
    Long(u64),
    Text(String),
    BuiltIn(BuiltInOperation),
}

pub type Type = GenType<Term>;
pub type NormalType = GenType<Type>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From)]
pub enum Key {
    Name(String),
    Index(usize),
    Shadow { level: usize, prev: Box<Key> },
}

impl From<&'_ str> for Key {
    fn from(value: &'_ str) -> Self {
        value.to_string().into()
    }
}

#[test]
fn key_size() {
    assert_eq!(std::mem::size_of::<Key>(), std::mem::size_of::<String>());
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

pub fn get(name: &str) -> Term {
    Term::Get(name.to_string().into())
}

#[test]
fn lol() {
    println!("{}", "aaa".to_string())
}
