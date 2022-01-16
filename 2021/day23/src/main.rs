#[macro_use]
extern crate scan_fmt;

use itertools::{all, Itertools};
use std::cmp;
use std::collections::HashSet;

use std::fs::read_to_string;
use std::ops::Range;

fn parse_situation(s: String) -> [char; 19] {
    let mut lines = s.lines();

    assert_eq!("#############", lines.next().unwrap());

    let hallway = scan_fmt!(lines.next().unwrap(), "#{}}#", String).unwrap();
    let top = scan_fmt!(
        lines.next().unwrap(),
        "###{}#{}#{}#{}###",
        char,
        char,
        char,
        char
    )
    .unwrap();
    let bottom = scan_fmt!(
        lines.next().unwrap(),
        "  #{}#{}#{}#{}#",
        char,
        char,
        char,
        char
    )
    .unwrap();

    assert_eq!("  #########", lines.next().unwrap());

    let mut hall = hallway[0..11].chars();

    [
        bottom.0,
        top.0,
        bottom.1,
        top.1,
        bottom.2,
        top.2,
        bottom.3,
        top.3,
        hall.next().unwrap(),
        hall.next().unwrap(),
        hall.next().unwrap(),
        hall.next().unwrap(),
        hall.next().unwrap(),
        hall.next().unwrap(),
        hall.next().unwrap(),
        hall.next().unwrap(),
        hall.next().unwrap(),
        hall.next().unwrap(),
        hall.next().unwrap(),
    ]
}

fn print(state: [char; 19]) -> String {
    let mut s = String::new();

    s += "#############\n";
    s += "#";
    s.extend(state[8..19].iter());
    s += "#\n";
    s += format!(
        "###{}#{}#{}#{}###\n",
        state[1], state[3], state[5], state[7]
    )
    .as_str();
    s += format!("  #{}#{}#{}#{}#\n", state[0], state[2], state[4], state[6]).as_str();
    s += "  #########\n";

    s
}

fn is_finished(state: &[char; 19]) -> bool {
    *state
        == [
            'A', 'A', 'B', 'B', 'C', 'C', 'D', 'D', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            '.', '.',
        ]
}

fn find_moves(state: &[char; 19]) -> Vec<([char; 19], usize)> {
    let pods = amphipods(state);

    let mut moves = vec![];

    for (c, pos) in pods {
        moves.extend(moves_for(c, pos, state));
    }

    moves
}

fn moves_for(amphipod_type: char, pos: usize, state: &[char; 19]) -> Vec<([char; 19], usize)> {
    if is_final_position_for(amphipod_type, pos, state) {
        return vec![];
    }

    let mut moves = vec![];

    if pos >= 8 {
        // moving in
        let (back, front) = match amphipod_type {
            'A' => (0, 1),
            'B' => (2, 3),
            'C' => (4, 5),
            'D' => (6, 7),
            _ => panic!("Unsupported amphipod type: {}", amphipod_type),
        };
        moves.extend(create_allowed_move(amphipod_type, pos, state, back));
        moves.extend(create_allowed_move(amphipod_type, pos, state, front));
    } else {
        // moving out
        let allowed_hallway_positions = vec![8, 9, 11, 13, 15, 17, 18];
        for hall_pos in allowed_hallway_positions {
            moves.extend(create_allowed_move(amphipod_type, pos, state, hall_pos));
        }
    }

    moves
}

fn is_final_position_for(amphipod_type: char, pos: usize, state: &[char; 19]) -> bool {
    match pos {
        0 => 'A' == amphipod_type,
        1 => 'A' == amphipod_type && state[0] == 'A',
        2 => 'B' == amphipod_type,
        3 => 'B' == amphipod_type && state[2] == 'B',
        4 => 'C' == amphipod_type,
        5 => 'C' == amphipod_type && state[4] == 'C',
        6 => 'D' == amphipod_type,
        7 => 'D' == amphipod_type && state[6] == 'D',
        _ => false,
    }
}

fn create_allowed_move(
    amphipod_type: char,
    pos: usize,
    state: &[char; 19],
    back: usize,
) -> Option<([char; 19], usize)> {
    let cost = is_allowed_move_for(amphipod_type, pos, back, state);
    if cost > 0 {
        let mut m = state.clone();
        m[pos] = '.';
        m[back] = amphipod_type;
        Some((m, cost))
    } else {
        None
    }
}

