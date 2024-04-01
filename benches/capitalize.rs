#[macro_use]
extern crate criterion;
use criterion::Criterion;
use learning_rust::bench_module;

fn bench_caps(c: &mut Criterion) {
    c.bench_function("capitalize strings", |b| {
        b.iter(|| bench_module::capitalize_string("rust".to_string()))
    });
}

criterion_group!(benches, bench_caps);
criterion_main!(benches);