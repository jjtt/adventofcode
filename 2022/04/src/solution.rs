use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    input.lines()
        .map(parse_row)
        .filter(contains)
        .count()
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

fn parse_row(row: &str) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
    let (start1, end1, start2, end2) =
        scan_fmt!(row, "{d}-{d},{d}-{d}", usize, usize, usize, usize).unwrap();
    (start1..=end1, start2..=end2)
}

fn contains(ranges: &(RangeInclusive<usize>, RangeInclusive<usize>)) -> bool {
    let mut ranges = ranges.clone();
    ranges.0.all(|v| ranges.1.contains(&v)) || ranges.1.all(|v| ranges.0.contains(&v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_rows() {
        assert_eq!((2..=8, 3..=7), parse_row("2-8,3-7"));
        assert_eq!((20..=80, 300..=700), parse_row("20-80,300-700"));
    }

    #[test]
    fn containing() {
        assert!(contains(&(2..=8, 3..=7)));
        assert!(contains(&(3..=7, 2..=8)));
        assert!(!contains(&(3..=7, 1..=5)));
        assert!(!contains(&(0..=1, 2..=3)));
        assert!(contains(&(1..=1, 1..=1)));
        assert!(contains(&(1..=2, 1..=1)));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(2, part1("sample.txt"));
    }
}