fn is_allowed_move_for(
    amphipod_type: char,
    start_pos: usize,
    target_pos: usize,
    state: &[char; 19],
) -> usize {
    // doorways are always clear
    assert!(state[10] == '.' && state[12] == '.' && state[14] == '.' && state[16] == '.');

    // target must be clear
    if state[target_pos] != '.' {
        return 0;
    }

    if !room_matches_type(amphipod_type, target_pos) {
        return 0;
    }

    if wrong_type_in_room(target_pos, state) {
        return 0;
    }

    let (s, t) = if start_pos < 8 {
        (start_pos, target_pos)
    } else {
        (target_pos, start_pos)
    };

    // can not move in/out?
    if back_of_room_blocked(s, state) {
        return 0;
    }

    let steps_out = 2 - (s % 2);

    let out = match s {
        0 | 1 => 10,
        2 | 3 => 12,
        4 | 5 => 14,
        6 | 7 => 16,
        _ => panic!(
            "Start or end must be in a room: {} (start:{}, end:{})",
            s, start_pos, target_pos
        ),
    };

    if !(cmp::min(out, t)..cmp::max(out, t))
        .skip(1)
        .all(|p| state[p] == '.')
    {
        0
    } else {
        (steps_out + if out < t { t - out } else { out - t }) * cost_for(amphipod_type)
    }
}

fn cost_for(amphipod_type: char) -> usize {
    match amphipod_type {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Unsupperted amphipod type: {}", amphipod_type),
    }
}

fn wrong_type_in_room(pos: usize, state: &[char; 19]) -> bool {
    match pos {
        1 => state[0] != 'A',
        3 => state[2] != 'B',
        5 => state[4] != 'C',
        7 => state[6] != 'D',
        _ => false,
    }
}

fn room_matches_type(amphipod_type: char, pos: usize) -> bool {
    match pos {
        0 | 1 => 'A' == amphipod_type,
        2 | 3 => 'B' == amphipod_type,
        4 | 5 => 'C' == amphipod_type,
        6 | 7 => 'D' == amphipod_type,
        _ => true,
    }
}

fn back_of_room_blocked(pos: usize, state: &[char; 19]) -> bool {
    (pos == 0 && state[1] != '.')
        || (pos == 2 && state[3] != '.')
        || (pos == 4 && state[5] != '.')
        || (pos == 6 && state[7] != '.')
}

