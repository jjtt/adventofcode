
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

fn parse2(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();

    let time = scan_fmt!(&time.replace(' ', ""), "Time:{}", usize).unwrap();
    let distance = scan_fmt!(&distance.replace(' ', ""), "Distance:{}", usize).unwrap();

    (time, distance)
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
    let input = read_to_string(input).unwrap();
    wins(parse2(input.trim()))
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

    #[test]
    fn part2_sample() {
        assert_eq!(71503, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(39132886, part2("input.txt"));
    }
}
