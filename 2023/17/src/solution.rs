use std::fs::read_to_string;

use pathfinding::prelude::astar;

struct City {
    blocks: Vec<Vec<u8>>,
    goal_col: i32,
    goal_row: i32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn straight(&self) -> Self {
        self.clone()
    }

    fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    row: i32,
    col: i32,
    direction: Direction,
    straight: usize,
    heat_loss: u8,
}

impl City {
    fn successors(&self, node: &Node) -> Vec<(Node, usize)> {
        [
            (node.direction.straight(), node.straight + 1),
            (node.direction.left(), 1),
            (node.direction.right(), 1),
        ]
        .into_iter()
        .filter_map(|(dir, straight)| {
            let off = dir.offset();
            let row = node.row + off.0;
            let col = node.col + off.1;
            if straight <= 3
                && row >= 0
                && row <= self.goal_row as i32
                && col >= 0
                && col <= self.goal_col as i32
            {
                Some((
                    Node {
                        row,
                        col,
                        direction: dir,
                        straight,
                        heat_loss: self.blocks[row as usize][col as usize],
                    },
                    self.blocks[row as usize][col as usize] as usize,
                ))
            } else {
                None
            }
        })
        .collect()
    }
}

fn solve(input: &str) -> usize {
    let blocks = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - 48).collect())
        .collect::<Vec<Vec<u8>>>();
    let goal_row = blocks.len() as i32 - 1;
    let goal_col = blocks[0].len() as i32 - 1;
    let city = City {
        blocks,
        goal_row,
        goal_col,
    };

    let start = Node {
        row: 0,
        col: 0,
        direction: Direction::Right, // arbitrarily right or down
        straight: 0,
        heat_loss: city.blocks[0][0],
    };

    let result = astar(
        &start,
        |p| city.successors(p),
        |p| ((goal_row - p.row) + (goal_col - p.col)) as usize,
        |p| p.row == goal_row && p.col == goal_col,
    );

    result.expect("a path").1
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    solve(&input)
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        solve(
            r#"
136
134
124
124
111"#,
        );
    }

    #[test]
    fn part1_sample() {
        assert_eq!(102, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(861, part1("input.txt"));
    }
}
