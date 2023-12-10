use std::collections::{HashMap, HashSet};
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

fn find_loop(
    input: &str,
) -> (
    usize,
    usize,
    usize,
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
) {
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
    let mut tunnel = HashSet::from([start, creature1.0, creature2.0]);
    let mut left = HashSet::new();
    let mut right = HashSet::new();

    while creature1.0 != creature2.0 {
        left.extend(on_left(creature1, &map));
        right.extend(on_right(creature1, &map));
        left.extend(on_right(creature2, &map));
        right.extend(on_left(creature2, &map));

        steps += 1;
        creature1 = find_for(creature1, &map);
        creature2 = find_for(creature2, &map);
        tunnel.insert(creature1.0);
        tunnel.insert(creature2.0);
    }

    left.retain(|pos| !tunnel.contains(pos));
    right.retain(|pos| !tunnel.contains(pos));

    let (maxx, maxy) = map
        .keys()
        .fold((0, 0), |(maxx, maxy), &(x, y)| (maxx.max(x), maxy.max(y)));

    (steps, maxx, maxy, tunnel, left, right)
}

fn on_left(
    creature: ((usize, usize), Direction),
    map: &HashMap<(usize, usize), Pipe>,
) -> Vec<(usize, usize)> {
    let pipe = map.get(&creature.0).unwrap();
    match creature.1 {
        Direction::North => match pipe {
            Pipe::NS => vec![(creature.0 .0 - 1, creature.0 .1)],
            Pipe::SE => vec![
                (creature.0 .0 - 1, creature.0 .1),
                (creature.0 .0, creature.0 .1 - 1),
            ],
            _ => vec![],
        },
        Direction::East => match pipe {
            Pipe::EW => vec![(creature.0 .0, creature.0 .1 - 1)],
            Pipe::SW => vec![
                (creature.0 .0, creature.0 .1 - 1),
                (creature.0 .0 + 1, creature.0 .1),
            ],
            _ => vec![],
        },
        Direction::South => match pipe {
            Pipe::NS => vec![(creature.0 .0 + 1, creature.0 .1)],
            Pipe::NW => vec![
                (creature.0 .0 + 1, creature.0 .1),
                (creature.0 .0, creature.0 .1 + 1),
            ],
            _ => vec![],
        },
        Direction::West => match pipe {
            Pipe::EW => vec![(creature.0 .0, creature.0 .1 + 1)],
            Pipe::NE => vec![
                (creature.0 .0, creature.0 .1 + 1),
                (creature.0 .0 - 1, creature.0 .1),
            ],
            _ => vec![],
        },
    }
}

fn on_right(
    creature: ((usize, usize), Direction),
    map: &HashMap<(usize, usize), Pipe>,
) -> Vec<(usize, usize)> {
    let pipe = map.get(&creature.0).unwrap();
    match creature.1 {
        Direction::North => match pipe {
            Pipe::NS => vec![(creature.0 .0 + 1, creature.0 .1)],
            Pipe::SW => vec![
                (creature.0 .0 + 1, creature.0 .1),
                (creature.0 .0, creature.0 .1 - 1),
            ],
            _ => vec![],
        },
        Direction::East => match pipe {
            Pipe::EW => vec![(creature.0 .0, creature.0 .1 + 1)],
            Pipe::NW => vec![
                (creature.0 .0, creature.0 .1 + 1),
                (creature.0 .0 + 1, creature.0 .1),
            ],
            _ => vec![],
        },
        Direction::South => match pipe {
            Pipe::NS => vec![(creature.0 .0 - 1, creature.0 .1)],
            Pipe::NE => vec![
                (creature.0 .0 - 1, creature.0 .1),
                (creature.0 .0, creature.0 .1 + 1),
            ],
            _ => vec![],
        },
        Direction::West => match pipe {
            Pipe::EW => vec![(creature.0 .0, creature.0 .1 - 1)],
            Pipe::SE => vec![
                (creature.0 .0, creature.0 .1 - 1),
                (creature.0 .0 - 1, creature.0 .1),
            ],
            _ => vec![],
        },
    }
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

pub fn part1(input: &str) -> usize {
    find_loop(input).0
}

pub fn part2(input: &str) -> usize {
    let (_steps, maxx, maxy, tunnel, left, right) = find_loop(input);
    assert!(left.is_disjoint(&right));
    assert!(tunnel.is_disjoint(&left));
    assert!(tunnel.is_disjoint(&right));

    fill(&left, maxx, maxy, &tunnel)
        .or_else(|| fill(&right, maxx, maxy, &tunnel))
        .expect("left or right is the inside")
}

fn fill(
    fill_from: &HashSet<(usize, usize)>,
    maxx: usize,
    maxy: usize,
    tunnel: &HashSet<(usize, usize)>,
) -> Option<usize> {
    let mut fill_from = fill_from.iter().cloned().collect::<Vec<_>>();
    let offsets = [(0, 1), (1, 0), (0, -1_i32), (-1_i32, 0)];
    let mut seen = HashSet::new();
    while let Some((x, y)) = fill_from.pop() {
        seen.insert((x, y));
        for (x_off, y_off) in offsets.iter() {
            let x = (x as i32 + x_off) as usize;
            let y = (y as i32 + y_off) as usize;
            if x == 0 || x > maxx || y == 0 || y > maxy {
                return None;
            }
            if !tunnel.contains(&(x, y)) && seen.insert((x, y)) {
                fill_from.push((x, y));
            }
        }
    }
    Some(seen.len())
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

    #[test]
    fn part2_sample() {
        assert_eq!(1, part2("sample.txt"));
    }

    #[test]
    fn part2_sample2() {
        assert_eq!(10, part2("sample2.txt"));
    }

    #[test]
    fn part2_sample3() {
        assert_eq!(4, part2("sample3.txt"));
    }

    #[test]
    fn part2_sample4() {
        assert_eq!(8, part2("sample4.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(357, part2("input.txt"));
    }
}
