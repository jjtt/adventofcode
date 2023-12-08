use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn solve(input: &str, part2: bool) -> usize {
    let input = read_to_string(input).unwrap();
    let mut lines = input.lines();
    let instructions = lines.next().expect("a string").chars().cycle();
    let lines = lines.skip(1);
    let map = lines
        .map(|line| {
            let (x, y, z) =
                scan_fmt!(line, "{} = ({},{})", String, String, String).expect("a valid line");
            (x, (y, z))
        })
        .collect::<HashMap<_, _>>();

    let mut loc = if part2 {
        map.keys().filter(|l| l.ends_with('A')).cloned().collect()
    } else {
        vec!["AAA".to_string()]
    };
    instructions
        .enumerate()
        .find_map(|(i, dir)| {
            loc = loc
                .iter()
                .map(|l| map.get(l).expect("a valid location"))
                .map(|(l, r)| {
                    match dir {
                        'L' => l,
                        'R' => r,
                        _ => panic!("invalid direction"),
                    }
                    .clone()
                })
                .collect();

            if loc.iter().all(|l| l.ends_with('Z')) {
                Some(i)
            } else {
                None
            }
        })
        .expect("to find a path")
        + 1
}
pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
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
        assert_eq!(0, part2("input.txt"));
    }
}
