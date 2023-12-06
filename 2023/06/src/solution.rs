use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

type Race = (usize, usize);

fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    let times = time
        .split_whitespace()
        .skip(1)
        .map(str::parse::<usize>)
        .map(Result::unwrap);
    let distances = distance
        .split_whitespace()
        .skip(1)
        .map(str::parse::<usize>)
        .map(Result::unwrap);
    times.zip(distances).collect()
}

fn wins(race: Race) -> usize {
    let time = race.0;
    let distance = race.1;
    (0..=time)
        .map(|t| t * (time - t))
        .filter(|&t| t > distance)
        .count()
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let races = parse(input.trim());
    races.into_iter().map(wins).product()
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
        assert_eq!(288, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(2374848, part1("input.txt"));
    }
}
