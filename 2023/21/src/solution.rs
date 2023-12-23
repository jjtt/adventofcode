use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

pub fn part1_do(input: &str, steps: usize) -> usize {
    //todo!()
    0
}

pub fn part1(input: &str) -> usize {
    part1_do(input, 64)
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(16, part1_do("sample.txt", 6));
    }

    #[test]
    fn part1_input() {
        assert_eq!(0, part1("input.txt"));
    }
}
