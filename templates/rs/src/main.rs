use crate::solution::{part1, part2};

mod solution;

fn main() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    println!("{NAME}:");
    println!("part1: {}", part1("input.txt"));
    println!("part2: {}", part2("input.txt"));
}
