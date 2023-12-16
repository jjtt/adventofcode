use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Add;

struct Grid {
    cells: HashMap<Position, Cell>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

enum Cell {
    MirrorUp,
    MirrorDown,
    SplitterUp,
    SplitterRight,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

fn parse(input: &str) -> Grid {
    let mut width = 0;
    let mut height = 0;
    let cells = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            height += 1;
            width = line.len();
            line.chars().enumerate().filter_map(move |(x, c)| {
                let cell = match c {
                    '.' => None,
                    '/' => Some(Cell::MirrorUp),
                    '\\' => Some(Cell::MirrorDown),
                    '|' => Some(Cell::SplitterUp),
                    '-' => Some(Cell::SplitterRight),
                    _ => panic!("Unknown cell: {}", c),
                };
                cell.map(|cell| (Position { x: x + 1, y: y + 1 }, cell))
            })
        })
        .collect();
    Grid {
        cells,
        width,
        height,
    }
}

fn walk(
    grid: &Grid,
    pos: Position,
    dir: Direction,
    mut visited: HashMap<Position, u8>,
) -> HashMap<Position, u8> {
    if pos.x == 0 || pos.y == 0 || pos.x == grid.width + 1 || pos.y == grid.height + 1 {
        return visited;
    }
    if let Some(&v) = visited.get(&pos) {
        if v & (dir as u8) != 0 {
            return visited;
        }
    }
    *visited.entry(pos).or_insert(0) |= dir as u8;
    match grid.cells.get(&pos) {
        None => walk(grid, pos + dir, dir, visited),
        Some(Cell::MirrorUp) => match dir {
            Direction::Up => walk(grid, pos + Direction::Right, Direction::Right, visited),
            Direction::Down => walk(grid, pos + Direction::Left, Direction::Left, visited),
            Direction::Left => walk(grid, pos + Direction::Down, Direction::Down, visited),
            Direction::Right => walk(grid, pos + Direction::Up, Direction::Up, visited),
        },
        Some(Cell::MirrorDown) => match dir {
            Direction::Up => walk(grid, pos + Direction::Left, Direction::Left, visited),
            Direction::Down => walk(grid, pos + Direction::Right, Direction::Right, visited),
            Direction::Left => walk(grid, pos + Direction::Up, Direction::Up, visited),
            Direction::Right => walk(grid, pos + Direction::Down, Direction::Down, visited),
        },
        Some(Cell::SplitterUp) => match dir {
            Direction::Up | Direction::Down => walk(grid, pos + dir, dir, visited),
            Direction::Left | Direction::Right => {
                let visited = walk(grid, pos + Direction::Up, Direction::Up, visited);
                walk(grid, pos + Direction::Down, Direction::Down, visited)
            }
        },
        Some(Cell::SplitterRight) => match dir {
            Direction::Up | Direction::Down => {
                let visited = walk(grid, pos + Direction::Left, Direction::Left, visited);
                walk(grid, pos + Direction::Right, Direction::Right, visited)
            }
            Direction::Left | Direction::Right => walk(grid, pos + dir, dir, visited),
        },
    }
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let cells = parse(&input);
    walk(
        &cells,
        Position { x: 1, y: 1 },
        Direction::Right,
        HashMap::new(),
    )
    .len()
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
        assert_eq!(46, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(7543, part1("input.txt"));
    }
}
