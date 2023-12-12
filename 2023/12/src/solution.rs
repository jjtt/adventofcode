use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

fn count(springs: &str, groups: &[usize], in_group: bool, was_group: bool) -> usize {
    let spring = springs.chars().next();
    match spring {
        None if groups.is_empty() => 1,
        None if !groups.is_empty() => 0,
        Some('.') if !in_group => count(&springs[1..], groups, false, false),
        Some('.') if in_group => 0,
        Some('#') if groups.is_empty() => 0,
        Some('#') if !in_group && was_group => 0,
        Some('#') => {
            let group = groups[0] - 1;
            if group > 0 {
                count(
                    &springs[1..],
                    &[&[group], &groups[1..]].concat(),
                    true,
                    true,
                )
            } else {
                count(&springs[1..], &groups[1..], false, true)
            }
        }
        Some('?') => {
            let with_spring = "#".to_string() + &springs[1..];
            let without_spring = ".".to_string() + &springs[1..];
            count(&with_spring, groups, in_group, was_group)
                + count(&without_spring, groups, in_group, was_group)
        }
        _ => panic!("invalid input"),
    }
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').expect("a valid line");
            let groups = groups
                .split(',')
                .map(|g| g.parse().expect("a number"))
                .collect::<Vec<usize>>();
            count(springs, &groups, false, false)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            let (springs, groups) = line.split_once(' ').expect("a valid line");
            let springs = std::iter::repeat(springs)
                .take(5)
                .collect::<Vec<_>>()
                .join("?");
            let groups = std::iter::repeat(groups)
                .take(5)
                .collect::<Vec<_>>()
                .join(",");

            let groups = groups
                .split(',')
                .map(|g| g.parse().expect("a number"))
                .collect::<Vec<usize>>();
            count(&springs, &groups, false, false)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("#", &[1] => 1; "one spring")]
    #[test_case(".", &[1] => 0; "no springs")]
    #[test_case(".", &[] => 1; "no springs, no groups")]
    #[test_case("#", &[] => 0; "one spring, no groups")]
    #[test_case("?", &[1] => 1; "something")]
    #[test_case("?", &[] => 1; "something, no groups")]
    #[test_case("??", &[1] => 2; "two something")]
    #[test_case("??", &[2] => 1; "two something, pair")]
    #[test_case("??", &[3] => 0; "two something, triplet")]
    #[test_case("??", &[] => 1; "two something, no groups")]
    #[test_case("???.###", &[1,1,3] => 1; "sample, row1")]
    fn counting(springs: &str, groups: &[usize]) -> usize {
        count(springs, groups, false, false)
    }

    #[test]
    fn part1_sample() {
        assert_eq!(21, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(7670, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(525152, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(0, part2("input.txt"));
    }
}
