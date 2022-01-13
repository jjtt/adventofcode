#[macro_use]
extern crate scan_fmt;

use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn parse_starting_positions(input: &str) -> ((i32, i32), (i32, i32)) {
    let s = read_to_string(input).unwrap();
    let mut lines = s.lines();

    (
        scan_fmt!(
            lines.next().unwrap(),
            "Player {d} starting position: {d}",
            i32,
            i32
        )
        .unwrap(),
        scan_fmt!(
            lines.next().unwrap(),
            "Player {d} starting position: {d}",
            i32,
            i32
        )
        .unwrap(),
    )
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(739785); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> usize {
        let ((_, start1), (_, start2)) = parse_starting_positions(input);

        0
    }
}
