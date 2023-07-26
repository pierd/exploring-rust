use criterion::{criterion_group, criterion_main, Criterion};

pub fn sync_sleep(c: &mut Criterion) {
    c.bench_function("sleep 1ns sync", |b| {
        b.iter(|| std::thread::sleep(std::time::Duration::from_nanos(1)))
    });
}

pub fn async_sleep(c: &mut Criterion) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap();
    c.bench_function("sleep 1ns async", |b| {
        b.to_async(&runtime)
            .iter(|| tokio::time::sleep(std::time::Duration::from_nanos(1)))
    });
}

criterion_group!(benches, sync_sleep, async_sleep);
criterion_main!(benches);
