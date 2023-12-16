use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    input.trim().split(',').map(hash).sum()
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

fn hash(input: &str) -> usize {
    let mut current_value = 0;
    input.chars().for_each(|c| {
        current_value += c as usize;
        current_value *= 17;
        current_value %= 256;
    });
    current_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashing() {
        assert_eq!(52, hash("HASH"));
        assert_eq!(30, hash("rn=1"));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(1320, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(515974, part1("input.txt"));
    }
}
