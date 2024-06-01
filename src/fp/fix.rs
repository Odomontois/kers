use super::Wrapper;

#[allow(unused)]
pub(crate) trait Peel<'a> {
    type Layer<A>: Sized + 'a;
}

#[allow(unused)]
pub(crate) struct Fix<'a, W: Wrapper<'a>, S: Peel<'a>>(W::Wrap<S::Layer<Fix<'a, W, S>>>);
