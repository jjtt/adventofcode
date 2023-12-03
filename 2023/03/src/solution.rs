


use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Number {
    value: usize,
    start: Pos,
    end: Pos,
}

struct Schematic {
    numbers: Vec<Number>,
    parts: HashMap<Pos, char>,
}

impl Schematic {
    pub(crate) fn parts_sum(&self) -> usize {
        self.parts().iter().sum()
    }
    fn parts(&self) -> Vec<usize> {
        self.numbers
            .iter()
            .filter(|n| {
                let minx = n.start.x - 1;
                let maxx = n.end.x + 1;
                let miny = n.start.y - 1;
                let maxy = n.end.y + 1;
                for y in miny..=maxy {
                    for x in minx..=maxx {
                        if self.parts.get(&Pos { x, y }).is_some() {
                            return true;
                        }
                    }
                }
                false
            })
            .map(|n| n.value)
            .collect()
    }
    pub(crate) fn gears_sum(&self) -> usize {
        self.gears().iter().sum()
    }
    fn gears(&self) -> Vec<usize> {
        let numbers = self
            .numbers
            .iter()
            .flat_map(|n| (n.start.x..=n.end.x).map(move |x| ((x, n.start.y), n)))
            .collect::<HashMap<_, _>>();

        self.parts
            .iter()
            .filter(|&(_p, c)| *c == '*')
            .map(|(p, _c)| {
                let mut nums = HashSet::new();
                if let Some(n) = numbers.get(&(p.x - 1, p.y - 1)) {
                    nums.insert(*n);
                }
                if let Some(n) = numbers.get(&(p.x - 1, p.y)) {
                    nums.insert(*n);
                }
                if let Some(n) = numbers.get(&(p.x - 1, p.y + 1)) {
                    nums.insert(*n);
                }
                if let Some(n) = numbers.get(&(p.x, p.y - 1)) {
                    nums.insert(*n);
                }
                if let Some(n) = numbers.get(&(p.x, p.y + 1)) {
                    nums.insert(*n);
                }
                if let Some(n) = numbers.get(&(p.x + 1, p.y - 1)) {
                    nums.insert(*n);
                }
                if let Some(n) = numbers.get(&(p.x + 1, p.y)) {
                    nums.insert(*n);
                }
                if let Some(n) = numbers.get(&(p.x + 1, p.y + 1)) {
                    nums.insert(*n);
                }
                nums
            })
            .filter(|nums| nums.len() == 2)
            .map(|nums| {
                let mut iter = nums.iter();
                iter.next().unwrap().value * iter.next().unwrap().value
            })
            .collect::<Vec<_>>()
    }
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut numbers = Vec::new();
        let mut parts = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            let y = y + 1;
            let mut number = None;
            for (x, c) in line.chars().enumerate() {
                let x = x + 1;
                match c {
                    '.' => {
                        if let Some(number) = number.take() {
                            numbers.push(number);
                        }
                    }
                    c if c.is_ascii_digit() => {
                        if number.is_none() {
                            number = Some(Number {
                                value: c.to_digit(10).unwrap() as usize,
                                start: Pos { x, y },
                                end: Pos { x, y },
                            });
                        } else if let Some(number) = number.as_mut() {
                            number.value *= 10;
                            number.value += c.to_digit(10).unwrap() as usize;
                            number.end = Pos { x, y };
                        }
                    }
                    _ => {
                        parts.insert(Pos { x, y }, c);
                        if let Some(number) = number.take() {
                            numbers.push(number);
                        }
                    }
                }
            }
            if let Some(number) = number.take() {
                numbers.push(number);
            }
        }
        Ok(Schematic { numbers, parts })
    }
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let schematic = Schematic::from_str(&input).unwrap();
    schematic.parts_sum()
}

pub fn part2(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let schematic = Schematic::from_str(&input).unwrap();
    schematic.gears_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(4361, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(531932, part1("input.txt"));
    }
    #[test]
    fn part2_sample() {
        assert_eq!(467835, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(73646890, part2("input.txt"));
    }
}
