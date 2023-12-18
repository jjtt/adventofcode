use std::collections::HashSet;
use std::fs::read_to_string;

use scan_fmt::scan_fmt;

fn flood_fill(x: usize, y: usize, map: &mut Vec<Vec<bool>>) {
    if !map[y][x] {
        map[y][x] = true;

        if y > 0 {
            flood_fill(x, y - 1, map);
        }
        if x > 0 {
            flood_fill(x - 1, y, map);
        }
        if y < map.len() - 1 {
            flood_fill(x, y + 1, map);
        }
        if x < map[0].len() - 1 {
            flood_fill(x + 1, y, map);
        }
    }
}

fn parse(input: &str) -> Vec<((i32, i32), usize, u32)> {
    let input = read_to_string(input).unwrap();
    input
        .trim()
        .lines()
        .map(|line| {
            let (dir, count, colour) =
                scan_fmt!(line, "{} {d} (#{x})", char, usize, [hex u32]).expect("valid input");
            let offset: (i32, i32) = match dir {
                'R' => (1, 0),
                'L' => (-1, 0),
                'U' => (0, -1),
                'D' => (0, 1),
                _ => panic!("Unknown direction {}", dir),
            };
            (offset, count, colour)
        })
        .collect::<Vec<_>>()
}

fn solve(plan: Vec<((i32, i32), usize, u32)>) -> usize {
    let trench = plan
        .into_iter()
        .fold(vec![(0, 0)], |mut trench, (offset, count, _colour)| {
            let mut pos = *trench.last().expect("trench not empty");
            for _ in 0..count {
                pos.0 += offset.0;
                pos.1 += offset.1;
                trench.push(pos);
            }
            trench
        })
        .into_iter()
        .collect::<HashSet<_>>();

    let minx = trench.iter().map(|(x, _)| x).min().expect("minx");
    let maxx = trench.iter().map(|(x, _)| x).max().expect("maxx");
    let miny = trench.iter().map(|(_, y)| y).min().expect("miny");
    let maxy = trench.iter().map(|(_, y)| y).max().expect("maxy");

    let mut dig = vec![vec![false; (maxx - minx + 1) as usize]; (maxy - miny + 1) as usize];
    for (x, y) in &trench {
        dig[(y - miny) as usize][(x - minx) as usize] = true;
    }

    (0..dig.len().min(dig[0].len()))
        .map(|i| {
            let mut d = dig.clone();
            flood_fill(i, i, &mut d);
            d.iter().flatten().filter(|&&b| b).count()
        })
        .max()
        .expect("max")
}

fn dehexify(colour: u32) -> ((i32, i32), usize) {
    let count = colour >> 4;
    let offset = match colour & 0xf {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        _ => panic!("Unknown direction {}", colour),
    };
    (offset, count as usize)
}

pub fn part1(input: &str) -> usize {
    let plan = parse(input);
    solve(plan)
}

pub fn part2(input: &str) -> usize {
    let plan = parse(input);
    let fixed_plan = plan
        .into_iter()
        .map(|(_offset, _count, colour)| {
            let (offset, count) = dehexify(colour);
            (offset, count, 0)
        })
        .collect();
    todo!("brute forcing doesn't cut it");
    solve(fixed_plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dehexifying() {
        assert_eq!(
            ((1, 0), 461937),
            dehexify(u32::from_str_radix("70c710", 16).unwrap())
        );
        assert_eq!(
            ((0, 1), 56407),
            dehexify(u32::from_str_radix("0dc571", 16).unwrap())
        );
    }

    #[test]
    fn part1_sample() {
        assert_eq!(38 + 24, part1("sample.txt"));
    }

    #[test]
    fn part1_sample2() {
        assert_eq!(2 * 38 + 131, part1("sample2.txt"));
    }

    #[test]
    fn part1_sample3() {
        assert_eq!(3 * 38 + 322, part1("sample3.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(61661, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(952408144115, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(0, part2("input.txt"));
    }
}
