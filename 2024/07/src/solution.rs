use itertools::Itertools;
use std::fs::read_to_string;

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    let input = read_to_string(input).unwrap();
    let equations = input
        .trim()
        .lines()
        .map(|line| {
            let (sum, inputs) = line.split(": ").collect_tuple().unwrap();
            let inputs = inputs
                .split(" ")
                .map(|i| i.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (sum.parse::<usize>().unwrap(), inputs)
        })
        .collect::<Vec<_>>();
    equations
}

fn is_result(sum: usize, inputs: &[usize], with_concat: bool) -> bool {
    let last_ndx = inputs.len() - 1;
    is_result_rec(sum, inputs, last_ndx, with_concat)
}

fn is_result_rec(sum: usize, inputs: &[usize], last_ndx: usize, with_concat: bool) -> bool {
    let last = inputs[last_ndx];

    if last_ndx == 0 {
        return sum == last;
    }

    let mut found = false;

    if sum >= last {
        found = is_result_rec(sum - last, inputs, last_ndx - 1, with_concat)
    }

    if !found && sum % last == 0 {
        found = is_result_rec(sum / last, inputs, last_ndx - 1, with_concat)
    }

    if with_concat && !found && last_ndx > 0 {
        let d = 10usize.pow(last.ilog10() + 1);
        if sum % d == last {
            found = is_result_rec(sum / d, inputs, last_ndx - 1, with_concat)
        }
    }

    found
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(sum, inputs)| is_result(*sum, inputs, false))
        .map(|(sum, _)| sum)
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|(sum, inputs)| is_result(*sum, inputs, true))
        .map(|(sum, _)| sum)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(8, &[2, 4], true => true)]
    #[test_case(8, &[2, 4], false => true)]
    #[test_case(6, &[2, 4], true => true)]
    #[test_case(6, &[2, 4], false => true)]
    #[test_case(24, &[2, 4], true => true)]
    #[test_case(24, &[2, 4], false => false)]
    #[test_case(248, &[2, 4, 8], true => true)]
    #[test_case(248, &[2, 4, 8], false => false)]
    #[test_case(156, &[15, 6], true => true)]
    #[test_case(7290, &[6, 8, 6, 15], true => true)]
    #[test_case(192, &[17, 8, 14], true => true)]
    fn test_is_result(sum: usize, inputs: &[usize], with_concat: bool) -> bool {
        is_result(sum, inputs, with_concat)
    }

    #[test]
    fn part1_sample() {
        assert_eq!(3749, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(2654749936343, part1("input.txt"));
    }

    #[test]
    fn part2_sample() {
        assert_eq!(11387, part2("sample.txt"));
    }

    #[test]
    fn part2_input() {
        assert_eq!(124060392153684, part2("input.txt"));
    }
}
