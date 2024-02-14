use crate::Term;

use super::{typing::CtxEvalType, variables::Var};

pub enum TypeError<'a> {
    Mismatch(Term, CtxEvalType<'a>, CtxEvalType<'a>),
}

struct TypeChecking;

impl TypeChecking {
    pub fn new() -> Self {
        TypeChecking
    }

    pub fn new_var(&mut self) -> Var {
        todo!("new_var")
    }

    fn unify<'a>(
        &mut self,
        inferred: CtxEvalType<'a>,
        expected: CtxEvalType<'a>,
    ) -> Result<CtxEvalType<'a>, TypeError> {
        todo!("unify")
    }

    #[allow(unused)]
    pub fn check<'a>(
        &'a mut self,
        term: Term,
        context: CtxEvalType<'a>,
        expected: CtxEvalType<'a>,
    ) -> Result<CtxEvalType<'a>, TypeError> {
        match term {
            Term::Empty => Ok(expected),
            Term::Reflect => self.unify(context, expected),
            
            _ => todo!("type_check"),
        }
    }
}
