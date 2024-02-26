use crate::evaltime::values::Value;

pub trait Runtime {}
pub trait Extension: Sized {
    type Value;
    type Plug: Plugin<Self>;
    fn plug_in(runtime: &mut dyn Runtime) -> Self::Plug;
}

pub trait Plugin<P: Extension>: Sized {
    fn root() -> Value<P>;
    fn then(&mut self, context: Value<P>, term: P::Value) -> Result<Value<P>, ()>;
}

pub enum NoValue {}

impl Extension for () {
    type Plug = ();
    type Value = NoValue;
    fn plug_in(_runtime: &mut dyn Runtime) -> () {
        todo!()
    }
}

impl Plugin<()> for () {
    fn root() -> Value<()> {
        Value::Record { fields: vec![] }
    }
    fn then(&mut self, _context: Value<()>, term: NoValue) -> Result<Value<()>, ()> {
        match term {}
    }
}
