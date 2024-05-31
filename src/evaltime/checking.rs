use std::borrow::Cow;

use crate::Term;

use super::{
    interpreter::Interpteter,
    values::{TypeValue, Value},
};

pub enum TypeError<P: Interpteter<P>> {
    #[allow(unused)]
    Mismatch(Term, Value<P>, Value<P>),
}

#[derive(Debug, Default)]
pub(crate) struct TypeChecking<P> {
    #[allow(unused)]
    plugins: P,
}

impl<P: Interpteter<P>> TypeChecking<P> {
    #[allow(unused)]
    pub fn new() -> Self {
        todo!("new")
    }

    #[allow(unused)]
    pub fn check<'a>(
        &'a mut self,
        term: &Term,
        context: &TypeValue<P>,
    ) -> Result<TypeValue<P>, TypeError<P>> {
        Ok(match term {
            Term::Empty => TypeValue::Record(vec![]),
            Term::Reflect => context.clone(),
            Term::Append { left, right } => {
                let left = self.check(left, context)?;
                let right = self.check(right, &context.extend(left.clone()))?;
                left.extend(right)
            }

            t => todo!("typecheck"),
        })
    }
}
