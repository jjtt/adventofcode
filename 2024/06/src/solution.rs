use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;

type Pos = (usize, usize);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self, pos: Pos) -> Pos {
        match self {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        }
    }
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let mut dir = Direction::Up;
    let mut start: Pos = (0, 0);
    let mut cols = 0;
    let mut rows = 0;
    let mut obstacles = HashSet::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            cols = cols.max(x + 1);
            rows = rows.max(y + 1);
            match c {
                '#' => {
                    obstacles.insert((x + 1, y + 1));
                }
                '^' => {
                    start = (x + 1, y + 1);
                }
                _ => {}
            }
        }
    }
    let mut visited = HashSet::new();
    while (start.0 > 0 && start.1 > 0 && start.0 <= cols && start.1 <= rows) {
        visited.insert(start);
        let pos = dir.step(start);
        if obstacles.contains(&pos) {
            dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            start = pos;
        }
    }
    visited.len()
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
        assert_eq!(41, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(5086, part1("input.txt"));
    }
}
