use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

pub fn part1(input: &str) -> u32 {
    let input = read_to_string(input).unwrap();
    let lines = input.trim().lines();
    lines.map(|line| {
        let nums = line.chars().filter_map(|c| {
            c.to_digit(10).and(Some(c))
        }).collect::<Vec<_>>();
        let f = nums.first().unwrap();
        let l = nums.last().unwrap();
        format!("{f}{l}").parse::<u32>().unwrap()
    }).sum()


}

pub fn part2(input: &str) -> u32 {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(142, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(54450, part1("input.txt"));
    }
}
