use std::collections::HashSet;
use std::fs::read_to_string;

pub fn part1_do(input: &str, mut steps: usize) -> usize {
    let input = read_to_string(input).unwrap();
    let mut positions = HashSet::new();
    let plots = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let mut plots_from_row = HashSet::new();
            for (x, c) in line.chars().enumerate() {
                plots_from_row.insert(match c {
                    '.' => (x + 1, y + 1),
                    'S' => {
                        positions.insert((x + 1, y + 1));
                        (x + 1, y + 1)
                    }
                    '#' => continue,
                    _ => panic!("invalid char: {}", c),
                });
            }
            plots_from_row
        })
        .collect::<HashSet<_>>();

    while steps > 0 {
        let mut new_positions = HashSet::new();
        for &(x, y) in &positions {
            for &(x, y) in &[(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)] {
                if plots.contains(&(x, y)) {
                    new_positions.insert((x, y));
                }
            }
        }
        positions = new_positions;
        steps -= 1;
    }

    positions.len()
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
        assert_eq!(3646, part1("input.txt"));
    }
}
