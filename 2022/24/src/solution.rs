use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Valley {
    width: usize,
    height: usize,
    blizzards: Vec<(usize, usize, Direction)>,
    expedition: (usize, usize),
}

impl Valley {
    pub(crate) fn find_shortest_path(
        &mut self,
        (fromx, fromy): (usize, usize),
        (tox, toy): (usize, usize),
    ) -> usize {
        let result = pathfinding::prelude::astar(
            self,
            |valley| {
                let mut successors = Vec::new();

                let (x, y) = valley.expedition;
                let next = valley.move_blizzards();
                if (x, y) == (usize::MAX, usize::MAX) {
                    if next.is_free(fromx, fromy) {
                        let mut next = next.clone();
                        next.expedition = (fromx, fromy);
                        successors.push((next, 1));
                    }
                    successors.push((next, 1));
                } else {
                    if x > 0 && next.is_free(x - 1, y) {
                        let mut next = next.clone();
                        next.expedition = (x - 1, y);
                        successors.push((next, 1));
                    }
                    if x < valley.width - 1 && next.is_free(x + 1, y) {
                        let mut next = next.clone();
                        next.expedition = (x + 1, y);
                        successors.push((next, 1));
                    }
                    if y > 0 && next.is_free(x, y - 1) {
                        let mut next = next.clone();
                        next.expedition = (x, y - 1);
                        successors.push((next, 1));
                    }
                    if y < valley.height - 1 && next.is_free(x, y + 1) {
                        let mut next = next.clone();
                        next.expedition = (x, y + 1);
                        successors.push((next, 1));
                    }
                    if next.is_free(x, y) {
                        successors.push((next, 1));
                    }
                }

                successors
            },
            |valley| {
                let (x, y) = valley.expedition;
                if (x, y) == (usize::MAX, usize::MAX) {
                    valley.width + valley.height + 42 // should be more than any path
                } else {
                    usize_abs_diff(tox, x) + usize_abs_diff(toy, y)
                }
            },
            |valley| valley.expedition == (tox, toy),
        );

        let mut result = result.expect("a path");

        assert_eq!(result.1, result.0.len() - 1);

        let goal = result.0.pop().expect("a goal");

        // move expedition to safety
        self.expedition = (usize::MAX, usize::MAX);
        goal.move_blizzards();
        self.blizzards = goal.blizzards;

        result.1
    }
}

fn usize_abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

impl Valley {
    pub(crate) fn is_free(&self, x: usize, y: usize) -> bool {
        for (bx, by, _) in &self.blizzards {
            if *bx == x && *by == y {
                return false;
            }
        }
        true
    }
}

impl FromStr for Valley {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut blizzards = Vec::new();
        for (y, line) in s.lines().enumerate() {
            height += 1;
            width = 0;
            for (x, c) in line.chars().enumerate() {
                width += 1;
                let direction = match c {
                    '>' => Some(Direction::Right),
                    '<' => Some(Direction::Left),
                    '^' => Some(Direction::Up),
                    'v' => Some(Direction::Down),
                    _ => None,
                };

                if let Some(direction) = direction {
                    blizzards.push((x - 1, y - 1, direction));
                }
            }
        }
        height -= 2;
        width -= 2;
        Ok(Valley {
            width,
            height,
            blizzards,
            expedition: (usize::MAX, usize::MAX),
        })
    }
}

impl Valley {
    fn move_blizzards(&self) -> Self {
        let mut new_blizzards = Vec::new();
        for (x, y, direction) in &self.blizzards {
            let (x, y) = match direction {
                Direction::Left => ((x + self.width - 1) % self.width, *y),
                Direction::Right => ((x + self.width + 1) % self.width, *y),
                Direction::Up => (*x, (y + self.height - 1) % self.height),
                Direction::Down => (*x, (y + self.height + 1) % self.height),
            };
            new_blizzards.push((x, y, *direction));
        }

        let mut valley = self.clone();
        valley.blizzards = new_blizzards;
        valley
    }
}

pub fn part1(input: &str) -> usize {
    let mut valley = read_to_string(input).unwrap().parse::<Valley>().unwrap();

    1 + valley.find_shortest_path((0, 0), (valley.width - 1, valley.height - 1))
}

pub fn part2(input: &str) -> usize {
    let mut valley = read_to_string(input).unwrap().parse::<Valley>().unwrap();

    let there = valley.find_shortest_path((0, 0), (valley.width - 1, valley.height - 1));

    let and_back = valley.find_shortest_path((valley.width - 1, valley.height - 1), (0, 0));

    let and_there_again = valley.find_shortest_path((0, 0), (valley.width - 1, valley.height - 1));

    1 + there + and_back + and_there_again
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("simple.txt" => (6, 2, 2))]
    #[test_case("sample.txt" => (6, 4, 19))]
    fn parsing(input: &str) -> (usize, usize, usize) {
        let valley = read_to_string(input).unwrap().parse::<Valley>().unwrap();
        (valley.width, valley.height, valley.blizzards.len())
    }

    #[test]
    fn moving_blizzards() {
        let valley = read_to_string("simple.txt")
            .unwrap()
            .parse::<Valley>()
            .unwrap();
        let blizzards = valley.move_blizzards().blizzards;
        assert_eq!(2, blizzards.len());
        assert!(blizzards.contains(&(0, 0, Direction::Left)));
        assert!(blizzards.contains(&(5, 0, Direction::Left)));
    }

    #[test]
    fn is_free() {
        let mut valley = read_to_string("simple.txt")
            .unwrap()
            .parse::<Valley>()
            .unwrap();
        assert!(!valley.is_free(0, 0));
        valley.blizzards = valley.move_blizzards().blizzards;
        assert!(!valley.is_free(0, 0));
        valley.blizzards = valley.move_blizzards().blizzards;
        assert!(valley.is_free(0, 0));
    }

    #[test]
    fn part1_simple() {
        assert_eq!(9, part1("simple.txt"));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(18, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(266, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(54, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(853, part2("input.txt"));
    }
}
