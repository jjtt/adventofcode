use anyhow::bail;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::cmp::Ordering;
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    read_to_string(input)
        .unwrap()
        .lines()
        .map(split)
        .map(find_first_common)
        .map(priority)
        .sum()
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

fn priority(c: char) -> usize {
    const SMALL_A: usize = 'a' as usize;
    const SMALL_Z: usize = 'z' as usize;
    const BIG_A: usize = 'A' as usize;
    const BIG_Z: usize = 'Z' as usize;
    match c as usize {
        v if (SMALL_A..=SMALL_Z).contains(&v) => v - SMALL_A + 1,
        v if (BIG_A..=BIG_Z).contains(&v) => v - BIG_A + 27,
        _ => panic!("Unsupported input char: {c}"),
    }
}

fn split(rucksack: &str) -> (&str, &str) {
    assert_eq!(
        0,
        rucksack.len() % 2,
        "String length must be divisible by 2"
    );
    (
        &rucksack[0..rucksack.len() / 2],
        &rucksack[rucksack.len() / 2..rucksack.len()],
    )
}

fn find_first_common(pair: (&str, &str)) -> char {
    let mut left_sorted = pair.0.chars().sorted();
    let mut right_sorted = pair.1.chars().sorted();

    let mut l = left_sorted.next();
    let mut r = right_sorted.next();
    while let (Some(left_char), Some(right_char)) = (l, r) {
        match left_char.cmp(&right_char) {
            Ordering::Less => l = left_sorted.next(),
            Ordering::Equal => return left_char,
            Ordering::Greater => r = right_sorted.next(),
        }
    }
    panic!("Could not find common item: {pair:?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priorities() {
        assert_eq!(1, priority('a'));
        assert_eq!(26, priority('z'));
        assert_eq!(27, priority('A'));
        assert_eq!(52, priority('Z'));
    }

    #[test]
    fn splitting() {
        assert_eq!("vJrwpWtwJgWr", split("vJrwpWtwJgWrhcsFMMfFFhFp").0);
        assert_eq!("hcsFMMfFFhFp", split("vJrwpWtwJgWrhcsFMMfFFhFp").1);
    }

    #[test]
    fn finding_first_common() {
        assert_eq!('p', find_first_common(("vJrwpWtwJgWr", "hcsFMMfFFhFp")));
        assert_eq!(
            'L',
            find_first_common(("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"))
        );
        assert_eq!('P', find_first_common(("PmmdzqPrV", "vPwwTWBwg")));
    }

    #[test]
    #[should_panic]
    fn finding_nothing_in_common() {
        find_first_common(("ABCDE", "abcdefg"));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(157, part1("sample.txt"));
    }
}
