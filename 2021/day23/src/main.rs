#[macro_use]
extern crate scan_fmt;
use itertools::Itertools;
use std::collections::HashSet;

use std::fs::read_to_string;
use std::ops::Range;

fn parse_situation(input: &str) -> () {
    let s = read_to_string(input).unwrap();
    let mut lines = s.lines();

    assert_eq!("#############", lines.next().unwrap());
    assert_eq!("#...........#", lines.next().unwrap());

    let top = scan_fmt!(
        lines.next().unwrap(),
        "###{}#{}#{}#{}###",
        char,
        char,
        char,
        char
    )
    .unwrap();
    let bottom = scan_fmt!(
        lines.next().unwrap(),
        "  #{}#{}#{}#{}#",
        char,
        char,
        char,
        char
    )
    .unwrap();

    assert_eq!("  #########", lines.next().unwrap());
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(12521); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> usize {
        let steps = parse_situation(input);

        0
    }
}
