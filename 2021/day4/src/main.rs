#![feature(slice_group_by)]

use std::collections::HashMap;
use std::fs::read_to_string;

const SIZE: i32 = 5;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Tile {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Board {
    marked: Vec<Tile>,
    numbers: HashMap<i32, Tile>,
}

impl Board {
    fn new(rows: &Vec<&str>) -> Board {
        assert_eq!("", *rows.get(0).unwrap());
        let mut numbers = HashMap::new();
        for y in 0..SIZE {
            let row = rows.get(y as usize + 1).unwrap();
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

    fn play(&mut self, num: i32) {
        let tile = self.numbers.get(&num);
        if tile.is_some() {
            let _ = &self.marked.push(tile.unwrap().clone());
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
        let mut by_rows = self.marked.clone();
        by_rows.sort_by(|t1, t2| t1.y.cmp(&t2.y));
        let mut by_cols = self.marked.clone();
        by_cols.sort_by(|t1, t2| t1.x.cmp(&t2.x));

        self.marked.len() as i32 >= SIZE
            && (by_cols
                .group_by(|t1, t2| t1.x == t2.x)
                .map(|col| col.len())
                .max()
                .unwrap() as i32
                >= SIZE
                || by_rows
                    .group_by(|t1, t2| t1.y == t2.y)
                    .map(|row| row.len())
                    .max()
                    .unwrap() as i32
                    >= SIZE)
    }
}

fn main() {
    println!("Hello, world!");
}

fn play(boards: &mut Vec<Board>, numbers: Vec<i32>) -> (i32, Board) {
    for n in numbers.iter() {
        for b in &mut *boards {
            b.play(*n);

            if b.is_winner() {
                return (*n, b.clone());
            }
        }
    }
    panic!("No winners")
}

#[cfg(test)]
mod test {
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
        assert!(Board {
            marked: vec![
                Tile { x: 4, y: 4 },
                Tile { x: 4, y: 0 },
                Tile { x: 3, y: 1 },
                Tile { x: 4, y: 3 },
                Tile { x: 1, y: 3 },
                Tile { x: 2, y: 0 },
                Tile { x: 2, y: 2 },
                Tile { x: 0, y: 4 },
                Tile { x: 1, y: 4 },
                Tile { x: 0, y: 0 },
                Tile { x: 1, y: 0 },
                Tile { x: 3, y: 0 },
            ],
            numbers: HashMap::new(),
        }
        .is_winner());
    }

    #[test_case("sample1.txt" => is eq(4512) ; "sample")]
    #[test_case("input.txt" => is eq(33348) ; "input")]
    fn part1(input: &str) -> i32 {
        let input = read_to_string(input).unwrap();

        let lines: Vec<&str> = input.trim().lines().collect();

        let numbers: Vec<i32> = lines
            .get(0)
            .unwrap()
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let mut boards: Vec<Board> = Vec::new();
        for b in lines[1..].chunks(SIZE as usize + 1) {
            boards.push(Board::new(&b.to_vec()));
        }

        dbg!(&numbers);
        dbg!(&boards);

        let (last_number, winner) = play(&mut boards, numbers);

        last_number * winner.sum_unused()
    }
}
