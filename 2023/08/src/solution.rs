use num::Integer;
use std::collections::HashMap;
use std::fs::read_to_string;

use scan_fmt::scan_fmt;

pub fn parse(input: &str) -> (String, HashMap<String, (String, String)>) {
    let input = read_to_string(input).unwrap();
    let mut lines = input.lines();
    let instructions = lines.next().expect("a string").to_string();
    let lines = lines.skip(1);
    let map = lines
        .map(|line| {
            let (x, y, z) =
                scan_fmt!(line, "{} = ({},{})", String, String, String).expect("a valid line");
            (x, (y, z))
        })
        .collect::<HashMap<_, _>>();
    (instructions, map)
}

pub fn solve(instructions: String, map: HashMap<String, (String, String)>, start: &str) -> usize {
    let instructions = instructions.chars().cycle();
    let mut loc = start;
    instructions
        .enumerate()
        .find_map(|(i, dir)| {
            let options = map.get(loc).expect("a valid location");
            loc = match dir {
                'L' => &options.0,
                'R' => &options.1,
                _ => panic!("invalid direction"),
            };
            if loc.ends_with('Z') {
                Some(i)
            } else {
                None
            }
        })
        .expect("to find a path")
        + 1
}
pub fn part1(input: &str) -> usize {
    let (instructions, map) = parse(input);
    solve(instructions, map, "AAA")
}

pub fn part2(input: &str) -> usize {
    let (instructions, map) = parse(input);

    let starts = map.keys().filter(|k| k.ends_with('A')).collect::<Vec<_>>();

    starts
        .into_iter()
        .map(|start| solve(instructions.clone(), map.clone(), start))
        .fold(1, |acc, steps| acc.lcm(&steps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(2, part1("sample.txt"));
    }

    #[test]
    fn part1_sample2() {
        assert_eq!(6, part1("sample2.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(13019, part1("input.txt"));
    }

    #[test]
    fn part2_sample3() {
        assert_eq!(6, part2("sample3.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(13524038372771, part2("input.txt"));
    }
}
