use anyhow::bail;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let galaxies = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some((x, y)),
                _ => None,
            })
        })
        .collect::<HashSet<_>>();
    let xs = galaxies.iter().map(|(x, _)| *x).collect::<HashSet<_>>();
    let ys = galaxies.iter().map(|(_, y)| *y).collect::<HashSet<_>>();
    let &maxx = xs.iter().max().unwrap();
    let &maxy = ys.iter().max().unwrap();

    let empty_columns = (0..=maxx).filter(|&x| !xs.contains(&x)).collect::<Vec<_>>();
    let empty_rows = (0..=maxy).filter(|&y| !ys.contains(&y)).collect::<Vec<_>>();

    let expanded = galaxies
        .into_iter()
        .map(|(x, y)| {
            (
                x + empty_columns.iter().filter(|&c| x > *c).count(),
                y + empty_rows.iter().filter(|&r| y > *r).count(),
            )
        })
        .collect::<HashSet<_>>();

    expanded
        .iter()
        .combinations(2)
        .map(|pair| {
            let a = *pair[0];
            let b = *pair[1];
            ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
        })
        .sum()
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
        assert_eq!(374, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(10033566, part1("input.txt"));
    }
}
