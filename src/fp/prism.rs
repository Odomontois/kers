use std::marker::PhantomData;

use either::Either::{self, Left, Right};

pub trait Prism: Sized + Copy {
    type Super;
    type Sub;
    fn downcast(self, a: Self::Super) -> Option<Self::Sub>;
    fn upcast(self, b: Self::Sub) -> Self::Super;

    fn compose<P2>(self, p2: P2) -> Compose<Self, P2>
    where
        Self: Sized,
        P2: Prism<Super = Self::Sub>,
    {
        Compose(self, p2)
    }

    fn to_left<A, B>(self) -> Compose<Self, ToLeft<A, B>>
    where
        Self: Prism<Sub = Either<A, B>>,
    {
        self.compose(ToLeft(PhantomData, PhantomData))
    }

    fn to_right<A, B>(self) -> Compose<Self, ToRight<A, B>>
    where
        Self: Prism<Sub = Either<A, B>>,
    {
        self.compose(ToRight(PhantomData, PhantomData))
    }
}

pub struct ToLeft<A, B>(PhantomData<A>, PhantomData<B>);
impl<A, B> Clone for ToLeft<A, B> {
    fn clone(&self) -> Self {
        ToLeft(PhantomData, PhantomData)
    }
}
impl<A, B> Copy for ToLeft<A, B> {}
pub struct ToRight<A, B>(PhantomData<A>, PhantomData<B>);
impl<A, B> Clone for ToRight<A, B> {
    fn clone(&self) -> Self {
        ToRight(PhantomData, PhantomData)
    }
}

impl<A, B> Copy for ToRight<A, B> {}

impl<A, B> Prism for ToLeft<A, B> {
    type Super = Either<A, B>;
    type Sub = A;
    fn downcast(self, a: Either<A, B>) -> Option<A> {
        a.left()
    }

    fn upcast(self, a: A) -> Either<A, B> {
        Left(a)
    }
}

impl<A, B> Prism for ToRight<A, B> {
    type Super = Either<A, B>;
    type Sub = B;
    fn downcast(self, a: Either<A, B>) -> Option<B> {
        a.right()
    }

    fn upcast(self, b: B) -> Either<A, B> {
        Right(b)
    }
}

#[derive(Clone, Copy)]
pub struct Compose<P1, P2>(P1, P2);

impl<P1, P2> Prism for Compose<P1, P2>
where
    P1: Prism,
    P2: Prism<Super = P1::Sub>,
{
    type Sub = P2::Sub;
    type Super = P1::Super;

    fn downcast(self, a: Self::Super) -> Option<Self::Sub> {
        self.0.downcast(a).and_then(|b| self.1.downcast(b))
    }

    fn upcast(self, c: Self::Sub) -> Self::Super {
        self.0.upcast(self.1.upcast(c))
    }
}
