use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::read_to_string;

#[cfg(feature = "bench_nightly")]
extern crate test;

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
    read_to_string(input)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(index, item)| (index / 3, to_bitmask(item)))
        .group_by(|(group, _)| *group)
        .into_iter()
        .map(|(_, items)| items.fold(usize::MAX, |a, (_, b)| a & b))
        .map(to_priority)
        .sum()
}

fn to_bitmask(core: &str) -> usize {
    core.chars()
        .map(priority)
        .map(|b| 0b1_usize << (b - 1))
        .reduce(|a, b| a | b)
        .unwrap_or(0)
}

fn to_priority(badge: usize) -> usize {
    1 + badge.trailing_zeros() as usize
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
    #[cfg(feature = "bench_nightly")]
    use test::Bencher;

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

    #[test]
    fn part2_sample() {
        assert_eq!(70, part2("sample.txt"));
    }

    #[cfg(feature = "bench_nightly")]
    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                part2("input.txt");
            }
        })
    }

    #[test]
    fn first_rule_of_optimisation() {
        const FIRST_5_BITS: usize = 0b11111;
        let r = "abcde";

        let b = to_bitmask(r);

        assert_eq!(FIRST_5_BITS, b);
    }

    #[test]
    fn first_rule_of_optimisation_2() {
        assert_eq!(priority('a'), to_priority(0b1));
        assert_eq!(priority('b'), to_priority(0b10));
        assert_eq!(priority('c'), to_priority(0b100));
        assert_eq!(priority('d'), to_priority(0b1000));
        assert_eq!(priority('A'), to_priority(0b100000000000000000000000000));
    }
}
