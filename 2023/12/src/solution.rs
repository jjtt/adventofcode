use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

fn start_count(springs: &str, groups: &[usize]) -> usize {
    let springs = springs.chars().collect::<Vec<_>>();
    count(Some(springs[0]), &springs[1..], groups, false, false)
}

fn count(
    spring: Option<char>,
    springs: &[char],
    groups: &[usize],
    in_group: bool,
    was_group: bool,
) -> usize {
    if cull(spring, springs, groups) {
        return 0;
    }
    let remaining = if springs.is_empty() {
        &[]
    } else {
        &springs[1..]
    };
    match spring {
        None if groups.is_empty() => 1,
        None if !groups.is_empty() => 0,
        Some('.') if !in_group => count(springs.first().copied(), remaining, groups, false, false),
        Some('.') if in_group => 0,
        Some('#') if groups.is_empty() => 0,
        Some('#') if !in_group && was_group => 0,
        Some('#') => {
            let group = groups[0] - 1;
            if group > 0 {
                count(
                    springs.first().copied(),
                    remaining,
                    &[&[group], &groups[1..]].concat(),
                    true,
                    true,
                )
            } else {
                count(
                    springs.first().copied(),
                    remaining,
                    &groups[1..],
                    false,
                    true,
                )
            }
        }
        Some('?') => {
            count(Some('#'), springs, groups, in_group, was_group)
                + count(Some('.'), springs, groups, in_group, was_group)
        }
        _ => panic!("invalid input"),
    }
}

fn cull(spring: Option<char>, springs: &[char], groups: &[usize]) -> bool {
    let sum: usize = groups.iter().sum();
    let hashes_and_question_marks = springs.iter().filter(|c| matches!(c, '#' | '?')).count()
        + spring.map_or(0, |c| matches!(c, '#' | '?') as usize);
    (sum) > hashes_and_question_marks
        || (sum + groups.len()) > springs.len() + spring.map_or(0, |_| 1) + 1
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
            start_count(springs, &groups)
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
            start_count(&springs, &groups)
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
        start_count(springs, groups)
    }

    #[test_case("???.###", &[2,1,3] => true; "too many in groups")]
    #[test_case("???.###", &[1,1,3] => false; "fits")]
    fn culling(springs: &str, groups: &[usize]) -> bool {
        let springs = springs.chars().collect::<Vec<_>>();
        cull(springs.first().copied(), &springs[1..], groups)
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
