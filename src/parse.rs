use core::panic;
use std::{fmt::Display, sync::Arc};

use pest::Parser;
use pest_derive::Parser;

use crate::Term;

mod decode;
mod error;
pub use error::SyntaxError;

#[derive(Parser)]
#[grammar = "kers.pest"]
pub struct Kers;


#[allow(unused)]
fn parse_term(input: &str) -> Result<Arc<Term>, SyntaxError> {
    let mut top = Kers::parse(Rule::term, input)?;
    decode::term(top.read(Rule::term)?)
}

trait UnwrapDisplay {
    type Output;
    fn unwrap_print(self) -> Self::Output;
}

impl<A, E: Display> UnwrapDisplay for Result<A, E> {
    type Output = A;
    fn unwrap_print(self) -> Self::Output {
        match self {
            Ok(term) => term,
            Err(err) => panic!("{err}"),
        }
    }
}

#[cfg(test)]
use crate::ToTerm;

#[test]
fn check_various_simple_stuff() {
    Kers::parse(Rule::single_quoted_string, "\'Hello\'").unwrap_print();
    Kers::parse(Rule::string, "\'Hello\'").unwrap_print();
    Kers::parse(Rule::assignment, "greet = 'Hello'").unwrap_print();
    Kers::parse(Rule::identifier, "greet")
        .unwrap_print()
        .as_str();
}

#[test]
fn check_object() {
    let input = "(
       greet = 'Hello',  
       'target' = \"World\";
       'my \"agy\"' = 38,
       xxx = xxx
    )";

    let res = parse_term(input).unwrap_print();
    assert_eq!(
        res,
        [
            ("greet", "Hello".to_term()),
            ("target", "World".to_term()),
            ("my \"agy\"", 38u64.to_term()),
            ("xxx", Term::get("xxx"))
        ]
        .to_arc_term()
    )
}

#[test]
fn check_empty_object() {
    let input = "()";
    let res = parse_term(input).unwrap_print();
    assert_eq!(res, Term::Empty.to_arc_term())
}

#[cfg(test)]
use crate::AsTyp;

use self::decode::PairsExt;
#[test]
fn check_record_type() {
    let input = "{greet: str, 'target': str, 'my \"agy\"': int, xxx: xxx}";
    let res = parse_term(input).unwrap_print();
    assert_eq!(
        res,
        AsTyp([
            ("greet", Term::get("str")),
            ("target", Term::get("str")),
            ("my \"agy\"", Term::get("int")),
            ("xxx", Term::get("xxx")),
        ])
        .to_arc_term()
    )
}
