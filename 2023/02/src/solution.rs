use std::collections::HashMap;
use std::fs::read_to_string;

use scan_fmt::scan_fmt;

type Game = Vec<Vec<(i32, String)>>;

fn parse_games(input: &str) -> Vec<(i32, Game)> {
    let input = read_to_string(input).unwrap();
    let lines = input.lines();

    lines
        .map(|line| {
            let (id, game) = line.split_once(": ").unwrap();
            let id = scan_fmt!(id, "Game {d}", i32).unwrap();
            (
                id,
                game.split(';')
                    .map(|round| {
                        round
                            .split(',')
                            .map(|colour| scan_fmt!(colour.trim(), "{d} {}", i32, String).unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> i32 {
    let games = parse_games(input);

    games
        .into_iter()
        .filter(|(_, game)| {
            game.iter().all(|round| {
                round.iter().all(|(count, colour)| match colour.as_str() {
                    "red" => *count <= 12,
                    "green" => *count <= 13,
                    "blue" => *count <= 14,
                    _ => panic!("Unknown colour: {}", colour),
                })
            })
        })
        .map(|(id, _)| id)
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let games = parse_games(input);

    games.into_iter().map(|(_, game)| solve_game(game)).sum()
}

fn solve_game(game: Game) -> i32 {
    game.into_iter()
        .flatten()
        .fold(HashMap::new(), |mut acc, (count, colour)| {
            acc.entry(colour)
                .and_modify(|c: &mut i32| *c = (*c).max(count))
                .or_insert(count);
            acc
        })
        .values()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solving_sample_first_game() {
        let game = vec![
            vec![(3, "blue".to_string()), (4, "red".to_string())],
            vec![(1, "red".to_string()), (2, "green".to_string())],
            vec![(6, "blue".to_string()), (2, "green".to_string())],
        ];
        assert_eq!(48, solve_game(game));
    }

    #[test]
    fn solving_sample_second_game() {
        let game = vec![
            vec![(1, "blue".to_string()), (2, "green".to_string())],
            vec![
                (3, "green".to_string()),
                (4, "blue".to_string()),
                (1, "red".to_string()),
            ],
            vec![(1, "green".to_string()), (1, "blue".to_string())],
        ];
        assert_eq!(12, solve_game(game));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(8, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(2285, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(2286, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(77021, part2("input.txt"));
    }
}
