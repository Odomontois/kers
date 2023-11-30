use core::panic;
use std::{borrow::Cow, fmt::Display, sync::Arc};

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::{Primitive, Term, ToTerm, Type};

mod error;
pub use error::SyntaxError;

#[derive(Parser)]
#[grammar = "kers.pest"]
pub struct KersParser;

trait PairsExt: Sized {
    type R;
    type P;
    fn read(&mut self, rule: Self::R) -> Result<Self::P, SyntaxError>;
}

impl<'a, R: Eq> PairsExt for pest::iterators::Pairs<'a, R>
where
    R: pest::RuleType,
{
    type R = R;
    type P = Pair<'a, R>;
    fn read(&mut self, rule: Self::R) -> Result<Self::P, SyntaxError> {
        let pair = self.next().unwrap();
        if pair.as_rule() != rule {
            return Err(format!("expected {:?}, got {:?}", rule, pair.as_rule()))?;
        }
        Ok(pair)
    }
}

type Parsed<'a> = Pair<'a, Rule>;

fn decode_assignment(mut input: Parsed) -> Result<Arc<Term>, SyntaxError> {
    let mut input = input.into_inner();
    let name = input.read(Rule::identifier)?.as_str().to_string();
    let value = input.read(Rule::expression)?;
    Term::Set {
        name: name.into(),
        value: Primitive::Text(value.as_str().to_string()).to_arc_term(),
    }
    .to_arc_ok()
}

fn decode_term(mut input: Parsed) -> Result<Arc<Term>, SyntaxError> {
    let object = input.into_inner().read(Rule::object)?;
    object
        .into_inner()
        .map(|assignment| decode_assignment(assignment))
        .reduce(|left, right| {
            let left = left?;
            let right = right?;
            Term::Append {
                left: left.clone(),
                right: right.clone(),
            }
            .to_arc_ok()
        })
        .unwrap_or(Type::Universe.to_arc_ok())
}

fn parse_term(input: &str) -> Result<Arc<Term>, SyntaxError> {
    let mut top = KersParser::parse(Rule::term, input)?;
    let term = top.read(Rule::term)?;
    decode_term(term)
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

#[test]
fn check() {
    let input = "{
       greet = 'Hello'  
       target = \"World\"
    }";

    KersParser::parse(Rule::single_quoted_string, "\'Hello\'").unwrap_print();
    let res = parse_term(input).unwrap_print();
    println!("{:?}", res);
}

#[cfg(test)]
fn check_decoder<A>(inputs: &[&str], decoder: impl Fn(&str) -> Result<A, SyntaxError>) {
    for &input in inputs {
        decoder(input).unwrap_print();
    }
}
