use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

pub fn part1(input: &str) -> i32 {
    let input = read_to_string(input).unwrap();
    let mut lines = input.lines();
    let games = lines
        .map(|line| {
            let (id, game) = line.split_once(": ").unwrap();
            let id = scan_fmt!(id, "Game {d}", i32).unwrap();
            (
                id,
                game.split(";")
                    .map(|round| {
                        round
                            .split(",")
                            .map(|colour| scan_fmt!(colour.trim(), "{d} {}", i32, String).unwrap())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

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
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(8, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(2285, part1("input.txt"));
    }
}
