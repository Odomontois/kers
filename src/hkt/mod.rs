use std::borrow::BorrowMut;
use std::ops::Deref;
use std::{rc::Rc, sync::Arc};

pub(crate) trait Wrapper<'a>: Sized + 'a {
    type In: 'a;
    type Wrap<A: 'a>: 'a + Deref<Target = A> + Sized + 'a;
    fn wrap<A: 'a>(a: A) -> Self::Wrap<A>;
}

macro_rules! impl_wrapper {
    ($($t:ident)*) => {
        $(
            impl<'a, T: 'a> Wrapper<'a> for $t<T> {
                type In = T;
                type Wrap<A: 'a> = $t<A>;
                fn wrap<A: 'a>(a: A) -> Self::Wrap<A> {
                    $t::new(a)
                }
            }
        )*
    };
}

impl_wrapper! {Box Rc Arc}

pub(crate) trait GetMut: Deref {
    fn get_mut(&mut self) -> Option<&mut Self::Target>;
}

impl<A> GetMut for Box<A> {
    fn get_mut(&mut self) -> Option<&mut Self::Target> {
        Some(self.borrow_mut())
    }
}

impl<A> GetMut for Rc<A> {
    fn get_mut(&mut self) -> Option<&mut Self::Target> {
        Rc::get_mut(self)
    }
}

impl<A> GetMut for Arc<A> {
    fn get_mut(&mut self) -> Option<&mut Self::Target> {
        Arc::get_mut(self)
    }
}
