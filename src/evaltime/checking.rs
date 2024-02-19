use slotmap::SlotMap;

use crate::Term;

use super::{typing::CtxEvalType, variables::VarIdx};

pub enum TypeError<'a> {
    #[allow(unused)]
    Mismatch(Term, CtxEvalType<'a>, CtxEvalType<'a>),
}

#[derive(Debug, Default)]
pub(crate) struct TypeChecking {
    #[allow(unused)]
    slot_map: SlotMap<VarIdx, ()>,
}

impl TypeChecking {
    #[allow(unused)]
    pub fn new() -> Self {
        TypeChecking::default()
    }

    #[allow(unused)]
    pub fn new_var(&mut self) -> VarIdx {
        todo!("new_var")
    }

    #[allow(unused)]
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
