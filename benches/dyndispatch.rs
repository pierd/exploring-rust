use std::{vec, marker::PhantomData};

use criterion::{criterion_group, criterion_main, Criterion};

trait Dispached<T> {
    fn dispatch(&self, data: &T);
}

struct DummyDispatched;
impl Dispached<Vec<u8>> for DummyDispatched {
    fn dispatch(&self, data: &Vec<u8>) {
        std::hint::black_box(data);
    }
}

type DynDispached<T> = Box<dyn Dispached<T>>;

struct MultiDispatcher<T> {
    dispatchers: Vec<DynDispached<T>>,
}
impl<T> Dispached<T> for MultiDispatcher<T> {
    fn dispatch(&self, data: &T) {
        for dispatcher in &self.dispatchers {
            dispatcher.dispatch(data);
        }
    }
}

struct ChainedDispatcher<T, D1: Dispached<T>, D2: Dispached<T>>(D1, D2, PhantomData<T>);
impl<T, D1: Dispached<T>, D2: Dispached<T>> Dispached<T> for ChainedDispatcher<T, D1, D2> {
    fn dispatch(&self, data: &T) {
        self.0.dispatch(data);
        self.1.dispatch(data);
    }
}
impl<T, D1: Dispached<T>, D2: Dispached<T>> ChainedDispatcher<T, D1, D2> {
    fn new(d1: D1, d2: D2) -> Self {
        Self(d1, d2, PhantomData)
    }

    fn add<D3: Dispached<T>>(self, d3: D3) -> ChainedDispatcher<T, Self, D3> {
        ChainedDispatcher::new(self, d3)
    }
}

pub fn dyn_dispatch(c: &mut Criterion) {
    let mut dispatchers = Vec::<DynDispached<_>>::new();
    for _ in 0..10 {
        dispatchers.push(Box::new(DummyDispatched));
    }
    let data = vec![0u8; 1000];
    let dispatcher = MultiDispatcher { dispatchers };
    c.bench_function("dyn dispatch", |b| b.iter(|| dispatcher.dispatch(&data)));
}

pub fn chained_dispatch(c: &mut Criterion) {
    let data = vec![0u8; 1000];
    let dispatcher = ChainedDispatcher::new(DummyDispatched, DummyDispatched);
    let dispatcher = dispatcher.add(DummyDispatched);
    let dispatcher = dispatcher.add(DummyDispatched);
    let dispatcher = dispatcher.add(DummyDispatched);
    let dispatcher = dispatcher.add(DummyDispatched);
    let dispatcher = dispatcher.add(DummyDispatched);
    let dispatcher = dispatcher.add(DummyDispatched);
    let dispatcher = dispatcher.add(DummyDispatched);
    let dispatcher = dispatcher.add(DummyDispatched);
    c.bench_function("chained dispatch", |b| b.iter(|| dispatcher.dispatch(&data)));
}

criterion_group!(benches, dyn_dispatch, chained_dispatch);
criterion_main!(benches);
