use std::fs::read_to_string;

#[derive(Debug)]
struct Pattern {
    pattern: Vec<u64>,
    transposed: Vec<u64>,
    maxx: usize,
    maxy: usize,
}

impl Pattern {
    fn mirror(&self, with_smudge: bool) -> (usize, usize) {
        (
            self.find_mirror(&self.pattern, self.maxx, with_smudge),
            self.find_mirror(&self.transposed, self.maxy, with_smudge),
        )
    }

    fn find_mirror(&self, pattern: &[u64], max: usize, with_smudge: bool) -> usize {
        for i in 1..max {
            let shorter_side = i.min(max - i);
            let sum = pattern
                .iter()
                .map(|&p| {
                    let left = (p << (64 - max + i - shorter_side)) >> (64 - shorter_side);
                    let right = (p >> (max - i - shorter_side)) << (64 - shorter_side);
                    let right = right.reverse_bits();
                    let xor = left ^ right;
                    xor.count_ones()
                })
                .sum::<u32>();
            if (!with_smudge && sum == 0) || (with_smudge && sum == 1) {
                return i;
            }
        }
        0
    }
}

fn transpose(input: &[u64], maxx: usize) -> Vec<u64> {
    let mut output = vec![0; maxx];
    for (i, &value) in input.iter().enumerate() {
        for (j, outbit) in output.iter_mut().enumerate() {
            let bit = (value >> (maxx - j - 1)) & 1;
            *outbit |= bit << (input.len() - i - 1);
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
            let transposed = transpose(&pattern, maxx);
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

pub fn solve(input: &str, with_smudge: bool) -> usize {
    let input = read_to_string(input).unwrap();
    let patterns = parse(&input);

    patterns
        .into_iter()
        .map(|p| p.mirror(with_smudge))
        .map(|(c, r)| c + 100 * r)
        .sum()
}
pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
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
        assert_eq!((2, 1), patterns[0].mirror(false));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(405, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(27505, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(400, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(22906, part2("input.txt"));
    }
}
