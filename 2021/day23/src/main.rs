#[macro_use]
extern crate scan_fmt;

use std::cmp;

use std::fs::read_to_string;

type State = [char; 27];

fn parse_situation(s: String) -> State {
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

    let possibly_folded = lines.next().unwrap();
    let basement = if "  #########" == possibly_folded {
        (('A', 'B', 'C', 'D'), ('A', 'B', 'C', 'D'))
    } else {
        (
            scan_fmt!(possibly_folded, "  #{}#{}#{}#{}#", char, char, char, char).unwrap(),
            scan_fmt!(
                lines.next().unwrap(),
                "  #{}#{}#{}#{}#",
                char,
                char,
                char,
                char
            )
            .unwrap(),
        )
    };

    let mut hall = hallway[0..11].chars();

    [
        basement.1 .0,
        basement.0 .0,
        bottom.0,
        top.0,
        basement.1 .1,
        basement.0 .1,
        bottom.1,
        top.1,
        basement.1 .2,
        basement.0 .2,
        bottom.2,
        top.2,
        basement.1 .3,
        basement.0 .3,
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

fn print(state: State) -> String {
    let mut s = String::new();

    s += "#############\n";
    s += "#";
    s.extend(state[16..27].iter());
    s += "#\n";
    s += format!(
        "###{}#{}#{}#{}###\n",
        state[3], state[7], state[11], state[15]
    )
    .as_str();
    s += format!(
        "  #{}#{}#{}#{}#\n",
        state[2], state[6], state[10], state[14]
    )
    .as_str();
    s += format!("  #{}#{}#{}#{}#\n", state[1], state[5], state[9], state[13]).as_str();
    s += format!("  #{}#{}#{}#{}#\n", state[0], state[4], state[8], state[12]).as_str();
    s += "  #########\n";

    s
}

fn is_finished(state: &State) -> bool {
    *state
        == [
            'A', 'A', 'A', 'A', 'B', 'B', 'B', 'B', 'C', 'C', 'C', 'C', 'D', 'D', 'D', 'D', '.',
            '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
        ]
}

fn find_moves(state: &State) -> Vec<(State, usize)> {
    let pods = amphipods(state);

    let mut moves = vec![];

    for (c, pos) in pods {
        moves.extend(moves_for(c, pos, state));
    }

    moves
}

fn moves_for(amphipod_type: char, pos: usize, state: &State) -> Vec<(State, usize)> {
    if is_final_position_for(amphipod_type, pos, state) {
        return vec![];
    }

    let mut moves = vec![];

    if pos >= 16 {
        // moving in
        let (backbackback, backback, back, front) = match amphipod_type {
            'A' => (0, 1, 2, 3),
            'B' => (4, 5, 6, 7),
            'C' => (8, 9, 10, 11),
            'D' => (12, 13, 14, 15),
            _ => panic!("Unsupported amphipod type: {}", amphipod_type),
        };
        moves.extend(create_allowed_move(amphipod_type, pos, state, backbackback));
        moves.extend(create_allowed_move(amphipod_type, pos, state, backback));
        moves.extend(create_allowed_move(amphipod_type, pos, state, back));
        moves.extend(create_allowed_move(amphipod_type, pos, state, front));
    } else {
        // moving out

        let allowed_hallway_positions = vec![21, 19, 23, 17, 25, 16, 26];
        for hall_pos in allowed_hallway_positions {
            moves.extend(create_allowed_move(amphipod_type, pos, state, hall_pos));
        }
    }

    moves
}

fn is_final_position_for(amphipod_type: char, pos: usize, state: &State) -> bool {
    match pos {
        0 => 'A' == amphipod_type,
        1 => 'A' == amphipod_type && state[0] == 'A',
        2 => 'A' == amphipod_type && state[1] == 'A',
        3 => 'A' == amphipod_type && state[2] == 'A',
        4 => 'B' == amphipod_type,
        5 => 'B' == amphipod_type && state[4] == 'B',
        6 => 'B' == amphipod_type && state[5] == 'B',
        7 => 'B' == amphipod_type && state[6] == 'B',
        8 => 'C' == amphipod_type,
        9 => 'C' == amphipod_type && state[8] == 'C',
        10 => 'C' == amphipod_type && state[9] == 'C',
        11 => 'C' == amphipod_type && state[10] == 'C',
        12 => 'D' == amphipod_type,
        13 => 'D' == amphipod_type && state[12] == 'D',
        14 => 'D' == amphipod_type && state[13] == 'D',
        15 => 'D' == amphipod_type && state[14] == 'D',
        _ => false,
    }
}

fn create_allowed_move(
    amphipod_type: char,
    pos: usize,
    state: &State,
    target_pos: usize,
) -> Option<(State, usize)> {
    let cost = is_allowed_move_for(amphipod_type, pos, target_pos, state);
    if cost > 0 {
        let mut m = state.clone();
        m[pos] = '.';
        m[target_pos] = amphipod_type;
        Some((m, cost))
    } else {
        None
    }
}

fn is_allowed_move_for(
    amphipod_type: char,
    start_pos: usize,
    target_pos: usize,
    state: &State,
) -> usize {
    // doorways are always clear
    assert!(state[18] == '.' && state[20] == '.' && state[22] == '.' && state[24] == '.');

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

    let (s, t) = if start_pos < 16 {
        (start_pos, target_pos)
    } else {
        (target_pos, start_pos)
    };

    // can not move in/out?
    if back_of_room_blocked(s, state) {
        return 0;
    }

    let steps_out = 4 - (s % 4);

    let out = match s {
        0 | 1 | 2 | 3 => 18,
        4 | 5 | 6 | 7 => 20,
        8 | 9 | 10 | 11 => 22,
        12 | 13 | 14 | 15 => 24,
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

fn wrong_type_in_room(pos: usize, state: &State) -> bool {
    match pos {
        1 => state[0] != 'A',
        2 => state[0] != 'A' || state[1] != 'A',
        3 => state[0] != 'A' || state[1] != 'A' || state[2] != 'A',
        5 => state[4] != 'B',
        6 => state[4] != 'B' || state[5] != 'B',
        7 => state[4] != 'B' || state[5] != 'B' || state[6] != 'B',
        9 => state[8] != 'C',
        10 => state[8] != 'C' || state[9] != 'C',
        11 => state[8] != 'C' || state[9] != 'C' || state[10] != 'C',
        13 => state[12] != 'D',
        14 => state[12] != 'D' || state[13] != 'D',
        15 => state[12] != 'D' || state[13] != 'D' || state[14] != 'D',
        _ => false,
    }
}

fn room_matches_type(amphipod_type: char, pos: usize) -> bool {
    match pos {
        0 | 1 | 2 | 3 => 'A' == amphipod_type,
        4 | 5 | 6 | 7 => 'B' == amphipod_type,
        8 | 9 | 10 | 11 => 'C' == amphipod_type,
        12 | 13 | 14 | 15 => 'D' == amphipod_type,
        _ => true,
    }
}

fn back_of_room_blocked(pos: usize, state: &State) -> bool {
    (pos == 0 && (state[1] != '.' || state[2] != '.' || state[3] != '.'))
        || (pos == 1 && (state[2] != '.' || state[3] != '.'))
        || (pos == 2 && state[3] != '.')
        || (pos == 4 && (state[5] != '.' || state[6] != '.' || state[7] != '.'))
        || (pos == 5 && (state[6] != '.' || state[7] != '.'))
        || (pos == 6 && state[7] != '.')
        || (pos == 8 && (state[9] != '.' || state[10] != '.' || state[11] != '.'))
        || (pos == 9 && (state[10] != '.' || state[11] != '.'))
        || (pos == 10 && state[11] != '.')
        || (pos == 12 && (state[13] != '.' || state[14] != '.' || state[15] != '.'))
        || (pos == 13 && (state[14] != '.' || state[15] != '.'))
        || (pos == 14 && state[15] != '.')
}

fn amphipods(state: &State) -> [(char, usize); 16] {
    let mut pods = [(' ', 0); 16];
    let mut count_a = 0;
    let mut count_b = 0;
    let mut count_c = 0;
    let mut count_d = 0;
    for (i, c) in state.iter().enumerate() {
        match c {
            'A' => {
                pods[0 + count_a] = (*c, i);
                count_a += 1;
            }
            'B' => {
                pods[4 + count_b] = (*c, i);
                count_b += 1;
            }
            'C' => {
                pods[8 + count_c] = (*c, i);
                count_c += 1;
            }
            'D' => {
                pods[12 + count_d] = (*c, i);
                count_d += 1;
            }
            '.' => (),
            _ => panic!("Unexpected character: {}", c),
        }
    }
    pods.reverse();
    pods
}

fn dfs(state: &State) -> Vec<Vec<State>> {
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

fn dfs_min_cost(state: &State) -> (usize, Vec<State>) {
    if is_finished(state) {
        (0, vec![state.clone()])
    } else {
        let mut min = usize::MAX;
        let mut solution = vec![];
        for m in find_moves(state) {
            if m.1 > min {
                // can't find a cheap one here
                continue;
            }
            let s = dfs_min_cost(&m.0);
            if s.0 < usize::MAX && s.0 + m.1 < min {
                min = s.0 + m.1;
                solution = vec![state.clone()];
                solution.extend(s.1);
            }
        }
        (min, solution)
    }
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
        let state = parse_situation(read_to_string("sample1_2.txt").unwrap());

        assert_eq!(read_to_string("sample1_2.txt").unwrap(), print(state));
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

        println!("{}", print(state));

        let moves = find_moves(&state);

        for m in moves.iter() {
            println!("{}", print(m.0));
        }

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

        assert_eq!(1143, solution.0);
        assert_eq!(12, solution.1.len());
    }

    #[test]
    fn why_choose_an_expensive_extra_step_for_b() {
        let state = parse_situation(
            indoc! {"
                #############
                #.A.....A...#
                ###.#.#C#D###
                  #B#B#C#D#
                  #########
            "}
            .to_string(),
        );

        println!("{}", print(state));

        let solution = dfs_min_cost(&state);

        for m in &solution.1 {
            println!("{}", print(*m));
        }

        assert_eq!(5, solution.1.len());
        assert_eq!(59, solution.0); // not sure if 59 is the correct answer, but 79 isn't
    }

    #[test]
    fn why_choose_an_expensive_extra_step_for_b_2() {
        let state = parse_situation(
            indoc! {"
                #############
                #.A.........#
                ###B#.#C#D###
                  #A#B#C#D#
                  #########
            "}
            .to_string(),
        );

        let solution = dfs_min_cost(&state);

        for m in &solution.1 {
            println!("{}", print(*m));
        }

        assert_eq!(4, solution.1.len());
        assert_eq!(42, solution.0);
    }

    #[test]
    fn why_doesnt_a_go_home() {
        let state = parse_situation(
            indoc! {"
                #############
                #.A.....A...#
                ###.#B#C#D###
                  #.#B#C#D#
                  #########
              "}
            .to_string(),
        );

        println!("{}", print(state));

        let solution = dfs_min_cost(&state);

        for m in &solution.1 {
            println!("{}", print(*m));
        }

        assert_eq!(3, solution.1.len());
        assert_eq!(9, solution.0);
    }

    #[test]
    fn cost_for_deep_rooms() {
        let state = parse_situation(
            indoc! {"
                #############
                #DD.D.D.....#
                ###A#B#C#.###
                  #A#B#C#.#
                  #A#B#C#.#
                  #A#B#C#.#
                  #########
              "}
            .to_string(),
        );

        println!("{}", print(state));

        let solution = dfs_min_cost(&state);

        for m in &solution.1 {
            println!("{}", print(*m));
        }

        assert_eq!(5, solution.1.len());
        assert_eq!(33000, solution.0);
    }

    #[test]
    fn verify_part2_sample1_cost() {
        let moves = [
            parse_situation(
                indoc! {"
                #############
                #...........#
                ###B#C#B#D###
                  #D#C#B#A#
                  #D#B#A#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #..........D#
                ###B#C#B#.###
                  #D#C#B#A#
                  #D#B#A#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #A.........D#
                ###B#C#B#.###
                  #D#C#B#.#
                  #D#B#A#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #A........BD#
                ###B#C#.#.###
                  #D#C#B#.#
                  #D#B#A#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #A......B.BD#
                ###B#C#.#.###
                  #D#C#.#.#
                  #D#B#A#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.....B.BD#
                ###B#C#.#.###
                  #D#C#.#.#
                  #D#B#.#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA...C.B.BD#
                ###B#.#.#.###
                  #D#C#.#.#
                  #D#B#.#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.....B.BD#
                ###B#.#.#.###
                  #D#C#.#.#
                  #D#B#C#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA...C.B.BD#
                ###B#.#.#.###
                  #D#.#.#.#
                  #D#B#C#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.....B.BD#
                ###B#.#.#.###
                  #D#.#C#.#
                  #D#B#C#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA...B.B.BD#
                ###B#.#.#.###
                  #D#.#C#.#
                  #D#.#C#C#
                  #A#D#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.D.B.B.BD#
                ###B#.#.#.###
                  #D#.#C#.#
                  #D#.#C#C#
                  #A#.#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.D...B.BD#
                ###B#.#.#.###
                  #D#.#C#.#
                  #D#.#C#C#
                  #A#B#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.D.....BD#
                ###B#.#.#.###
                  #D#.#C#.#
                  #D#B#C#C#
                  #A#B#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.D......D#
                ###B#.#.#.###
                  #D#B#C#.#
                  #D#B#C#C#
                  #A#B#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.D...C..D#
                ###B#.#.#.###
                  #D#B#C#.#
                  #D#B#C#.#
                  #A#B#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.D......D#
                ###B#.#C#.###
                  #D#B#C#.#
                  #D#B#C#.#
                  #A#B#C#A#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.D.....AD#
                ###B#.#C#.###
                  #D#B#C#.#
                  #D#B#C#.#
                  #A#B#C#.#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.......AD#
                ###B#.#C#.###
                  #D#B#C#.#
                  #D#B#C#.#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.B.....AD#
                ###.#.#C#.###
                  #D#B#C#.#
                  #D#B#C#.#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.......AD#
                ###.#B#C#.###
                  #D#B#C#.#
                  #D#B#C#.#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA...D...AD#
                ###.#B#C#.###
                  #.#B#C#.#
                  #D#B#C#.#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.......AD#
                ###.#B#C#.###
                  #.#B#C#.#
                  #D#B#C#D#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #AA.D.....AD#
                ###.#B#C#.###
                  #.#B#C#.#
                  #.#B#C#D#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #A..D.....AD#
                ###.#B#C#.###
                  #.#B#C#.#
                  #A#B#C#D#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #...D.....AD#
                ###.#B#C#.###
                  #A#B#C#.#
                  #A#B#C#D#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #.........AD#
                ###.#B#C#.###
                  #A#B#C#D#
                  #A#B#C#D#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #..........D#
                ###A#B#C#.###
                  #A#B#C#D#
                  #A#B#C#D#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
            parse_situation(
                indoc! {"
                #############
                #...........#
                ###A#B#C#D###
                  #A#B#C#D#
                  #A#B#C#D#
                  #A#B#C#D#
                  #########
                "}
                .to_string(),
            ),
        ];

        let mut cost = 0;
        for i in 0..moves.len() {
            for m in find_moves(&moves[i]) {
                if m.0 == moves[i + 1] {
                    cost += m.1;
                }
            }
        }

        assert_eq!(44169, cost);
    }

    #[test_case("sample1.txt" => is eq(12521); "sample1")]
    #[test_case("input.txt" => is eq(14148); "input")]
    fn part1(input: &str) -> usize {
        let state = parse_situation(read_to_string(input).unwrap());

        let solution = dfs_min_cost(&state);

        for m in solution.1 {
            println!("{}", print(m));
        }

        solution.0
    }

    #[test_case("sample1_2.txt" => is eq(44169); "sample1")]
    #[test_case("input_2.txt" => is eq(43814); "input")]
    fn part2(input: &str) -> usize {
        let state = parse_situation(read_to_string(input).unwrap());

        let solution = dfs_min_cost(&state);

        for m in solution.1 {
            println!("{}", print(m));
        }

        solution.0
    }
}
