use core::panic;
use std::{fmt::Display, sync::Arc};

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::{Term, ToTerm, Type};

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

impl<'a, R: Eq> PairsExt for Pairs<'a, R>
where
    R: pest::RuleType,
{
    type R = R;
    type P = Pair<'a, R>;
    fn read(&mut self, rule: Self::R) -> Result<Self::P, SyntaxError> {
        let pair = self
            .next()
            .ok_or_else(|| format!("expected {rule:?} got nothing"))?;
        if pair.as_rule() != rule {
            return Err(format!("expected {rule:?}, got {:?}", pair.as_rule()))?;
        }
        Ok(pair)
    }
}

type Parsed<'a> = Pair<'a, Rule>;
type Parsing<A> = Result<A, SyntaxError>;
type ParsingTerm = Parsing<Arc<Term>>;

fn decode_char(input: Parsed) -> Parsing<char> {
    let input = input.as_str();
    let mut chars = input.chars();
    let error = || SyntaxError::CharError(input.to_string());
    let Some(initial) = chars.next() else {
        return Err(error());
    };
    let Some(second) = chars.next() else {
        return Ok(initial);
    };
    if initial != '\\' {
        return Err(error());
    }
    match second {
        'u' => {
            let Ok(code) = input[2..].parse::<u32>() else {
                return Err(error());
            };
            std::char::from_u32(code).ok_or_else(error)
        }
        '\"' | '\'' | '\\' | '/' => Ok(second),
        't' => Ok('\t'),
        'r' => Ok('\r'),
        'n' => Ok('\n'),
        'b' => Ok('\x08'),
        'f' => Ok('\x0c'),
        _ => Err(error()),
    }
}

fn decode_string(string: Parsed) -> Parsing<String> {
    let s = string.as_str();
    let error = || SyntaxError::Other(format!("{s} is not a string"));
    let contents = string.into_inner().next().ok_or_else(error)?;
    contents.into_inner().map(decode_char).collect()
}

fn decode_key(key: Parsed) -> Parsing<String> {
    let mut key = key.into_inner();
    let first = key.next().ok_or("Empty key")?;
    match first.as_rule() {
        Rule::identifier => Ok(first.as_str().to_string()),
        Rule::string => decode_string(first),
        rule => Err(SyntaxError::Other(format!("Not an identifier {rule:?}"))),
    }
}

fn decode_assignment(input: Parsed) -> ParsingTerm {
    let mut input = input.into_inner();
    let name = decode_key(input.read(Rule::key)?)?.into();
    let value = decode_term(input.read(Rule::term)?)?;
    Term::Set { name, value }.to_arc_ok()
}

fn decode_record(expr: Parsed) -> ParsingTerm {
    decode_sequence(
        expr,
        Rule::assignment,
        decode_assignment,
        |left, right| Term::Append { left, right }.to_arc_term(),
        Assocciation::Left,
    )
}

fn decode_lam_sequence(expr: Parsed) -> ParsingTerm {
    decode_sequence(
        expr,
        Rule::intersection,
        decode_intersection,
        |dom, codom| Type::Function { dom, codom }.to_arc_term(),
        Assocciation::Right,
    )
}

fn decode_intersection(expr: Parsed) -> ParsingTerm {
    decode_sequence(
        expr,
        Rule::application,
        decode_application,
        |left, right| Type::And { left, right }.to_arc_term(),
        Assocciation::Left,
    )
}

fn decode_application(expr: Parsed) -> ParsingTerm {
    decode_sequence(
        expr,
        Rule::then_chain,
        decode_then_chain,
        |func, args| Term::apply(func, args).to_arc_term(),
        Assocciation::Left,
    )
}

fn decode_then_chain(expr: Parsed) -> ParsingTerm {
    decode_sequence(
        expr,
        Rule::atomic_term,
        decode_atomic_term,
        |left, right| Term::Then { left, right }.to_arc_term(),
        Assocciation::Left,
    )
}

fn decode_atomic_term(term: Parsed) -> ParsingTerm {
    let term = term.into_inner().next().ok_or("Empty Term")?;

    match term.as_rule() {
        Rule::record => decode_record(term),
        Rule::string => decode_string(term)?.to_arc_ok(),
        Rule::natural => decode_natural(term)?.to_arc_ok(),
        Rule::identifier => decode_get(term),
        rule => Err(SyntaxError::Other(format!("Not an atomic term {rule:?}"))), // Rule::string =>
    }
}

fn decode_term(term: Parsed) -> ParsingTerm {
    decode_lam_sequence(term.into_inner().read(Rule::lam_sequence)?)
}

enum Assocciation {
    Left,
    Right,
}

fn decode_sequence<A: Default>(
    expr: Parsed,
    inner_rule: Rule,
    inner: impl Fn(Parsed) -> Parsing<A>,
    combine: impl Fn(A, A) -> A,
    association: Assocciation,
) -> Parsing<A> {
    let inners = expr.into_inner().filter(|s| s.as_rule() == inner_rule);
    let inners = inners.map(inner);

    match association {
        Assocciation::Left => combine_sequence(inners, combine),
        Assocciation::Right => combine_sequence(inners.rev(), combine),
    }
}

fn combine_sequence<A: Default>(
    iter: impl Iterator<Item = Parsing<A>>,
    combine: impl Fn(A, A) -> A,
) -> Parsing<A> {
    iter.reduce(|left, right| Ok(combine(left?, right?)))
        .unwrap_or_else(|| Ok(A::default()))
}

fn decode_natural(term: Parsed) -> Parsing<u64> {
    Ok(term.as_str().parse()?)
}

fn decode_get(term: Parsed) -> ParsingTerm {
    let name = term.as_str().to_string().into();
    Term::Get(name).to_arc_ok()
}

#[allow(unused)]
fn parse_term(input: &str) -> ParsingTerm {
    let mut top = KersParser::parse(Rule::term, input)?;
    decode_term(top.read(Rule::term)?)
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
fn check_various_simple_stuff() {
    KersParser::parse(Rule::single_quoted_string, "\'Hello\'").unwrap_print();
    KersParser::parse(Rule::string, "\'Hello\'").unwrap_print();
    KersParser::parse(Rule::assignment, "greet = 'Hello'").unwrap_print();
    KersParser::parse(Rule::identifier, "greet").unwrap_print().as_str();
}

#[test]
fn check_object() {
    let input = "{
       greet = 'Hello',  
       'target' = \"World\";
       'my \"agy\"' = 38,
       xxx = xxx
    }";

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
fn check_record_type() {}
