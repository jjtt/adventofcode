use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    let (lt, gt, pages) = parse(input);

    let mut sum = 0;
    for p in pages {
        let mut sorted = p.clone();
        sorted.sort_by(|a, b| {
            if lt.contains(&(*a, *b)) {
                return std::cmp::Ordering::Less;
            } else if gt.contains(&(*a, *b)) {
                return std::cmp::Ordering::Greater;
            } else {
                panic!("no rule for {} and {}", a, b);
            }
        });
        if sorted == p {
            // find middle item in p
            let middle = p.len() / 2;
            sum += p[middle];
        }
    }

    sum
}

fn parse(
    input: &str,
) -> (
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
    Vec<Vec<usize>>,
) {
    let input = read_to_string(input).unwrap();
    let mut lines = input.lines();
    let mut rules = true;
    let mut lt = HashSet::new();
    let mut gt = HashSet::new();
    let mut pages = Vec::new();
    for line in lines {
        match line {
            "" => {
                rules = false;
            }
            _ if rules => {
                let (left, right) = scan_fmt!(line, "{d}|{d}", usize, usize).unwrap();
                assert!(lt.insert((left, right)));
                assert!(gt.insert((right, left)));
            }
            _ => {
                pages.push(
                    line.split(",")
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }
        }
    }
    (lt, gt, pages)
}

pub fn part2(input: &str) -> usize {
    let (lt, gt, pages) = parse(input);

    let mut sum = 0;
    for p in pages {
        let mut sorted = p.clone();
        sorted.sort_by(|a, b| {
            if lt.contains(&(*a, *b)) {
                return std::cmp::Ordering::Less;
            } else if gt.contains(&(*a, *b)) {
                return std::cmp::Ordering::Greater;
            } else {
                panic!("no rule for {} and {}", a, b);
            }
        });
        if sorted != p {
            let middle = sorted.len() / 2;
            sum += sorted[middle];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(143, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(4905, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(123, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(6204, part2("input.txt"));
    }
}
