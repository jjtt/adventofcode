use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            &Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Empty,
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Start,
}

impl Pipe {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Pipe::Empty,
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'S' => Pipe::Start,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            '7' => Pipe::SW,
            'F' => Pipe::SE,
            _ => panic!("Unknown pipe char: {}", c),
        }
    }

    fn connects_from(&self, dir: Direction) -> bool {
        match self {
            Pipe::Empty => false,
            Pipe::NS => dir == Direction::North || dir == Direction::South,
            Pipe::EW => dir == Direction::East || dir == Direction::West,
            Pipe::SW => dir == Direction::North || dir == Direction::East,
            Pipe::SE => dir == Direction::North || dir == Direction::West,
            Pipe::NW => dir == Direction::South || dir == Direction::East,
            Pipe::NE => dir == Direction::South || dir == Direction::West,
            Pipe::Start => true,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let map = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x + 1, y + 1), Pipe::from_char(c)))
        })
        .collect::<HashMap<_, _>>();

    let &start = map
        .iter()
        .find(|(_, &ref pipe)| *pipe == Pipe::Start)
        .unwrap()
        .0;
    let mut steps = 1;
    let (mut creature1, mut creature2) = find(start, &map);

    while creature1.0 != creature2.0 {
        steps += 1;
        creature1 = find_for(creature1, &map);
        creature2 = find_for(creature2, &map);
    }
    steps
}

fn find_for(
    creature: ((usize, usize), Direction),
    map: &HashMap<(usize, usize), Pipe>,
) -> ((usize, usize), Direction) {
    let (c1, c2) = find(creature.0, map);
    if c1.1 != creature.1.opposite() {
        c1
    } else {
        c2
    }
}

fn find(
    pos: (usize, usize),
    map: &HashMap<(usize, usize), Pipe>,
) -> (((usize, usize), Direction), ((usize, usize), Direction)) {
    let current = map.get(&pos).expect("current");
    let mut neighbours = vec![];
    if let Some(pipe) = map.get(&(pos.0, pos.1 - 1)) {
        if pipe.connects_from(Direction::North) && current.connects_from(Direction::South) {
            neighbours.push(((pos.0, pos.1 - 1), Direction::North));
        }
    }
    if let Some(pipe) = map.get(&(pos.0, pos.1 + 1)) {
        if pipe.connects_from(Direction::South) && current.connects_from(Direction::North) {
            neighbours.push(((pos.0, pos.1 + 1), Direction::South));
        }
    }
    if let Some(pipe) = map.get(&(pos.0 - 1, pos.1)) {
        if pipe.connects_from(Direction::West) && current.connects_from(Direction::East) {
            neighbours.push(((pos.0 - 1, pos.1), Direction::West));
        }
    }
    if let Some(pipe) = map.get(&(pos.0 + 1, pos.1)) {
        if pipe.connects_from(Direction::East) && current.connects_from(Direction::West) {
            neighbours.push(((pos.0 + 1, pos.1), Direction::East));
        }
    }
    assert_eq!(neighbours.len(), 2);
    (neighbours[0], neighbours[1])
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
        assert_eq!(4, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(6931, part1("input.txt"));
    }
}
