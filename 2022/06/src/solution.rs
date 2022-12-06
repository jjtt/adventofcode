use anyhow::bail;
use scan_fmt::scan_fmt;
use std::collections::VecDeque;
use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let signal = input.lines().next().unwrap();
    find_marker(signal)
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

fn find_marker(signal: &str) -> usize {
    let mut remaining = signal.chars();
    let mut buffer = VecDeque::new();
    buffer.push_back(0 as char);
    buffer.push_back(remaining.next().unwrap());
    buffer.push_back(remaining.next().unwrap());
    buffer.push_back(remaining.next().unwrap());
    for (index, c) in remaining.enumerate() {
        buffer.pop_front();
        buffer.push_back(c);
        let buf = buffer.clone();
        if !has_duplicates(buf) {
            return index + 4;
        }
    }

    dbg!(signal);
    panic!("Not found");
}

fn has_duplicates(buf: VecDeque<char>) -> bool {
    let mut counts = [0_usize; 256];
    for c in buf {
        let count = counts[c as usize] + 1;
        if count > 1 {
            return true;
        }
        counts[c as usize] = count;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicates() {
        assert!(has_duplicates(VecDeque::from(['a', 'a'])));
        assert!(!has_duplicates(VecDeque::from(['a', 'b'])));
        assert!(!has_duplicates(VecDeque::from(['b', 'a'])));
        assert!(!has_duplicates(VecDeque::from(['d', 'c', 'b', 'a'])));
        assert!(has_duplicates(VecDeque::from(['d', 'c', 'b', 'a', 'd'])));
    }

    #[test]
    fn samples() {
        assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(7, part1("sample.txt"));
    }
}
