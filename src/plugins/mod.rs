use slotmap::new_key_type;

use crate::evaltime::{typing::ClosedEvalType, values::EvalValue};

new_key_type! {
    pub struct PluginIdx;
}

pub trait Plugin {
    type Type;
    type Value;
    fn then(&mut self, context: EvalValue, term: Self::Value) -> Result<Self::Value, ()>;
}

pub(crate) type EvalPlugin = Box<dyn Plugin<Type = ClosedEvalType, Value = EvalValue>>;
