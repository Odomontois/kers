use std::{collections::HashMap, sync::Arc};

use crate::{Key, PrimType};

use super::variables::Var;

pub enum Solved {}

pub enum EvalType<V> {
    Var(V),
    Prim(PrimType),
    Record {
        fields: HashMap<Key, EvalType<V>>,
    },
    Function {
        dom: Arc<EvalType<V>>,
        codom: Arc<EvalType<V>>,
    },
}

pub type ClosedEvalType = EvalType<Solved>;
pub type CtxEvalType<'a> = EvalType<Var<'a>>;
