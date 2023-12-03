use crate::Part;
use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
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
                return false;
            })
            .map(|n| n.value)
            .collect()
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
    //todo!()
    0
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
}
