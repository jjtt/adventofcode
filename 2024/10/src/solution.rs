use scan_fmt::scan_fmt;
use std::fs::read_to_string;
use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let input = input.trim();
    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();
    let map = input.lines().flat_map(|l| l.chars())
        .map(|c| c as u8 - b'0')
        .collect::<Vec<_>>();

    (0..map.len()).filter(|i| map[*i] == 0)
        .map(|i| count_trails(&map, i, cols, rows, 9, 1))
        .sum()
}

fn count_trails(map: &[u8], i: usize, cols: usize, rows: usize, end: u8, step: u8) -> usize {
    find_trailheads(map, i, cols, rows, end, step).iter().unique().count()
}
fn find_trailheads(map: &[u8], i: usize, cols: usize, rows: usize, end: u8, step: u8) -> Vec<usize> {
    if map[i] == end {
        return vec![i];
    }

    let mut heads = Vec::new();

    if i >= cols && map[i - cols] == map[i] + step {
        heads.extend_from_slice(&find_trailheads(map, i - cols, cols, rows, end, step));
    }
    if i < map.len() - cols && map[i + cols] == map[i] + step {
        heads.extend_from_slice(&find_trailheads(map, i + cols, cols, rows, end, step));
    }
    if i % cols > 0 && map[i - 1] == map[i] + step {
        heads.extend_from_slice(&find_trailheads(map, i - 1, cols, rows, end, step));
    }
    if i % cols < cols - 1 && map[i + 1] == map[i] + step {
        heads.extend_from_slice(&find_trailheads(map, i + 1, cols, rows, end, step));
    }

    heads
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
        assert_eq!(36, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(0, part1("input.txt"));
    }
}
