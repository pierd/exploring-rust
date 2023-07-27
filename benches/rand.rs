use criterion::{criterion_group, criterion_main, Criterion};

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("random u64", |b| b.iter(|| rand::random::<u64>()));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
