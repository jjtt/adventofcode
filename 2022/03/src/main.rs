use d03::solution::{part1, part2};
use std::time::Instant;

#[derive(Debug)]
enum Part {
    One,
    Two,
}

fn solve_measure_and_print(part: Part, input: &str) {
    let solver = match part {
        Part::One => part1,
        Part::Two => part2,
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
