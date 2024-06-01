use std::{any::Any, fmt::Debug};
// stolen from https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=96009663098cd89e53a890735697723c
// by @kolsky

trait Het: Any + Debug {
    fn any(&self) -> &dyn Any;
    fn dyn_eq(&self, other: &dyn Het) -> bool;
}

impl<T: Any + Eq + Debug> Het for T {
    fn any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn Het) -> bool {
        other.any().downcast_ref::<T>().is_some_and(|o| self == o)
    }
}

impl PartialEq for dyn Het {
    fn eq(&self, other: &dyn Het) -> bool {
        self.dyn_eq(other)
    }
}

impl Eq for dyn Het {}

#[test]
fn check() {
    let x: &dyn Het = &0;
    let y: &dyn Het = &0;
    dbg!(x == y);
}
