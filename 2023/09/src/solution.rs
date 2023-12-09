use std::fs::read_to_string;

fn next(seq: &[i32]) -> i32 {
    let next_seq = seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    if next_seq.iter().all(|n| *n == 0) {
        seq[0]
    } else {
        let next_val = next(&next_seq);
        seq.last().expect("seq is not empty") + next_val
    }
}

fn prev(seq: &[i32]) -> i32 {
    let next_seq = seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    if next_seq.iter().all(|n| *n == 0) {
        seq[0]
    } else {
        let prev_val = prev(&next_seq);
        seq.first().expect("seq is not empty") - prev_val
    }
}

pub fn part1(input: &str) -> i32 {
    let input = read_to_string(input).unwrap();
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|seq| next(&seq))
        .sum()
}

pub fn part2(input: &str) -> i32 {
    let input = read_to_string(input).unwrap();
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|seq| prev(&seq))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_sequence_prev() {
        let seq = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(5, prev(&seq));
    }

    #[test]
    fn another_sequence_prev() {
        let seq = vec![0, 2, 4, 6];
        assert_eq!(-2, prev(&seq));
    }

    #[test]
    fn one_sequence() {
        let seq = vec![3, 3, 3];
        assert_eq!(3, next(&seq));
    }

    #[test]
    fn another_sequence() {
        let seq = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(18, next(&seq));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(114, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(1953784198, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(2, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(957, part2("input.txt"));
    }
}
