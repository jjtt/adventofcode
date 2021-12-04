#![feature(slice_group_by)]

use std::collections::{HashMap, LinkedList};
use std::fs::read_to_string;

const SIZE: i32 = 5;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Tile {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Board {
    marked: Vec<Tile>,
    numbers: HashMap<i32, Tile>,
}

impl Board {
    fn new(rows: &mut LinkedList<&str>) -> Board {
        assert_eq!("", rows.pop_front().unwrap());
        let mut numbers = HashMap::new();
        for y in 0..SIZE {
            let row = rows.pop_front().unwrap();
            for x in 0..SIZE {
                let string = row.chars().skip(x as usize * 3).take(2).collect::<String>();
                let number = string.trim().parse::<i32>().unwrap();
                numbers.insert(number, Tile { x, y });
            }
        }
        assert_eq!(SIZE * SIZE, numbers.len() as i32);
        Board {
            marked: vec![],
            numbers: numbers,
        }
    }

    fn sum_unused(&self) -> i32 {
        let mut sum = 0;
        for (num, tile) in &self.numbers {
            if !self.marked.contains(&tile) {
                sum += num;
            }
        }
        sum
    }

    fn is_winner(&self) -> bool {
        self.marked.len() as i32 >= SIZE
            && (self
                .marked
                .group_by(|t1, t2| t1.x == t2.x)
                .map(|col| col.len())
                .max()
                .unwrap() as i32
                >= SIZE
                || self
                    .marked
                    .group_by(|t1, t2| t1.y == t2.y)
                    .map(|col| col.len())
                    .max()
                    .unwrap() as i32
                    >= SIZE)
    }
}

fn main() {
    println!("Hello, world!");
}

fn play(boards: &Vec<Board>, numbers: Vec<i32>) -> (i32, &Board) {
    (0, boards.get(0).unwrap())
}

#[cfg(test)]
mod test {
    use std::collections::LinkedList;
    use test_case::test_case;

    use super::*;

    #[test]
    fn sum_unused() {
        assert_eq!(
            42,
            Board {
                marked: vec![Tile { x: 1, y: 1 }],
                numbers: [(42, Tile { x: 0, y: 0 })].iter().cloned().collect(),
            }
            .sum_unused()
        );

        assert_eq!(
            0,
            Board {
                marked: vec![Tile { x: 0, y: 0 }],
                numbers: [(42, Tile { x: 0, y: 0 })].iter().cloned().collect(),
            }
            .sum_unused()
        );
    }

    #[test]
    fn is_winner() {
        assert!(Board {
            marked: vec![
                Tile { x: 1, y: 0 },
                Tile { x: 1, y: 1 },
                Tile { x: 1, y: 2 },
                Tile { x: 1, y: 3 },
                Tile { x: 1, y: 4 },
            ],
            numbers: HashMap::new(),
        }
        .is_winner());
        assert!(Board {
            marked: vec![
                Tile { x: 0, y: 4 },
                Tile { x: 1, y: 4 },
                Tile { x: 2, y: 4 },
                Tile { x: 3, y: 4 },
                Tile { x: 4, y: 4 },
            ],
            numbers: HashMap::new(),
        }
        .is_winner());
        assert!(!Board {
            marked: vec![
                Tile { x: 0, y: 4 },
                Tile { x: 1, y: 4 },
                Tile { x: 2, y: 4 },
                Tile { x: 3, y: 4 },
                Tile { x: 4, y: 3 },
            ],
            numbers: HashMap::new(),
        }
        .is_winner());
        assert!(!Board {
            marked: vec![],
            numbers: HashMap::new(),
        }
        .is_winner());
    }

    #[test_case("sample1.txt" => is eq(4512) ; "sample")]
    #[test_case("input.txt" => is eq(0) ; "input")]
    fn part1(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        let mut lines: LinkedList<&str> = input.trim().lines().collect();

        let numbers: Vec<i32> = lines
            .pop_front()
            .unwrap()
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let mut boards: Vec<Board> = Vec::new();
        while !lines.is_empty() {
            boards.push(Board::new(&mut lines));
        }

        dbg!(&numbers);
        dbg!(&boards);

        let (last_number, winner) = play(&boards, numbers);

        last_number * winner.sum_unused()
    }
}
