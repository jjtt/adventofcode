use std::fs::read_to_string;

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let signal = input.lines().next().unwrap();
    find_marker(signal, 4)
}

pub fn part2(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let signal = input.lines().next().unwrap();
    find_marker(signal, 14)
}

pub fn find_marker(signal: &str, num: usize) -> usize {
    signal
        .as_bytes()
        .windows(num)
        .enumerate()
        .find(|(_, w)| !has_duplicates(w))
        .unwrap()
        .0
        + num
}

fn has_duplicates(buf: &[u8]) -> bool {
    let mut counts = [0_usize; u8::MAX as usize];
    for &c in buf {
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
        assert!(has_duplicates(&[1, 1]));
        assert!(!has_duplicates(&[1, 2]));
        assert!(!has_duplicates(&[2, 1]));
        assert!(!has_duplicates(&[4, 3, 2, 1]));
        assert!(has_duplicates(&[4, 3, 2, 1, 4]));
    }

    #[test]
    fn samples() {
        assert_eq!(5, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
        assert_eq!(6, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4));
        assert_eq!(10, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
        assert_eq!(11, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));
    }

    #[test]
    fn samples2() {
        assert_eq!(19, find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
        assert_eq!(23, find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
        assert_eq!(23, find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14));
        assert_eq!(29, find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
        assert_eq!(26, find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(7, part1("sample.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(19, part2("sample.txt"));
    }
}
