use anyhow::bail;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn parse(input: String) -> (i32, i32, HashMap<char, Vec<(i32, i32)>>) {
    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();
    let antennas = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c != '.' {
                    Some((c, (x as i32, y as i32)))
                } else {
                    None
                }
            })
        })
        .into_group_map();
    (cols as i32, rows as i32, antennas)
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let (cols, rows, antennas) = parse(input);

    antennas
        .values()
        .flat_map(|antennas| {
            antennas.iter().combinations(2).flat_map(|pair| {
                assert_eq!(2, pair.len());
                let (p1, p2) = (pair[0], pair[1]);
                let (a1, a2) = antinodes(*p1, *p2);
                [a1, a2]
            })
        })
        .filter(|p| p.0 >= 0 && p.0 < cols && p.1 >= 0 && p.1 < rows)
        .collect::<HashSet<_>>()
        .len()
}

pub fn part2(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let (cols, rows, antennas) = parse(input);

    antennas
        .values()
        .flat_map(|antennas| {
            antennas.iter().combinations(2).flat_map(|pair| {
                assert_eq!(2, pair.len());
                let (p1, p2) = (pair[0], pair[1]);
                harmonics(*p1, *p2, cols, rows)
            })
        })
        .collect::<HashSet<_>>()
        .len()
}

fn harmonics(p0: (i32, i32), p1: (i32, i32), cols: i32, rows: i32) -> Vec<(i32, i32)> {
    let dx = p1.0 - p0.0;
    let dy = p1.1 - p0.1;
    let mut result = Vec::new();
    let mut x = p0.0;
    let mut y = p0.1;
    while x >= 0 && x < cols && y >= 0 && y < rows {
        result.push((x, y));
        x -= dx;
        y -= dy;
    }
    x = p0.0 + dx;
    y = p0.1 + dy;
    while x >= 0 && x < cols && y >= 0 && y < rows {
        result.push((x, y));
        x += dx;
        y += dy;
    }
    result
}

fn antinodes(p0: (i32, i32), p1: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let dx = p1.0 - p0.0;
    let dy = p1.1 - p0.1;
    ((p1.0 + dx, p1.1 + dy), (p0.0 - dx, p0.1 - dy))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case((4,3), (5,5), (3,1) => true)]
    #[test_case((4,3), (5,5), (6,7) => true)]
    #[test_case((5,5), (4,3), (3,1) => true)]
    #[test_case((5,5), (4,3), (6,7) => true)]
    #[test_case((5,5), (4,3), (1,1) => false)]
    fn test_is_antinode(p1: (i32, i32), p2: (i32, i32), test: (i32, i32)) -> bool {
        let (a1, a2) = antinodes(p1, p2);

        dbg!(p1, p2, a1, a2);

        a1 == test || a2 == test
    }

    #[test]
    fn part1_sample() {
        assert_eq!(14, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(336, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(34, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(1131, part2("input.txt"));
    }
}
