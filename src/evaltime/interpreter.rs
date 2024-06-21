use either::Either::{self, Left, Right};

use crate::fp::{Compose, Prism, ToLeft, ToRight};

use super::{values::Value, Runtime};

pub trait Interpteter<V>: Clone {
    type Root;
    type Plug<P>: Plugin<V>;
    fn plug_in<P: Prism<Super = V, Sub = Self>>(
        root: Self::Root,
        runtime: &mut Runtime,
        prism: P,
    ) -> Self::Plug<P>;
}

pub trait Plugin<V>: Sized {
    type Own;
    fn roots(&mut self) -> Vec<Value>;
    fn then(&mut self, context: Value, term: Self::Own) -> Result<Value, ()>;
}

#[derive(Clone)]
pub enum NoValue {}

impl<V> Interpteter<V> for NoValue {
    type Root = ();
    type Plug<P> = EmptyPlugin;
    fn plug_in<P>(_: Self::Root, _: &mut Runtime, _prism: P) -> EmptyPlugin {
        EmptyPlugin
    }
}

pub struct EmptyPlugin;

impl<V> Plugin<V> for EmptyPlugin {
    type Own = NoValue;
    fn roots(&mut self) -> Vec<Value> {
        vec![]
    }
    fn then(&mut self, _context: Value, term: NoValue) -> Result<Value, ()> {
        match term {}
    }
}

impl<V, A: Interpteter<V>, B: Interpteter<V>> Interpteter<V> for Either<A, B> {
    type Root = (A::Root, B::Root);

    type Plug<P> =
        PairPlugin<A::Plug<Compose<P, ToLeft<A, B>>>, B::Plug<Compose<P, ToRight<A, B>>>>;

    fn plug_in<P: Prism<Super = V, Sub = Self>>(
        root: Self::Root,
        rt: &mut Runtime,
        prism: P,
    ) -> Self::Plug<P> {
        let (a, b) = root;
        let pa = A::plug_in(a, rt, prism.to_left());
        let pb = B::plug_in(b, rt, prism.to_right());
        PairPlugin(pa, pb)
    }
}

pub struct PairPlugin<P1, P2>(P1, P2);

impl<V, PL: Plugin<V>, PR: Plugin<V>> Plugin<V> for PairPlugin<PL, PR> {
    type Own = Either<PL::Own, PR::Own>;

    fn roots(&mut self) -> Vec<Value> {
        let PairPlugin(a, b) = self;
        a.roots().into_iter().chain(b.roots()).collect()
    }

    fn then(&mut self, context: Value, term: Self::Own) -> Result<Value, ()> {
        let PairPlugin(l, r) = self;
        match term {
            Left(lt) => l.then(context, lt),
            Right(rt) => r.then(context, rt),
        }
    }
}
