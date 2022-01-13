#[macro_use]
extern crate scan_fmt;

use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

struct DeterministicDie {
    iter: Box<dyn Iterator<Item = i32>>,
    throws: i32,
}

impl DeterministicDie {
    fn new() -> DeterministicDie {
        let cycle = (1..=100).cycle();
        DeterministicDie {
            iter: Box::new(cycle),
            throws: 0,
        }
    }

    fn next3_mod10(&mut self) -> i32 {
        (self.next().unwrap() % 10 + self.next().unwrap() % 10 + self.next().unwrap() % 10) % 10
    }
}

impl Iterator for DeterministicDie {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.throws += 1;
        self.iter.next()
    }
}

fn parse_starting_positions(input: &str) -> ((i32, i32), (i32, i32)) {
    let s = read_to_string(input).unwrap();
    let mut lines = s.lines();

    (
        scan_fmt!(
            lines.next().unwrap(),
            "Player {d} starting position: {d}",
            i32,
            i32
        )
        .unwrap(),
        scan_fmt!(
            lines.next().unwrap(),
            "Player {d} starting position: {d}",
            i32,
            i32
        )
        .unwrap(),
    )
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test]
    fn die_as_iterator() {
        let mut die = DeterministicDie::new();

        assert_eq!(0, die.throws);

        assert_eq!(1, die.next().unwrap());
        assert_eq!(2, die.next().unwrap());
        assert_eq!(3, die.next().unwrap());

        assert_eq!(3, die.throws);
    }

    #[test]
    fn die_as_iterator_next3_mod10() {
        let mut die = DeterministicDie::new();

        assert_eq!(0, die.throws);

        assert_eq!(6, die.next3_mod10());
        assert_eq!(5, die.next3_mod10());
        assert_eq!(4, die.next3_mod10());
        assert_eq!(3, die.next3_mod10());

        assert_eq!(12, die.throws);
    }

    #[test_case("sample1.txt" => is eq(739785); "sample1")]
    #[test_case("input.txt" => is eq(903630); "input")]
    fn part1(input: &str) -> i32 {
        let ((_, pos1), (_, pos2)) = parse_starting_positions(input);
        let mut pos = vec![pos1 - 1, pos2 - 1];
        let mut score = vec![0; 2];
        let mut die = DeterministicDie::new();

        let mut cur = 0;
        while score[0] < 1000 && score[1] < 1000 {
            let cast = die.next3_mod10();
            pos[cur] = (pos[cur] + cast) % 10;
            score[cur] += pos[cur] + 1;
            cur = (cur + 1) % 2;
        }

        score[cur] * die.throws
    }
}
