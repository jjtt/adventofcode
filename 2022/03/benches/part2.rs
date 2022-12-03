use criterion::{black_box, criterion_group, criterion_main, Criterion};
use d03::solution::part2;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part2", |b| b.iter(|| part2(black_box("input.txt"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
