#![feature(slice_group_by)]
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn game_from_input(input: &str) -> (String, HashMap<String, String>) {
    let string = read_to_string(input).unwrap();
    let mut lines = string.lines();

    (
        lines.next().unwrap().to_string(),
        lines
            .skip(1)
            .map(|r| {
                r.split(" -> ")
                    .map(|r| r.to_string())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use test_case::test_case;

    use super::*;

    #[test_case("sample1.txt" => is eq(1588) ; "sample1")]
    #[test_case("input.txt" => is eq(3306) ; "input")]
    fn part1(input: &str) -> usize {
        let (template, rules) = game_from_input(input);

        dbg!(&template);
        dbg!(&rules);

        let mut polymer = template;
        for _ in 0..10 {
            polymer = apply(polymer, &rules);
        }

        let (most, least) = count_most_least(polymer);

        most - least
    }

    fn count_most_least(polymer: String) -> (usize, usize) {
        let counts = polymer.chars().fold(HashMap::new(), |mut m, c| {
            *m.entry(c).or_insert(0) += 1;
            m
        });
        (
            *counts.values().max().unwrap(),
            *counts.values().min().unwrap(),
        )
    }

    fn apply(template: String, rules: &HashMap<String, String>) -> String {
        let mut polymer = template[..1].to_string();

        for i in 0..template.len() - 1 {
            let pair = &template[i..=i + 1];
            let last = &pair[1..];
            match rules.get(pair) {
                Some(insert) => {
                    polymer += insert;
                    polymer += last
                }
                None => polymer += last,
            }
        }

        polymer
    }
}
