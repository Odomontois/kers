use core::panic;
use std::{fmt::Display, sync::Arc};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{Term, ToTerm, Type};

mod error;
pub use error::SyntaxError;

#[derive(Parser)]
#[grammar = "kers.pest"]
pub struct KersParser;

trait PairsExt: Iterator {
    fn read(&mut self, rule: Rule) -> Result<Self::Item, SyntaxError>;
}

trait PairExt {
    fn check(&self, rule: Rule) -> Result<(), SyntaxError>;
}

impl<'a, I> PairsExt for I
where
    I: Iterator<Item = Pair<'a, Rule>>,
{
    fn read(&mut self, rule: Rule) -> Result<Self::Item, SyntaxError> {
        let pair = self
            .next()
            .ok_or_else(|| format!("expected {rule:?} got nothing"))?;
        pair.check(rule)?;
        Ok(pair)
    }
}

impl<'a> PairExt for Pair<'a, Rule> {
    fn check(&self, rule: Rule) -> Result<(), SyntaxError> {
        let got = self.as_rule();
        if got != rule {
            return Err(format!("expected {rule:?}, got {got:?}"))?;
        }
        Ok(())
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
        Rule::modified_term,
        decode_modifed_term,
        |first, next| Term::Then { first, next }.to_arc_term(),
        Assocciation::Left,
    )
}

fn decode_modifed_term(expr: Parsed) -> ParsingTerm {
    let mut subs = expr.into_inner().rev();
    let atomic = decode_atomic_term(subs.read(Rule::atomic_term)?)?;
    subs.try_fold(atomic, |term, sub: Pair<'_, Rule>| {
        sub.check(Rule::modifier)?;
        match sub.as_str() {
            "~" => Term::Box(term).to_arc_ok(),
            "@" => Term::Unlambda(term).to_arc_ok(),
            s => Err(SyntaxError::Other(format!("Unknown modifier {s}"))),
        }
    })
}

fn decode_atomic_term(term: Parsed) -> ParsingTerm {
    let term = term.into_inner().next().ok_or("Empty Term")?;

    match term.as_rule() {
        Rule::record => decode_record(term),
        Rule::string => decode_string(term)?.to_arc_ok(),
        Rule::natural => decode_natural(term)?.to_arc_ok(),
        Rule::identifier => decode_get(term),
        Rule::reflect => Term::Reflect.to_arc_ok(),
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
    let inners = expr.into_inner().map(|s| {
        s.check(inner_rule)?;
        inner(s)
    });

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
    KersParser::parse(Rule::identifier, "greet")
        .unwrap_print()
        .as_str();
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
