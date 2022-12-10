use crate::solution::{part1, part2};
use std::time::Instant;

mod solution;

#[derive(Debug)]
enum Part {
    One,
    Two,
}

fn part2_wrapper(input: &str) -> i64 {
    println!("{}", part2(input));
    -1
}

fn solve_measure_and_print(part: Part, input: &str) {
    let solver = match part {
        Part::One => part1,
        Part::Two => part2_wrapper,
    };
    let start = Instant::now();
    let result = solver(input);
    let micros = start.elapsed().as_micros();
    println!("Part {part:?}: {result} ({micros}µs)");
}

fn main() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    println!("{NAME}:");
    solve_measure_and_print(Part::One, "input.txt");
    solve_measure_and_print(Part::Two, "input.txt");
}
