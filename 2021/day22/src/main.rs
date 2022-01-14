#[macro_use]
extern crate scan_fmt;
use itertools::Itertools;
use std::collections::HashSet;

use std::fs::read_to_string;
use std::ops::Range;

fn main() {
    println!("Hello, world!");
}

fn parse_reboot_steps(input: &str) -> Vec<(bool, Range<i32>, Range<i32>, Range<i32>)> {
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
                xmin..xmax + 1,
                ymin..ymax + 1,
                zmin..zmax + 1,
            )
        })
        .collect()
}

fn find_split_coordinates(
    steps: &Vec<(bool, Range<i32>, Range<i32>, Range<i32>)>,
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

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test]
    fn finding_split_coordinates() {
        let steps = parse_reboot_steps("sample1.txt");

        let (x, y, z) = find_split_coordinates(&steps);

        assert_eq!(vec![9, 10, 11, 12, 13, 14], x);
        assert_eq!(vec![9, 10, 11, 12, 13, 14], y);
        assert_eq!(vec![9, 10, 11, 12, 13, 14], z);
    }

    #[test]
    fn splitting() {
        let steps = parse_reboot_steps("sample1.txt");

        let (x, y, z) = find_split_coordinates(&steps);

        let s = split(&steps.first().unwrap(), x, y, z);
        assert_eq!(27, s.len());

        let s = split(&steps.first().unwrap(), vec![11], vec![], vec![]);
        assert_eq!(2, s.len());
        assert_eq!(
            vec![
                (true, 10..11, 10..13, 10..13),
                (true, 11..13, 10..13, 10..13)
            ],
            s
        );

        let s = split(&steps.first().unwrap(), vec![11], vec![], vec![11]);
        assert_eq!(4, s.len());

        let s = split(&steps.first().unwrap(), vec![11], vec![11], vec![11]);
        assert_eq!(8, s.len());
    }

    fn split(
        step: &(bool, Range<i32>, Range<i32>, Range<i32>),
        x: Vec<i32>,
        y: Vec<i32>,
        z: Vec<i32>,
    ) -> Vec<(bool, Range<i32>, Range<i32>, Range<i32>)> {
        let split_x = split_range(&step.1, x);
        let split_y = split_range(&step.2, y);
        let split_z = split_range(&step.3, z);

        split_x
            .into_iter()
            .cartesian_product(split_y)
            .cartesian_product(split_z)
            .map(|((x, y), z)| (step.0, x, y, z))
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

    #[test_case("sample1.txt" => is eq(39); "sample1")]
    #[test_case("sample2.txt" => is eq(590784); "sample2")]
    #[test_case("sample3.txt" => is eq(474140); "sample3")]
    #[test_case("input.txt" => is eq(648681); "input")]
    fn part1(input: &str) -> usize {
        let steps = parse_reboot_steps(input);

        let mut on: HashSet<(i32, i32, i32)> = HashSet::new();

        for step in steps.into_iter().filter(|(_, x, y, z)| {
            x.start >= -50
                && x.end <= 51
                && y.start >= -50
                && y.end <= 51
                && z.start >= -50
                && z.end <= 51
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

    #[test_case("sample1.txt" => is eq(39); "sample1")]
    #[test_case("sample3.txt" => is eq(2758514936282235); "sample3")]
    #[test_case("input.txt" => is eq(1302784472088899); "input")]
    fn part2(input: &str) -> usize {
        let steps = parse_reboot_steps(input);

        let (x, y, z) = find_split_coordinates(&steps);

        let mut on: HashSet<(Range<i32>, Range<i32>, Range<i32>)> = HashSet::new();

        for step in steps
            .iter()
            .flat_map(|step| split(step, x.clone(), y.clone(), z.clone()))
        {
            let expanded = (step.1, step.2, step.3);

            if step.0 {
                on.insert(expanded);
            } else {
                on.remove(&expanded);
            }
        }

        on.iter().map(|(x, y, z)| count(x, y, z)).sum()
    }

    fn count(x: &Range<i32>, y: &Range<i32>, z: &Range<i32>) -> usize {
        (x.end - x.start) as usize * (y.end - y.start) as usize * (z.end - z.start) as usize
    }
}
