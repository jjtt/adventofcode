use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

#[derive(Debug)]
struct Pattern {
    pattern: Vec<u64>,
    transposed: Vec<u64>,
    maxx: usize,
    maxy: usize,
}

impl Pattern {
    fn mirror(&self) -> (usize, usize) {
        let vertical = self.find_mirror(&self.pattern, self.maxx);
        let horizontal = if vertical == 0 {
            self.find_mirror(&self.transposed, self.maxy)
        } else {
            0
        };
        (vertical, horizontal)
    }

    fn find_mirror(&self, pattern: &[u64], max: usize) -> usize {
        for i in 1..max {
            let shorter_side = i.min(max - i);
            if pattern.iter().all(|&p| {
                let left = (p << (64 - max + i - shorter_side)) >> (64 - shorter_side);
                let right = (p >> (max - i - shorter_side)) << (64 - shorter_side);
                let right = right.reverse_bits();
                left == right
            }) {
                return i;
            }
        }
        0
    }
}

fn transpose(input: &[u64], maxy: usize) -> Vec<u64> {
    let mut output = vec![0; input.len()];
    for (i, &value) in input.iter().enumerate() {
        for j in 0..maxy {
            let bit = (value >> j) & 1;
            output[maxy - j - 1] |= bit << (maxy - i - 1);
        }
    }
    output
}

fn parse(input: &str) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let mut pattern = Vec::new();
    let mut maxx = 0;
    let mut maxy = 0;
    for line in (input.trim().to_string() + "\n\n").lines() {
        if line.is_empty() {
            let transposed = transpose(&pattern, maxy);
            patterns.push(Pattern {
                pattern,
                transposed,
                maxx,
                maxy,
            });
            pattern = Vec::new();
            maxx = 0;
            maxy = 0;
        } else {
            maxy += 1;
            maxx = line.len();
            pattern
                .push(u64::from_str_radix(&line.replace('.', "0").replace('#', "1"), 2).unwrap());
        }
    }
    patterns
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let patterns = parse(&input);

    patterns
        .into_iter()
        .map(|p| p.mirror())
        .map(|(c, r)| c + 100 * r)
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
    fn parsing() {
        let patterns = parse(
            r#"
..#
...
...

..#
..#
...
            "#,
        );
        assert_eq!(2, patterns.len());
        assert_eq!(vec![0b001, 0b000, 0b000], patterns[0].pattern);
        assert_eq!(vec![0b000, 0b000, 0b100], patterns[0].transposed);
        assert_eq!(vec![0b001, 0b001, 0b000], patterns[1].pattern);
        assert_eq!(vec![0b000, 0b000, 0b110], patterns[1].transposed);
    }

    #[test]
    fn mirroring() {
        let patterns = parse("#..#\n#..#\n\n");
        assert_eq!((2, 1), patterns[0].mirror());
    }

    #[test]
    fn part1_sample() {
        assert_eq!(405, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(27505, part1("input.txt"));
    }
}
