use std::sync::Arc;

use pest::iterators::Pair;

use crate::{Key, PrimType, Term, ToTerm, Type};

use super::{Rule, SyntaxError};

type Parsed<'a> = Pair<'a, Rule>;
type Decoding<A> = Result<A, SyntaxError>;
type DecodingTerm = Decoding<Arc<Term>>;

pub(super) fn term(term: Parsed) -> DecodingTerm {
    lam_sequence(term.into_inner().read(Rule::lam_sequence)?)
}

pub(super) trait PairsExt: Iterator {
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

fn char(input: Parsed) -> Decoding<char> {
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

fn string(string: Parsed) -> Decoding<String> {
    let s = string.as_str();
    let error = || -> SyntaxError { format!("{s} is not a string").into() };
    let contents = string.into_inner().next().ok_or_else(error)?;
    contents.into_inner().map(char).collect()
}

fn key(key: Parsed) -> Decoding<String> {
    let mut key = key.into_inner();
    let first = key.next().ok_or("Empty key")?;
    match first.as_rule() {
        Rule::identifier => Ok(first.as_str().to_string()),
        Rule::string => string(first),
        rule => Err(format!("Not an identifier {rule:?}").into()),
    }
}

fn record(expr: Parsed) -> DecodingTerm {
    sequence(
        expr,
        Rule::assignment,
        assignment,
        |left, right| Term::Append { left, right }.to_arc_term(),
        Assocciation::Left,
    )
}

fn lam_sequence(expr: Parsed) -> DecodingTerm {
    sequence(
        expr,
        Rule::intersection,
        intersection,
        |dom, codom| Type::Function { dom, codom }.to_arc_term(),
        Assocciation::Right,
    )
}

fn intersection(expr: Parsed) -> DecodingTerm {
    sequence(
        expr,
        Rule::application,
        application,
        |left, right| Type::And { left, right }.to_arc_term(),
        Assocciation::Left,
    )
}

fn application(expr: Parsed) -> DecodingTerm {
    sequence(
        expr,
        Rule::then_chain,
        then_chain,
        |func, args| Term::apply(func, args).to_arc_term(),
        Assocciation::Left,
    )
}

fn then_chain(expr: Parsed) -> DecodingTerm {
    sequence(
        expr,
        Rule::modified_term,
        modifed_term,
        |first, next| Term::Then { first, next }.to_arc_term(),
        Assocciation::Left,
    )
}

fn record_type(expr: Parsed) -> DecodingTerm {
    sequence(
        expr,
        Rule::ascription,
        ascription,
        |left, right| Type::And { left, right }.to_arc_term(),
        Assocciation::Left,
    )
}

fn key_value_pair<R: ToTerm>(
    input: Parsed,
    fterm: impl FnOnce(Key, Arc<Term>) -> R,
) -> DecodingTerm {
    let mut input = input.into_inner();
    let name = key(input.read(Rule::key)?)?.into();
    let value = term(input.read(Rule::term)?)?;
    fterm(name, value).to_arc_ok()
}

fn assignment(input: Parsed) -> DecodingTerm {
    key_value_pair(input, |name, value| Term::Set { name, value })
}

fn ascription(expr: Parsed) -> DecodingTerm {
    key_value_pair(expr, |name, typ| Type::Field { name, typ })
}

fn modifed_term(expr: Parsed) -> DecodingTerm {
    let mut subs = expr.into_inner().rev();
    let atomic = atomic_term(subs.read(Rule::atomic_term)?)?;
    subs.try_fold(atomic, |term, sub: Pair<'_, Rule>| {
        sub.check(Rule::modifier)?;
        match sub.as_str() {
            "@" => Term::Unlambda(term).to_arc_ok(),
            s => Err(format!("Unknown modifier {s}").into()),
        }
    })
}

fn atomic_term(term: Parsed) -> DecodingTerm {
    let term = term.into_inner().next().ok_or("Empty Term")?;

    match term.as_rule() {
        Rule::record => record(term),
        Rule::string => string(term)?.to_arc_ok(),
        Rule::natural => natural(term)?.to_arc_ok(),
        Rule::identifier => get(term),
        Rule::reflect => Term::Reflect.to_arc_ok(),
        Rule::record_type => record_type(term), // Add missing function call
        Rule::universe => PrimType::Universe.to_arc_ok(),
        Rule::empty => Term::Empty.to_arc_ok(),
        rule => Err(format!("Not an atomic term {rule:?}").into()), // Rule::string =>
    }
}

enum Assocciation {
    Left,
    Right,
}

fn sequence<A: Default>(
    expr: Parsed,
    inner_rule: Rule,
    inner: impl Fn(Parsed) -> Decoding<A>,
    combine: impl Fn(A, A) -> A,
    association: Assocciation,
) -> Decoding<A> {
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
    iter: impl Iterator<Item = Decoding<A>>,
    combine: impl Fn(A, A) -> A,
) -> Decoding<A> {
    iter.reduce(|left, right| Ok(combine(left?, right?)))
        .unwrap_or_else(|| Ok(A::default()))
}

fn natural(term: Parsed) -> Decoding<u64> {
    Ok(term.as_str().parse()?)
}

fn get(term: Parsed) -> DecodingTerm {
    let name = term.as_str().to_string().into();
    Term::Get(name).to_arc_ok()
}
