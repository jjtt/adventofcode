use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
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

    let mut loc = "AAA";
    instructions
        .enumerate()
        .find_map(|(i, dir)| {
            let options = map.get(loc).expect("a valid location");
            loc = match dir {
                'L' => &options.0,
                'R' => &options.1,
                _ => panic!("invalid direction"),
            };
            if loc == "ZZZ" {
                Some(i)
            } else {
                None
            }
        })
        .expect("to find a path")
        + 1
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
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
}
