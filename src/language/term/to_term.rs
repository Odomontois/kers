use std::sync::Arc;

use crate::{Primitive, Term, Type};

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

fn reduce_term<I: ToTerm, R: ToTerm, D: ToTerm>(
    is: impl IntoIterator<Item = I>,
    d: D,
    f: impl Fn(Arc<Term>, Arc<Term>) -> R,
) -> Term {
    is.into_iter()
        .map(ToTerm::to_term)
        .reduce(|x, y| f(x.to_arc_term(), y.to_arc_term()).to_term())
        .unwrap_or(d.to_term())
}

impl<const N: usize, X: ToTerm> ToTerm for [X; N] {
    fn to_term(self) -> Term {
        let append = |left, right| Term::Append { left, right };
        reduce_term(self, Term::Empty, append)
    }
}

impl<X: ToTerm + Clone> ToTerm for &[X] {
    fn to_term(self) -> Term {
        let append = |left, right| Term::Append { left, right };
        reduce_term(self.iter().cloned(), Term::Empty, append)
    }
}

pub struct Typ<A>(pub A);

impl ToTerm for Typ<Term> {
    fn to_term(self) -> Term {
        self.0
    }
}

impl<S: ToString, A> ToTerm for Typ<(S, A)>
where
    Typ<A>: ToTerm,
{
    fn to_term(self) -> Term {
        Type::Field {
            name: self.0 .0.to_string(),
            typ: Typ(self.0 .1).to_arc_term(),
        }
        .to_term()
    }
}

impl<X: Clone> ToTerm for Typ<&[X]>
where
    Typ<X>: ToTerm,
{
    fn to_term(self) -> Term {
        let and = |left, right: Arc<Term>| Type::And { left, right }.to_term();
        reduce_term(self.0.iter().cloned().map(Typ), Type::Universe, and)
    }
}

impl<const N: usize, X> ToTerm for Typ<[X; N]>
where
    Typ<X>: ToTerm,
{
    fn to_term(self) -> Term {
        let and = |left, right: Arc<Term>| Type::And { left, right }.to_term();
        reduce_term(self.0.into_iter().map(Typ), Type::Universe, and)
    }
}
