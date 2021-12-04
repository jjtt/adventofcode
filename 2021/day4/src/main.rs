use std::collections::{HashMap, LinkedList};
use std::fs::read_to_string;

const SIZE: i32 = 5;

#[derive(Hash, Eq, PartialEq, Debug)]
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
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::collections::LinkedList;
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(0) ; "sample")]
    #[test_case("input.txt" => is eq(0) ; "input")]
    fn part1(input: &str) -> u32 {
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

        dbg!(numbers);
        dbg!(boards);

        0
    }
}
