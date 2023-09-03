use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
enum Action {
    Move(u32),
    Left,
    Right,
}

impl Action {
    pub(crate) fn parse_actions(input: &str) -> Vec<Action> {
        let mut actions = vec![];
        let mut num = 0;
        for (ndx, c) in input.chars().enumerate() {
            match c {
                'R' => {
                    actions.push(Action::Move(num));
                    num = 0;
                    actions.push(Action::Right);
                }
                'L' => {
                    actions.push(Action::Move(num));
                    num = 0;
                    actions.push(Action::Left);
                }
                c => {
                    num *= 10;
                    num += c.to_digit(10).expect("a number")
                }
            }
        }
        actions
    }
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<(usize, usize), bool>,
}

impl Map {
    fn parse_map(input: &str) -> (Map, Vec<Action>) {
        let (map, actions) = input
            .split_once("\n\n")
            .expect("One empty line to separate map from actions");
        let tiles = map
            .lines()
            .enumerate()
            .flat_map(|(row_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col_index, c)| (row_index + 1, col_index + 1, c))
                    .filter_map(|(row, col, c)| match c {
                        ' ' => None,
                        '.' => Some(((row, col), true)),
                        '#' => Some(((row, col), false)),
                        _ => panic!("{} not supported", c),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        (Map { tiles }, Action::parse_actions(actions.trim()))
    }
}

pub fn part1(input: &str) -> i32 {
    //todo!()
    0
}

pub fn part2(input: &str) -> i32 {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parsing_actions() {
        let actions = Action::parse_actions("10R5L");
        assert_eq!(
            vec![
                Action::Move(10),
                Action::Right,
                Action::Move(5),
                Action::Left
            ],
            actions
        );
    }

    #[test]
    fn parsing() {
        let input = indoc! {"
              . 
            ... 
              .#

            L1R
        "};

        let (map, actions) = Map::parse_map(input);

        dbg!(&map);
        dbg!(&actions);

        assert_eq!(6, map.tiles.len());
        assert!(matches!(map.tiles.get(&(1, 3)), Some(true)));
        assert!(matches!(map.tiles.get(&(2, 1)), Some(true)));
        assert!(matches!(map.tiles.get(&(2, 2)), Some(true)));
        assert!(matches!(map.tiles.get(&(2, 3)), Some(true)));
        assert!(matches!(map.tiles.get(&(3, 3)), Some(true)));
        assert!(matches!(map.tiles.get(&(3, 4)), Some(false)));

        assert_eq!(
            vec![
                Action::Move(0),
                Action::Left,
                Action::Move(1),
                Action::Right
            ],
            actions
        )
    }

    #[test]
    fn part1_sample() {
        assert_eq!(6032, part1("sample.txt"));
    }
}
