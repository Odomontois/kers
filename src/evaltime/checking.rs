use crate::{plugins::Extension, Term};

use super::{values::Value, variables::VarIdx};

pub enum TypeError<P: Extension> {
    #[allow(unused)]
    Mismatch(Term, Value<P>, Value<P>),
}

#[derive(Debug, Default)]
pub(crate) struct TypeChecking<P> {
    #[allow(unused)]
    plugins: P,
}

impl<P: Extension> TypeChecking<P> {
    #[allow(unused)]
    pub fn new() -> Self {
        todo!("new")
    }

    #[allow(unused)]
    pub fn new_var(&mut self) -> VarIdx {
        todo!("new_var")
    }

    #[allow(unused)]
    fn unify<'a>(
        &mut self,
        inferred: Value<P>,
        expected: Value<P>,
    ) -> Result<Value<P>, TypeError<P>> {
        todo!("unify")
    }

    #[allow(unused)]
    pub fn check<'a>(
        &'a mut self,
        term: Term,
        context: Value<P>,
        expected: Value<P>,
    ) -> Result<Value<P>, TypeError<P>> {
        match term {
            Term::Empty => Ok(expected),
            Term::Reflect => self.unify(context, expected),

            _ => todo!("type_check"),
        }
    }
}