fn amphipods(state: &[char; 19]) -> [(char, usize); 8] {
    let mut pods = [(' ', 0); 8];
    let mut seen_a = false;
    let mut seen_b = false;
    let mut seen_c = false;
    let mut seen_d = false;
    for (i, c) in state.iter().enumerate() {
        match c {
            'A' => {
                if seen_a {
                    pods[1] = (*c, i)
                } else {
                    pods[0] = (*c, i);
                    seen_a = true
                }
            }
            'B' => {
                if seen_b {
                    pods[3] = (*c, i)
                } else {
                    pods[2] = (*c, i);
                    seen_b = true
                }
            }
            'C' => {
                if seen_c {
                    pods[5] = (*c, i)
                } else {
                    pods[4] = (*c, i);
                    seen_c = true
                }
            }
            'D' => {
                if seen_d {
                    pods[7] = (*c, i)
                } else {
                    pods[6] = (*c, i);
                    seen_d = true
                }
            }
            '.' => (),
            _ => panic!("Unexpected character: {}", c),
        }
    }
    pods
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use test_case::test_case;

    use super::*;

    #[test]
    fn print_sample1() {
        let state = parse_situation(read_to_string("sample1.txt").unwrap());

        assert_eq!(read_to_string("sample1.txt").unwrap(), print(state));
    }

    #[test]
    fn only_one_possible_move_in_for_a() {
        let state = parse_situation(
            indoc! {"
                #############
                #A..........#
                ###.#B#C#D###
                  #A#B#C#D#
                  #########
        "}
            .to_string(),
        );

        let moves = find_moves(&state);

        assert_eq!(1, moves.len());

        assert!(is_finished(&moves.first().unwrap().0));
        assert_eq!(3, moves.first().unwrap().1);
    }

    #[test]
    fn only_one_possible_move_in_for_d() {
        let state = parse_situation(
            indoc! {"
                #############
                #.......D...#
                ###A#B#C#.###
                  #A#B#C#D#
                  #########
        "}
            .to_string(),
        );

        let moves = find_moves(&state);

        assert_eq!(1, moves.len());

        assert!(is_finished(&moves.first().unwrap().0));
        assert_eq!(2000, moves.first().unwrap().1);
    }

    #[test]
    fn two_possible_moves_out_for_d() {
        let state = parse_situation(
            indoc! {"
                #############
                #...A.B.C.D.#
                ###.#.#.#.###
                  #D#C#B#A#
                  #########
        "}
            .to_string(),
        );

        let moves = find_moves(&state);

        assert_eq!(2, moves.len());

        assert!(!is_finished(&moves.first().unwrap().0));
        assert!(!is_finished(&moves.last().unwrap().0));
        assert_ne!(moves.first().unwrap(), moves.last().unwrap());
        assert_eq!(7000, moves.first().unwrap().1 + moves.last().unwrap().1);
    }

    #[test]
    fn two_possible_moves_out_for_c_and_d() {
        let state = parse_situation(
            indoc! {"
                #############
                #.A...B.C.D.#
                ###.#.#.#.###
                  #D#C#B#A#
                  #########
        "}
            .to_string(),
        );

        let moves = find_moves(&state);

        assert_eq!(2, moves.len());

        assert!(!is_finished(&moves.first().unwrap().0));
        assert!(!is_finished(&moves.last().unwrap().0));
        assert_ne!(moves.first().unwrap(), moves.last().unwrap());
        assert_eq!(3300, moves.first().unwrap().1 + moves.last().unwrap().1);
    }

    #[test]
    fn find_solution_for_easy_case() {
        let state = parse_situation(
            indoc! {"
                #############
                #A..........#
                ###.#B#C#D###
                  #A#B#C#D#
                  #########
        "}
            .to_string(),
        );

        let solutions = dfs(&state);

        assert_eq!(1, solutions.len());
        assert_eq!(2, solutions.first().unwrap().len());
    }

    #[test]
    fn find_solution_for_slightly_harder_case() {
        let state = parse_situation(
            indoc! {"
                #############
                #AA.........#
                ###.#B#C#D###
                  #.#B#C#D#
                  #########
        "}
            .to_string(),
        );

        let solutions = dfs(&state);

        assert_eq!(1, solutions.len());

        for m in solutions.first().unwrap() {
            println!("{}", print(*m));
        }

        assert_eq!(3, solutions.first().unwrap().len());
    }

    #[test]
    fn find_solution_for_harder_case_based_on_input() {
        let state = parse_situation(
            indoc! {"
                #############
                #.A.........#
                ###.#A#C#D###
                  #B#C#B#D#
                  #########
          "}
            .to_string(),
        );

        let solution = dfs_min_cost(&state);


        for m in &solution.1 {
            println!("{}", print(*m));
        }

        assert_eq!(0, solution.0);
        assert_eq!(1, solution.1.len());
    }

    #[test_case("sample1.txt" => is eq(12521); "sample1")]
    #[test_case("input.txt" => is eq(0); "input")]
    fn part1(input: &str) -> usize {
        let state = parse_situation(read_to_string(input).unwrap());

        let solution = dfs_min_cost(&state);

        for m in solution.1 {
            println!("{}", print(m));
        }

        solution.0
    }

    fn dfs(state: &[char; 19]) -> Vec<Vec<[char; 19]>> {
        if is_finished(state) {
            vec![vec![state.clone()]]
        } else {
            let mut solutions = vec![];
            for m in find_moves(state) {
                for mut solution in dfs(&m.0) {
                    solution.insert(0, state.clone());
                    solutions.push(solution);
                }
            }
            solutions
        }
    }

    fn dfs_min_cost(state: &[char; 19]) -> (usize, Vec<[char; 19]>) {
        if is_finished(state) {
            (0, vec![state.clone()])
        } else {
            let mut min = usize::MAX;
            let mut solution = vec![];
            let mut move_cost = 0;
            for m in find_moves(state) {
                let s = dfs_min_cost(&m.0);
                if s.0 < min {
                    min = s.0;
                    move_cost = m.1;
                    solution = vec![state.clone()];
                    solution.extend(s.1);
                }
            }
            (min + move_cost, solution)
        }
    }
}
