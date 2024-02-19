use std::marker::PhantomData;

use slotmap::new_key_type;

new_key_type! {
    pub (crate) struct VarIdx;
}

pub struct Var<'a>(VarIdx, PhantomData<&'a ()>);
