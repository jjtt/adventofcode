use anyhow::bail;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

pub fn part1(input: &str) -> u32 {
    let input = read_to_string(input).unwrap();
    let lines = input.trim().lines();
    lines
        .map(|line| {
            let nums = line
                .chars()
                .filter_map(|c| c.to_digit(10).and(Some(c)))
                .collect::<Vec<_>>();
            let f = nums.first().unwrap();
            let l = nums.last().unwrap();
            format!("{f}{l}").parse::<u32>().unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let input = read_to_string(input).unwrap();
    let input = input.trim().to_lowercase();
    let lines = input.lines();
    lines
        .map(|line| {
            let line = replace_string_nums(line);

            let nums = line
                .chars()
                .filter_map(|c| {
                    c.to_digit(10).and(Some(c)).or_else(|| match c {
                        'A' => Some('1'),
                        'B' => Some('2'),
                        'C' => Some('3'),
                        'D' => Some('4'),
                        'E' => Some('5'),
                        'F' => Some('6'),
                        'G' => Some('7'),
                        'H' => Some('8'),
                        'I' => Some('9'),
                        _ => None,
                    })
                })
                .collect::<Vec<_>>();
            let f = nums.first().unwrap();
            let l = nums.last().unwrap();
            format!("{f}{l}").parse::<u32>().unwrap()
        })
        .sum()
}

fn replace_string_nums(line: &str) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < line.len() {
        if line[i..].starts_with("one") {
            out.push('A');
            i += 3;
        } else if line[i..].starts_with("two") {
            out.push('B');
            i += 3;
        } else if line[i..].starts_with("three") {
            out.push('C');
            i += 5;
        } else if line[i..].starts_with("four") {
            out.push('D');
            i += 4;
        } else if line[i..].starts_with("five") {
            out.push('E');
            i += 4;
        } else if line[i..].starts_with("six") {
            out.push('F');
            i += 3;
        } else if line[i..].starts_with("seven") {
            out.push('G');
            i += 5;
        } else if line[i..].starts_with("eight") {
            out.push('H');
            i += 5;
        } else if line[i..].starts_with("nine") {
            out.push('I');
            i += 4;
        } else {
            out.push(line.chars().nth(i).unwrap());
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        assert_eq!(142, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(54450, part1("input.txt"));
    }

    #[test]
    fn part2_sample2() {
        assert_eq!(281, part2("sample2.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(0, part2("input.txt"));
    }
}
