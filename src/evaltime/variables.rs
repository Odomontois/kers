use std::marker::PhantomData;

use slotmap::new_key_type;

new_key_type! {
    pub  struct VarIdx;
}

#[allow(unused)]
pub struct Var<'a>(VarIdx, PhantomData<&'a ()>);
