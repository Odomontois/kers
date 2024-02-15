use std::sync::Arc;

use crate::{PrimType, Term, Type};

use crate::language::term::Primitive;

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

impl ToTerm for PrimType {
    fn to_term(self) -> Term {
        Type::Prim(self).to_term()
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

pub struct AsTyp<A>(pub A);

impl ToTerm for AsTyp<Term> {
    fn to_term(self) -> Term {
        self.0
    }
}

impl<S: ToString, A> ToTerm for AsTyp<(S, A)>
where
    AsTyp<A>: ToTerm,
{
    fn to_term(self) -> Term {
        let AsTyp((name, typ)) = self;
        Type::Field {
            name: name.to_string().into(),
            typ: AsTyp(typ).to_arc_term(),
        }
        .to_term()
    }
}

impl<X: Clone> ToTerm for AsTyp<&[X]>
where
    AsTyp<X>: ToTerm,
{
    fn to_term(self) -> Term {
        let and = |left, right: Arc<Term>| Type::And { left, right }.to_term();
        reduce_term(self.0.iter().cloned().map(AsTyp), PrimType::Universe, and)
    }
}

impl<const N: usize, X> ToTerm for AsTyp<[X; N]>
where
    AsTyp<X>: ToTerm,
{
    fn to_term(self) -> Term {
        let and = |left, right: Arc<Term>| Type::And { left, right }.to_term();
        reduce_term(self.0.into_iter().map(AsTyp), PrimType::Universe, and)
    }
}
