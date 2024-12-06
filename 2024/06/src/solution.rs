use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;

type Pos = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

    fn bump(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let (mut dir, mut start, cols, rows, obstacles) = parse(input);
    let mut visited = HashSet::new();
    while (start.0 > 0 && start.1 > 0 && start.0 <= cols && start.1 <= rows) {
        visited.insert(start);
        let pos = dir.step(start);
        if obstacles.contains(&pos) {
            dir = dir.bump();
        } else {
            start = pos;
        }
    }
    visited.len()
}

fn parse(input: &str) -> (Direction, Pos, usize, usize, HashSet<(usize, usize)>) {
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
    (dir, start, cols, rows, obstacles)
}

pub fn part2(input: &str) -> usize {
    let (mut dir, start, cols, rows, obstacles) = parse(input);
    let mut pos = start;
    let mut new_obstacles = HashSet::new();
    let mut visited = HashSet::new();
    while (pos.0 > 0 && pos.1 > 0 && pos.0 <= cols && pos.1 <= rows) {
        visited.insert(pos);
        let mut check_dir = dir.bump();
        let mut check_pos = pos;
        let mut checked = HashSet::new();
        let new_pos = dir.step(pos);
        while !obstacles.contains(&new_pos)
            && (new_pos.0 > 0 && new_pos.1 > 0 && new_pos.0 <= cols && new_pos.1 <= rows)
            && (check_pos.0 > 0 && check_pos.1 > 0 && check_pos.0 <= cols && check_pos.1 <= rows)
        {
            if checked.contains(&(check_pos, check_dir)) {
                new_obstacles.insert(new_pos);
                break;
            }
            checked.insert((check_pos, check_dir));
            let new_check_pos = check_dir.step(check_pos);
            if obstacles.contains(&new_check_pos) || new_pos == new_check_pos {
                check_dir = check_dir.bump();
            } else {
                check_pos = new_check_pos;
            }
        }

        if obstacles.contains(&new_pos) {
            dir = dir.bump();
        } else {
            pos = new_pos;
        }
    }
    dbg!(visited.len());
    new_obstacles.len() - new_obstacles.contains(&start) as usize
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

    #[test]
    fn part2_sample() {
        assert_eq!(6, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(5086, part2("input.txt"));
    }
}
