use crate::solution::Direction::{East, North, South, West};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops;

type Pos = (i32, i32);
enum Direction {
    North = 0,
    South = 1,
    West = 2,
    East = 3,
}

impl ops::Add<i32> for Direction {
    type Output = Direction;

    fn add(self, rhs: i32) -> Self::Output {
        match (self as i32 + rhs) % 4 {
            0 => North,
            1 => South,
            2 => West,
            3 => East,
            _ => unreachable!(),
        }
    }
}

impl ops::Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = self;
        match rhs {
            North => (x, y - 1),
            South => (x, y + 1),
            West => (x - 1, y),
            East => (x + 1, y),
        }
    }
}

fn parse_input(input: &str) -> HashSet<Pos> {
    let input = read_to_string(input).unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some((x as i32, y as i32)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn move_elves(input: &str, rounds: i32) -> i32 {
    let mut elves = parse_input(input);

    for round in 0..rounds {
        let proposed = propose(&elves, round % 4);
        let proposed_positions = proposed
            .values()
            .sorted()
            .group_by(|pos| *pos)
            .into_iter()
            .map(|(pos, group)| (pos, group.count()))
            .filter(|(_, count)| *count > 1)
            .map(|(pos, _)| *pos)
            .collect::<HashSet<_>>();
        elves = proposed
            .into_iter()
            .map(|(pos, new_pos)| {
                if proposed_positions.contains(&new_pos) {
                    pos
                } else {
                    new_pos
                }
            })
            .collect();
        //print_elves(&elves, round + 1, -3, -2, 10, 9);
    }

    let minx = elves.iter().map(|(x, _)| x).min().unwrap();
    let maxx = elves.iter().map(|(x, _)| x).max().unwrap();
    let miny = elves.iter().map(|(_, y)| y).min().unwrap();
    let maxy = elves.iter().map(|(_, y)| y).max().unwrap();
    (maxx - minx + 1) * (maxy - miny + 1) - elves.len() as i32
}

#[allow(dead_code)]
fn print_elves(elves: &HashSet<Pos>, round: i32, minx: i32, miny: i32, maxx: i32, maxy: i32) {
    println!("Round: {round}");
    for y in miny..=maxy {
        for x in minx..=maxx {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn propose(elves: &HashSet<Pos>, round: i32) -> HashMap<Pos, Pos> {
    let mut proposed = HashMap::new();

    for pos in elves.iter() {
        let new_pos = if no_neighbours(elves, pos) {
            *pos
        } else if let Some(new_pos) = try_move(elves, pos, North + round) {
            new_pos
        } else if let Some(new_pos) = try_move(elves, pos, South + round) {
            new_pos
        } else if let Some(new_pos) = try_move(elves, pos, West + round) {
            new_pos
        } else if let Some(new_pos) = try_move(elves, pos, East + round) {
            new_pos
        } else {
            *pos
        };
        proposed.insert(*pos, new_pos);
    }

    proposed
}

fn no_neighbours(elves: &HashSet<Pos>, pos: &Pos) -> bool {
    !elves.contains(&(*pos + East))
        && !elves.contains(&(*pos + East + South))
        && !elves.contains(&(*pos + South))
        && !elves.contains(&(*pos + South + West))
        && !elves.contains(&(*pos + West))
        && !elves.contains(&(*pos + West + North))
        && !elves.contains(&(*pos + North))
        && !elves.contains(&(*pos + North + East))
}

fn try_move(elves: &HashSet<Pos>, pos: &Pos, direction: Direction) -> Option<Pos> {
    match direction {
        North
            if !elves.contains(&(*pos + North + West))
                && !elves.contains(&(*pos + North))
                && !elves.contains(&(*pos + North + East)) =>
        {
            Some(*pos + North)
        }
        South
            if !elves.contains(&(*pos + South + West))
                && !elves.contains(&(*pos + South))
                && !elves.contains(&(*pos + South + East)) =>
        {
            Some(*pos + South)
        }
        West if !elves.contains(&(*pos + West + North))
            && !elves.contains(&(*pos + West))
            && !elves.contains(&(*pos + West + South)) =>
        {
            Some(*pos + West)
        }
        East if !elves.contains(&(*pos + East + North))
            && !elves.contains(&(*pos + East))
            && !elves.contains(&(*pos + East + South)) =>
        {
            Some(*pos + East)
        }
        _ => None,
    }
}

pub fn part1(input: &str) -> i32 {
    move_elves(input, 10)
}

pub fn part2(input: &str) -> i32 {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_with_0_rounds() {
        assert_eq!(3, move_elves("small.txt", 0));
    }

    #[test]
    fn small_with_3_rounds() {
        assert_eq!(25, move_elves("small.txt", 3));
    }

    #[test]
    fn small_with_10_rounds() {
        assert_eq!(25, move_elves("small.txt", 10));
    }

    #[test]
    fn propose_small_first_round() {
        let elves = parse_input("small.txt");
        let proposed = propose(&elves, 0);
        let mut proposed_positions = proposed.into_iter().map(|(_, pos)| pos).collect::<Vec<_>>();
        proposed_positions.sort();
        assert_eq!(
            vec![(2, 0), (2, 3), (2, 3), (3, 0), (3, 3)],
            proposed_positions
        );
    }

    #[test]
    fn propose_nothing_for_lone_elf() {
        let elves = vec![(0, 0)].into_iter().collect();
        let proposed = propose(&elves, 0);
        assert_eq!(1, proposed.len());
        assert!(proposed.contains_key(&(0, 0)));
        assert_eq!(Some(&(0, 0)), proposed.get(&(0, 0)));
    }

    #[test]
    fn propose_north_for_two_elves_side_by_side() {
        let elves = vec![(0, 0), (1, 0)].into_iter().collect();
        let proposed = propose(&elves, 0);
        assert_eq!(2, proposed.len());
        let proposed = proposed.into_values().sorted().collect::<Vec<_>>();
        assert!(proposed.contains(&(1, -1)));
        assert!(proposed.contains(&(0, -1)));
    }

    #[test]
    fn propose_south_and_west_for_two_elves_diagonally_side_by_side() {
        let elves = vec![(0, 0), (1, 1)].into_iter().collect();
        let proposed = propose(&elves, South as i32);
        dbg!(&proposed);
        assert_eq!(2, proposed.len());
        let proposed = proposed.into_values().sorted().collect::<Vec<_>>();
        assert!(proposed.contains(&(-1, 0)));
        assert!(proposed.contains(&(1, 2)));
    }

    #[test]
    fn all_elves_have_neighbours_in_small() {
        let elves = parse_input("small.txt");
        assert!(elves.iter().all(|pos| !no_neighbours(&elves, pos)));
    }

    #[test]
    fn all_elves_have_neighbours_in_sample() {
        let elves = parse_input("sample.txt");
        assert!(elves.iter().all(|pos| !no_neighbours(&elves, pos)));
    }

    #[test]
    fn lone_elf_has_no_neighbours() {
        let elves = vec![(0, 0)].into_iter().collect();
        assert!(no_neighbours(&elves, &(0, 0)));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(110, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(4045, part1("input.txt"));
    }
}
