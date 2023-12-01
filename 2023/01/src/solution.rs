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
            let f: char = find_first(line);
            let l: char = find_last(line);
            format!("{f}{l}").parse::<u32>().unwrap()
        })
        .sum()
}

fn find_last(line: &str) -> char {
    let line = line.chars().rev().collect::<String>();
    let mut i = 0;
    while i < line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            return c;
        } else if line[i..].starts_with("eno") {
            return '1';
        } else if line[i..].starts_with("owt") {
            return '2';
        } else if line[i..].starts_with("eerht") {
            return '3';
        } else if line[i..].starts_with("ruof") {
            return '4';
        } else if line[i..].starts_with("evif") {
            return '5';
        } else if line[i..].starts_with("xis") {
            return '6';
        } else if line[i..].starts_with("neves") {
            return '7';
        } else if line[i..].starts_with("thgie") {
            return '8';
        } else if line[i..].starts_with("enin") {
            return '9';
        }
        i += 1;
    }
    panic!("no digit found")
}

fn find_first(line: &str) -> char {
    let mut i = 0;
    while i < line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            return c;
        } else if line[i..].starts_with("one") {
            return '1';
        } else if line[i..].starts_with("two") {
            return '2';
        } else if line[i..].starts_with("three") {
            return '3';
        } else if line[i..].starts_with("four") {
            return '4';
        } else if line[i..].starts_with("five") {
            return '5';
        } else if line[i..].starts_with("six") {
            return '6';
        } else if line[i..].starts_with("seven") {
            return '7';
        } else if line[i..].starts_with("eight") {
            return '8';
        } else if line[i..].starts_with("nine") {
            return '9';
        }
        i += 1;
    }
    panic!("no digit found")
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
        assert_eq!(54265, part2("input.txt"));
    }
}
