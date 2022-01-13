#[macro_use]
extern crate scan_fmt;

use std::fs::read_to_string;
use std::ops::RangeInclusive;

fn main() {
    println!("Hello, world!");
}

fn parse_reboot_steps(
    input: &str,
) -> Vec<(
    bool,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
    RangeInclusive<i32>,
)> {
    read_to_string(input)
        .unwrap()
        .lines()
        .filter_map(|line| {
            scan_fmt!(
                line,
                "{} x={d}..{d},y={d}..{d},z={d}..{d}",
                String,
                i32,
                i32,
                i32,
                i32,
                i32,
                i32
            )
            .ok()
        })
        .map(|(on_off, xmin, xmax, ymin, ymax, zmin, zmax)| {
            (
                if "on" == on_off { true } else { false },
                RangeInclusive::new(xmin, xmax),
                RangeInclusive::new(ymin, ymax),
                RangeInclusive::new(zmin, zmax),
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(39); "sample1")]
    #[test_case("sample2.txt" => is eq(590784); "sample2")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> i32 {
        let foo = parse_reboot_steps(input);

        dbg!(foo);

        0
    }
}
