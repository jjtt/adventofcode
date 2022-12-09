use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(dir: &str) -> Direction {
        match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid input: {dir}"),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Move {
    direction: Direction,
    count: usize,
}

#[derive(Copy, Clone)]
struct End {
    x: i64,
    y: i64,
}

impl End {
    fn move_towards(&mut self, d: &Direction) {
        match *d {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn follow(&mut self, head: &End) {
        match (head.x - self.x, head.y - self.y) {
            (-1, -2) | (-2, -1) | (-2, -2) => {
                self.x -= 1;
                self.y -= 1;
            }
            (1, -2) | (2, -1) | (2, -2) => {
                self.x += 1;
                self.y -= 1;
            }
            (1, 2) | (2, 1) | (2, 2) => {
                self.x += 1;
                self.y += 1;
            }
            (-1, 2) | (-2, 1) | (-2, 2) => {
                self.x -= 1;
                self.y += 1;
            }
            (2, 0) => self.x += 1,
            (-2, 0) => self.x -= 1,
            (0, 2) => self.y += 1,
            (0, -2) => self.y -= 1,
            _ => (),
        }
    }
}

struct Rope {
    head: End,
    tail: Vec<End>,
}

impl Rope {
    fn new(x: i64, y: i64, tail_length: usize) -> Rope {
        Rope {
            head: End { x, y },
            tail: (0..tail_length).map(|_| End { x, y }).collect(),
        }
    }

    fn perform(&mut self, moves: &Vec<Move>) -> usize {
        let mut tail_visited = HashSet::new();
        let last = self.tail.last().unwrap();
        tail_visited.insert((last.x, last.y));
        for Move { direction, count } in moves {
            for _ in 0..*count {
                self.head.move_towards(direction);
                let mut prev = self.head;
                for next in self.tail.iter_mut() {
                    next.follow(&prev);
                    prev = *next;
                }
                let last = self.tail.last().unwrap();
                tail_visited.insert((last.x, last.y));
            }
        }
        tail_visited.len()
    }
}

pub fn part1(input: &str) -> usize {
    let mut rope = Rope::new(0, 0, 1);
    rope.perform(&parse_moves(&read_to_string(input).unwrap()))
}

pub fn part2(input: &str) -> usize {
    let mut rope = Rope::new(0, 0, 9);
    rope.perform(&parse_moves(&read_to_string(input).unwrap()))
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| scan_fmt!(l, "{} {}", String, usize).unwrap())
        .map(|(d, count)| Move {
            direction: Direction::from(&d),
            count,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn parsing_moves() {
        let input = "R 4";
        let expected = vec![Move {
            direction: Direction::Right,
            count: 4,
        }];

        assert_eq!(expected, parse_moves(input));
    }

    #[test_case(0, 0, 0, 0 => (0, 0); "same pos")]
    #[test_case(0, 1, 0, 0 => (0, 0); "above")]
    #[test_case(0, -1, 0, 0 => (0, 0); "below")]
    #[test_case(1, 1, 0, 0 => (0, 0); "ne")]
    #[test_case(-1, -1, 0, 0 => (0, 0); "sw")]
    #[test_case(-2, -1, 0, 0 => (-1, -1); "knight")]
    #[test_case(2, -1, 0, 0 => (1, -1); "knight2")]
    #[test_case(0, 2, 0, 0 => (0, 1); "two above")]
    #[test_case(0, -2, 0, 0 => (0, -1); "two below")]
    #[test_case(2, -2, 0, 0 => (1, -1); "diagon alley")]
    fn following(x1: i64, y1: i64, x2: i64, y2: i64) -> (i64, i64) {
        let head = End { x: x1, y: y1 };
        let mut tail = End { x: x2, y: y2 };

        tail.follow(&head);

        (tail.x, tail.y)
    }

    #[test]
    fn longer_rope() {
        let mut rope = Rope::new(0, 0, 2);

        assert_eq!(1, rope.perform(&parse_moves("R 2")));

        assert_eq!(2, rope.head.x);
        assert_eq!(1, rope.tail.get(0).unwrap().x);
        assert_eq!(0, rope.tail.get(1).unwrap().x);

        assert_eq!(3, rope.perform(&parse_moves("R 2")));

        assert_eq!(4, rope.head.x);
        assert_eq!(3, rope.tail.get(0).unwrap().x);
        assert_eq!(2, rope.tail.get(1).unwrap().x);
    }

    #[test]
    fn part1_sample() {
        assert_eq!(13, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(1, part2("sample.txt"));
    }

    #[test]
    fn part2_sample2() {
        assert_eq!(36, part2("sample2.txt"));
    }
}
