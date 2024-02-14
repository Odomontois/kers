use std::marker::PhantomData;

use slotmap::new_key_type;

new_key_type! {
    struct VarKey;
}

pub struct Var<'a>(VarKey, PhantomData<&'a ()>);
