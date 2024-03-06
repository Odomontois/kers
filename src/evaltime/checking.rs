use crate::Term;

use super::{interpreter::Interpteter, values::Value, variables::VarIdx};

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
        term: &Term,
        context: &Value<P>,
    ) -> Result<Value<P>, TypeError<P>> {
        match term {
            Term::Empty => Ok(Value::Record { fields: vec![] }),
            Term::Reflect => {
                let cloned: Value<P> = context.clone();
                Ok(context.clone())
            }
            Term::Append { left, right } => {
                let left = self.check(left, context)?;
                let right = self.check(right, context)?;
                todo!()
            }

            _ => todo!("type_check"),
        }
    }
}
