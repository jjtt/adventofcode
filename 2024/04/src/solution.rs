use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

pub fn part1(input: &str) -> usize {
    let map = parse(input);

    map.iter()
        .filter(|(_, &c)| c == 'X')
        .map(|(pos, _)| count_xmases(&map, *pos))
        .sum()
}

fn parse(input: &str) -> HashMap<(usize, usize), char> {
    let input = read_to_string(input).unwrap();
    let map: HashMap<_, _> = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x + 1, y + 1), c))
        })
        .collect();
    map
}

fn count_xmases(map: &HashMap<(usize, usize), char>, pos: (usize, usize)) -> usize {
    let mut count = 0;
    for &dir in &[
        Direction::N,
        Direction::NE,
        Direction::E,
        Direction::SE,
        Direction::S,
        Direction::SW,
        Direction::W,
        Direction::NW,
    ] {
        let mut pos = pos;
        for l in "MAS".chars() {
            pos = match dir {
                Direction::N => (pos.0, pos.1 - 1),
                Direction::NE => (pos.0 + 1, pos.1 - 1),
                Direction::E => (pos.0 + 1, pos.1),
                Direction::SE => (pos.0 + 1, pos.1 + 1),
                Direction::S => (pos.0, pos.1 + 1),
                Direction::SW => (pos.0 - 1, pos.1 + 1),
                Direction::W => (pos.0 - 1, pos.1),
                Direction::NW => (pos.0 - 1, pos.1 - 1),
            };
            if pos.0 == 0 || pos.1 == 0 {
                break;
            }
            match map.get(&pos) {
                Some(&c) if c == l => {
                    if l == 'S' {
                        count += 1
                    }
                }
                _ => break,
            }
        }
    }
    count
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
        assert_eq!(18, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(2496, part1("input.txt"));
    }

    #[test]
    fn counting_xmases() {
        let map = parse("sample.txt");
        assert_eq!(1, count_xmases(&map, (5, 1)));
    }
}
