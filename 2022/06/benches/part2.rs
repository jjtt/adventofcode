use criterion::{black_box, criterion_group, criterion_main, Criterion};
use d06::solution::find_marker;
use std::fs::read_to_string;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = read_to_string("input.txt").unwrap();
    let signal = input.lines().next().unwrap();

    c.bench_function("part2", |b| b.iter(|| find_marker(black_box(signal), 14)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
