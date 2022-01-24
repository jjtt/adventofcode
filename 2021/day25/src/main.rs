use std::env;
use std::fs::read_to_string;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

fn parse_cucumber_map(input: String) -> (Vec<bool>, Vec<bool>, usize, usize) {
    (
        input
            .lines()
            .flat_map(|l| l.chars().map(|c| c == '>'))
            .collect(),
        input
            .lines()
            .flat_map(|l| l.chars().map(|c| c == 'v'))
            .collect(),
        input.lines().next().unwrap().len(),
        input.lines().count(),
    )
}

fn do_the_cucumber_dance(
    east: &mut Vec<bool>,
    south: &mut Vec<bool>,
    size_east: usize,
    size_south: usize,
) -> bool {
    let mut moved = false;
    moved |= do_moves(east, south, size_east, size_south, next_east);
    moved |= do_moves(south, east, size_east, size_south, next_south);

    moved
}

fn do_moves(
    cucumbers: &mut Vec<bool>,
    other: &mut Vec<bool>,
    size_east: usize,
    size_south: usize,
    next: fn(usize, usize, usize) -> usize,
) -> bool {
    let mut moves = vec![];
    for i in 0..cucumbers.len() {
        let to = next(i, size_east, size_south);
        if cucumbers[i] && !cucumbers[to] && !other[to] {
            moves.push((i, to))
        }
    }
    for (from, to) in &moves {
        cucumbers[*from] = false;
        cucumbers[*to] = true;
    }
    !moves.is_empty()
}

fn next_east(pos: usize, size_east: usize, _size_south: usize) -> usize {
    ((pos + 1) % size_east) + pos / size_east * size_east
}

fn next_south(pos: usize, size_east: usize, size_south: usize) -> usize {
    ((pos / size_east + 1) % size_south) * size_east + pos % size_east
}

fn part1(input: &str, do_print: bool) -> usize {
    let (mut east, mut south, size_east, size_south) =
        parse_cucumber_map(read_to_string(input).unwrap());

    let mut moves = 0;
    let mut moved = true;
    while moved {
        moves += 1;

        if do_print {
            print(&east, &south, size_east)
        };
        moved = do_the_cucumber_dance(&mut east, &mut south, size_east, size_south);
    }
    if do_print {
        print(&east, &south, size_east)
    };

    moves
}

fn print(east: &Vec<bool>, south: &Vec<bool>, size_east: usize) {
    print!("{esc}c", esc = 27 as char);
    for i in 0..east.len() {
        let c = if east[i] {
            '>'
        } else if south[i] {
            'v'
        } else {
            '.'
        };
        print!("{}", c);
        if (i + 1) % size_east == 0 {
            println!()
        }
    }
    stdout().flush().unwrap();
    sleep(Duration::from_millis(20));
}

fn main() {
    let input = env::args().nth(1).unwrap();
    part1(&input, true);
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test]
    fn finding_next() {
        let size_east = 2;
        let size_south = 3;

        assert_eq!(1, next_east(0, size_east, size_south));
        assert_eq!(0, next_east(1, size_east, size_south));

        assert_eq!(3, next_east(2, size_east, size_south));
        assert_eq!(2, next_east(3, size_east, size_south));

        assert_eq!(2, next_south(0, size_east, size_south));
        assert_eq!(4, next_south(2, size_east, size_south));
        assert_eq!(0, next_south(4, size_east, size_south));

        assert_eq!(3, next_south(1, size_east, size_south));
        assert_eq!(5, next_south(3, size_east, size_south));
        assert_eq!(1, next_south(5, size_east, size_south));
    }

    #[test_case("sample1.txt" => is eq(58); "sample1")]
    #[test_case("input.txt" => is eq(414); "input")]
    fn part1(input: &str) -> usize {
        super::part1(input, false)
    }
}
