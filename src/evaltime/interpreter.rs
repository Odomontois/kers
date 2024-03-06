use std::marker::PhantomData;

use either::Either::{self, Left, Right};

use crate::fp::{Compose, Prism, ToLeft, ToRight};

use super::{ruintime::Runtime, values::Value};

pub trait Interpteter: Clone{
    type Val;
    type Plug<'a, V: 'a, P>: Plugin<Val = V, Own = Self::Val> + 'a;
    fn plug_in<'a, V, P: Prism<Super = V, Sub = Self::Val>>(
        self,
        runtime: &mut Runtime,
        prism: P,
    ) -> Self::Plug<'a, V, P>;
}

pub trait Plugin: Sized {
    type Val;
    type Own;
    fn roots(&mut self) -> Vec<Value<Self::Val>>;
    fn then(&mut self, context: Value<Self::Val>, term: Self::Own) -> Result<Value<Self::Val>, ()>;
}

pub enum NoValue {}

impl Interpteter for () {
    type Plug<'a, V: 'a, P> = EmptyPlugin<V>;
    type Val = NoValue;
    fn plug_in<'a, V: 'a, P>(self, _: &mut Runtime, _prism: P) -> EmptyPlugin<V> {
        EmptyPlugin(PhantomData)
    }
}

pub struct EmptyPlugin<V>(PhantomData<V>);

impl<V> Plugin for EmptyPlugin<V> {
    type Own = NoValue;
    type Val = V;
    fn roots(&mut self) -> Vec<Value<V>> {
        vec![]
    }
    fn then(&mut self, _context: Value<V>, term: NoValue) -> Result<Value<V>, ()> {
        match term {}
    }
}

impl<A: Interpteter, B: Interpteter> Interpteter for (A, B) {
    type Val = Either<A::Val, B::Val>;

    type Plug<'a, V: 'a, P> = PairPlugin<
        A::Plug<'a, V, Compose<P, ToLeft<A::Val, B::Val>>>,
        B::Plug<'a, V, Compose<P, ToRight<A::Val, B::Val>>>,
    >;

    fn plug_in<'a, V, P: Prism<Super = V, Sub = Self::Val>>(
        self,
        rt: &mut Runtime,
        prism: P,
    ) -> Self::Plug<'a, V, P> {
        let (a, b) = self;
        let pa = a.plug_in(rt, prism.to_left());
        let b = b.plug_in(rt, prism.to_right());
        PairPlugin(pa, b)
    }
}

pub struct PairPlugin<P1, P2>(P1, P2);

impl<PL: Plugin<Val = V>, PR: Plugin<Val = V>, V> Plugin for PairPlugin<PL, PR> {
    type Val = V;
    type Own = Either<PL::Own, PR::Own>;

    fn roots(&mut self) -> Vec<Value<V>> {
        let PairPlugin(a, b) = self;
        a.roots().into_iter().chain(b.roots()).collect()
    }

    fn then(&mut self, context: Value<Self::Val>, term: Self::Own) -> Result<Value<Self::Val>, ()> {
        let PairPlugin(l, r) = self;
        match term {
            Left(lt) => l.then(context, lt),
            Right(rt) => r.then(context, rt),
        }
    }
}

