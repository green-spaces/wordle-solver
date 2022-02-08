use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wordle_solver::rank::outcome;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("outcome", |b| {
        b.iter(|| outcome(black_box("hello"), black_box("world")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
