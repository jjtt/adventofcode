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
        .map(|line| {
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
            .unwrap()
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
    use itertools::Itertools;
    use std::collections::HashSet;
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(39); "sample1")]
    #[test_case("sample2.txt" => is eq(590784); "sample2")]
    #[test_case("input.txt" => is eq(648681); "input")]
    fn part1(input: &str) -> usize {
        let steps = parse_reboot_steps(input);

        let mut on: HashSet<(i32, i32, i32)> = HashSet::new();

        for step in steps.into_iter().filter(|(_, x, y, z)| {
            *x.start() >= -50
                && *x.end() <= 50
                && *y.start() >= -50
                && *y.end() <= 50
                && *z.start() >= -50
                && *z.end() <= 50
        }) {
            let expanded = step
                .1
                .cartesian_product(step.2)
                .cartesian_product(step.3)
                .map(|((x, y), z)| (x, y, z))
                .collect::<HashSet<(i32, i32, i32)>>();

            if step.0 {
                on = on.union(&expanded).cloned().collect();
            } else {
                on = on.difference(&expanded).cloned().collect();
            }
        }

        on.len()
    }
}
