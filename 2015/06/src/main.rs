#[macro_use]
extern crate scan_fmt;
use itertools::Itertools;
use std::collections::HashSet;

use std::fs::read_to_string;
use std::ops::Range;

fn main() {
    println!("Hello, world!");
}

fn parse_reboot_steps(input: &str) -> Vec<(String, Range<i32>, Range<i32>, Range<i32>)> {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(|line| {
            scan_fmt!(
                line,
                "{[^0-9]} {d},{d} through {d},{d}",
                String,
                i32,
                i32,
                i32,
                i32
            )
            .unwrap()
        })
        .map(|(on_off, xmin, ymin, xmax, ymax)| (on_off, xmin..xmax + 1, ymin..ymax + 1, 0..1))
        .collect()
}

fn find_split_coordinates(
    steps: &Vec<(String, Range<i32>, Range<i32>, Range<i32>)>,
) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    (
        steps
            .iter()
            .flat_map(|(_, x, _, _)| vec![x.start, x.end])
            .sorted()
            .unique()
            .collect(),
        steps
            .iter()
            .flat_map(|(_, _, y, _)| vec![y.start, y.end])
            .sorted()
            .unique()
            .collect(),
        steps
            .iter()
            .flat_map(|(_, _, _, z)| vec![z.start, z.end])
            .sorted()
            .unique()
            .collect(),
    )
}

fn split(
    step: &(String, Range<i32>, Range<i32>, Range<i32>),
    x: Vec<i32>,
    y: Vec<i32>,
    z: Vec<i32>,
) -> Vec<(String, Range<i32>, Range<i32>, Range<i32>)> {
    let split_x = split_range(&step.1, x);
    let split_y = split_range(&step.2, y);
    let split_z = split_range(&step.3, z);

    split_x
        .into_iter()
        .cartesian_product(split_y)
        .cartesian_product(split_z)
        .map(|((x, y), z)| (step.0.clone(), x, y, z))
        .collect()
}

fn split_range(range: &Range<i32>, splits: Vec<i32>) -> Vec<Range<i32>> {
    let mut out = vec![];

    let mut last = range.start;
    for s in splits {
        if s > range.start && s < range.end {
            out.push(last..s);
            last = s;
        }
    }
    out.push(last..range.end);

    out
}

fn count(x: &Range<i32>, y: &Range<i32>, z: &Range<i32>) -> usize {
    (x.end - x.start) as usize * (y.end - y.start) as usize * (z.end - z.start) as usize
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test]
    fn finding_split_coordinates() {
        let steps = parse_reboot_steps("sample1.txt");

        let (x, y, z) = find_split_coordinates(&steps);

        assert_eq!(vec![0, 499, 501, 1000], x);
        assert_eq!(vec![0, 1, 499, 501, 1000], y);
        assert_eq!(vec![0, 1], z);
    }

    #[test]
    fn splitting() {
        let steps = parse_reboot_steps("sample1.txt");

        let (x, y, z) = find_split_coordinates(&steps);

        let s = split(&steps.first().unwrap(), x, y, z);
        assert_eq!(12, s.len());
    }

    #[test_case("sample1.txt" => is eq(1000000-1000-4); "sample1")]
    #[test_case("input.txt" => is eq(400410); "input")]
    fn part1(input: &str) -> usize {
        let steps = parse_reboot_steps(input);

        let (x, y, z) = find_split_coordinates(&steps);

        let mut on: HashSet<(Range<i32>, Range<i32>, Range<i32>)> = HashSet::new();

        for step in steps
            .iter()
            .flat_map(|step| split(step, x.clone(), y.clone(), z.clone()))
        {
            let expanded = (step.1, step.2, step.3);

            match step.0.as_str() {
                "turn on " => on.insert(expanded),
                "turn off " => on.remove(&expanded),
                "toggle " => {
                    if on.contains(&expanded) {
                        on.remove(&expanded)
                    } else {
                        on.insert(expanded)
                    }
                }
                _ => panic!("Unsupported: '{}'", step.0),
            };
        }

        on.iter().map(|(x, y, z)| count(x, y, z)).sum()
    }
}
