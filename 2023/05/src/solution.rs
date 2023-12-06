use scan_fmt::scan_fmt;
use std::fs::read_to_string;
use std::ops::Range;
use std::str::FromStr;

struct MapperEntry {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

struct Mapper {
    entries: Vec<MapperEntry>,
}

impl FromStr for Mapper {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .lines()
            .skip(1)
            .map(|l| {
                let (destination_start, source_start, length) =
                    scan_fmt!(l, "{d} {d} {d}", usize, usize, usize).unwrap();
                MapperEntry {
                    destination_start,
                    source_start,
                    length,
                }
            })
            .collect();
        Ok(Mapper { entries })
    }
}

impl Mapper {
    fn map(&self, value: Vec<Range<usize>>) -> Vec<Range<usize>> {
        value
            .into_iter()
            .flat_map(|r| self.map_range(r))
            .collect::<Vec<_>>()
    }

    fn map_range(&self, value: Range<usize>) -> Vec<Range<usize>> {
        for entry in &self.entries {
            if value >= entry.source_start && value < entry.source_start + entry.length {
                return entry.destination_start + value - entry.source_start;
            }
        }
        vec![value]
    }
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let mut blocks = input.split("\n\n");
    let seeds = blocks.next().unwrap();
    let seeds = seeds[7..]
        .split(' ')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .map(|s| vec![s..(s + 1)]);
    let mapppers = blocks
        .map(|b| b.parse::<Mapper>().unwrap())
        .collect::<Vec<_>>();

    seeds
        .map(|s| mapppers.iter().fold(s, |s, m| m.map(s)))
        .flatten()
        .flatten()
        .min()
        .expect("a min value")
}

pub fn part2(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let mut blocks = input.split("\n\n");
    let seeds = blocks.next().unwrap();
    let seeds = seeds[7..]
        .split(' ')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let seeds = seeds.chunks(2).map(|w| vec![w[0]..(w[0] + w[1])]);

    let mapppers = blocks
        .map(|b| b.parse::<Mapper>().unwrap())
        .collect::<Vec<_>>();

    seeds
        .map(|s| mapppers.iter().fold(s, |s, m| m.map(s)))
        .flatten()
        .flatten()
        .min()
        .expect("a min value")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(35, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(486613012, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(46, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(56931769, part2("input.txt"));
    }
}
