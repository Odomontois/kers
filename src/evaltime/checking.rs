use crate::{plugins::Interpteter, Term};

use super::{values::Value, variables::VarIdx};

pub enum TypeError<P: Interpteter> {
    #[allow(unused)]
    Mismatch(Term, Value<P>, Value<P>),
}

#[derive(Debug, Default)]
pub(crate) struct TypeChecking<P> {
    #[allow(unused)]
    plugins: P,
}

impl<P: Interpteter> TypeChecking<P> {
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
            Term::Empty => Value::Record { fields: vec![] }.adapt(expected),
            Term::Reflect => self.unify(context, expected),

            _ => todo!("type_check"),
        }
    }
}
