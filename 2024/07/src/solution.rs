use anyhow::bail;
use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::fs::read_to_string;

fn is_result(sum: usize, inputs: &[usize], ndx: usize) -> bool {
    if ndx == inputs.len() {
        return sum == 0;
    }

    let last = inputs[inputs.len() - ndx - 1];

    let mut found = false;

    if sum >= last {
        found = is_result(sum - last, inputs, ndx + 1)
    }

    if !found && sum % last == 0 {
        found = is_result(sum / last, inputs, ndx + 1)
    }

    found
}

pub fn part1(input: &str) -> usize {
    let input = read_to_string(input).unwrap();
    let mut equations = input
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
        .iter()
        .filter(|(sum, inputs)| is_result(*sum, inputs, 0))
        .map(|(sum, _)| sum)
        .sum()
}

pub fn part2(input: &str) -> usize {
    //todo!()
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_result() {
        let inputs = vec![2, 4];
        //assert_eq!(true, is_result(8, &inputs, 0));
        assert_eq!(true, is_result(6, &inputs, 0));
    }

    #[test]
    fn part1_sample() {
        assert_eq!(3749, part1("sample.txt"));
    }

    #[test]
    fn part1_input() {
        assert_eq!(2654749936343, part1("input.txt"));
    }
}
