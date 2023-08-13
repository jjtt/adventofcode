use criterion::{black_box, criterion_group, criterion_main, Criterion};
use d17::solution::part1;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part1", |b| b.iter(|| part1("input.txt")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
