use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::read_to_string;

enum Rock {
    Round,
    Cube,
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let mut height = 0;
    let mut width = 0;
    let rocks = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            height += 1;
            width = line.len();
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                '#' => Some(((x, y), Rock::Cube)),
                'O' => Some(((x, y), Rock::Round)),
                _ => panic!("invalid input"),
            })
        })
        .collect::<HashMap<(usize, usize), Rock>>();

    let mut total_load = 0;
    let mut columns = vec![0; width];
    for y in 0..height {
        for x in 0..width {
            let rock = rocks.get(&(x, y));
            match rock {
                Some(Rock::Cube) => {
                    columns[x] = y + 1;
                }
                Some(Rock::Round) => {
                    total_load += height - columns[x];
                    columns[x] += 1;
                }
                None => {}
            }
        }
    }

    total_load
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
        assert_eq!(136, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(110821, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(64, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(0, part2("input.txt"));
    }
}
